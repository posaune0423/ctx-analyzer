use std::path::{Path, PathBuf};

use crate::constants::preview::PREVIEW_LIMIT_CHARS;
use crate::domain::session::ParseWarning;
use crate::domain::{
    AgentKind, Confidence, ContextCategory, ContextSegment, ContextSourceKind, Session,
    SessionGraph, SessionSource, SourceRef, TokenEstimate, Turn,
};
use crate::ports::TokenEstimator;
use crate::utils::text::truncate_chars;

use super::classifiers as classifier;
use super::parsers::ParsedLine;
use super::raw::{
    BaseInstructions, ContentPart, Envelope, EventMsg, Message, MetaSource, Payload, Reasoning,
    ResponseItem, SessionMeta, TokenInfo, TokenUsage, TurnContext,
};

pub fn map_to_session(
    path: &Path,
    parsed: Vec<ParsedLine>,
    warnings: Vec<ParseWarning>,
    estimator: &dyn TokenEstimator,
) -> Session {
    let source_path: PathBuf = path.to_path_buf();
    let mut state = MapperState::new(source_path.clone());

    for ParsedLine { line, envelope } in parsed {
        let Envelope {
            timestamp: _,
            payload,
        } = envelope;
        match payload {
            Payload::SessionMeta(meta) => state.handle_session_meta(meta, line, estimator),
            Payload::TurnContext(tc) => state.handle_turn_context(tc, line, estimator),
            Payload::EventMsg(ev) => state.handle_event_msg(ev, line, estimator),
            Payload::ResponseItem(item) => state.handle_response_item(item, line, estimator),
            Payload::Other => {}
        }
    }

    state.finalize(warnings)
}

struct MapperState {
    source_path: PathBuf,
    session_id: Option<String>,
    started_at: Option<chrono::DateTime<chrono::Utc>>,
    cwd: Option<PathBuf>,
    cli_version: Option<String>,
    model_provider: Option<String>,
    source: SessionSource,
    is_delegated_child: bool,
    subagent_label: Option<String>,
    segments: Vec<ContextSegment>,
    turns: Vec<Turn>,
    current_turn_id: Option<String>,
    seen_developer_block: bool,
    seen_project_doc: bool,
    final_token_usage: Option<TokenUsage>,
}

impl MapperState {
    fn new(source_path: PathBuf) -> Self {
        Self {
            source_path,
            session_id: None,
            started_at: None,
            cwd: None,
            cli_version: None,
            model_provider: None,
            source: SessionSource::default(),
            is_delegated_child: false,
            subagent_label: None,
            segments: Vec::new(),
            turns: Vec::new(),
            current_turn_id: None,
            seen_developer_block: false,
            seen_project_doc: false,
            final_token_usage: None,
        }
    }

    fn handle_session_meta(
        &mut self,
        meta: SessionMeta,
        line: usize,
        estimator: &dyn TokenEstimator,
    ) {
        self.session_id = Some(meta.id);
        self.started_at = meta.timestamp;
        self.cwd = meta.cwd.map(PathBuf::from);
        self.cli_version = meta.cli_version;
        self.model_provider = meta.model_provider;

        if let Some(MetaSource {
            subagent: Some(name),
        }) = meta.source
        {
            self.is_delegated_child = true;
            self.subagent_label = Some(name.clone());
            self.source.subagent = Some(name.clone());
            // Emit a delegated marker segment.
            let label = format!("Subagent: {name}");
            let preview = format!("This session was launched as a delegated subagent ({name}).");
            let est = estimator.estimate(&preview);
            self.push_segment(
                ContextCategory::Delegated,
                ContextSourceKind::SubagentMarker,
                label,
                preview,
                line,
                est,
                Confidence::Observed,
            );
        }

        if let Some(BaseInstructions { text: Some(text) }) = meta.base_instructions {
            let est = estimator.estimate(&text);
            self.push_segment(
                ContextCategory::System,
                ContextSourceKind::BaseInstructions,
                "Base Instructions".to_string(),
                truncate_preview(&text),
                line,
                est,
                Confidence::Estimated,
            );
            // Replace preview in last segment with full text length tracking already happens.
            if let Some(seg) = self.segments.last_mut() {
                seg.full_len_chars = text.chars().count();
            }
        }
    }

    fn handle_turn_context(
        &mut self,
        tc: TurnContext,
        line: usize,
        estimator: &dyn TokenEstimator,
    ) {
        if let Some(id) = tc.turn_id.clone() {
            self.ensure_turn(id, tc.model.clone(), tc.user_instructions.clone());
        }
        if let Some(text) = tc.user_instructions {
            let est = estimator.estimate(&text);
            let chars = text.chars().count();
            let preview = truncate_preview(&text);
            self.push_segment_full(
                ContextCategory::Configuration,
                ContextSourceKind::UserInstructions,
                "User Instructions (turn)".to_string(),
                preview,
                chars,
                line,
                est,
                Confidence::Estimated,
            );
        }
    }

    fn handle_event_msg(&mut self, ev: EventMsg, line: usize, estimator: &dyn TokenEstimator) {
        match ev {
            EventMsg::TaskStarted(t) => {
                if let Some(id) = t.turn_id {
                    self.ensure_turn(id.clone(), None, None);
                    self.current_turn_id = Some(id);
                    if let Some(turn) = self.turns.last_mut() {
                        turn.started_at = chrono::Utc::now().into();
                        // Reset to None; we don't have a precise timestamp from the event itself.
                        turn.started_at = None;
                    }
                }
            }
            EventMsg::TaskComplete(t) => {
                if let Some(id) = t.turn_id {
                    if let Some(turn) = self.turns.iter_mut().find(|x| x.id == id) {
                        turn.completed_at = None;
                    }
                    if self.current_turn_id.as_deref() == Some(id.as_str()) {
                        // keep current_turn_id so trailing events still attach to it
                    }
                }
            }
            EventMsg::TokenCount(tc) => {
                if let Some(TokenInfo {
                    total_token_usage: Some(total),
                    last_token_usage,
                }) = tc.info
                {
                    self.final_token_usage = Some(total);
                    if let Some(last) = last_token_usage {
                        if let Some(turn) = self.current_turn_mut() {
                            turn.last_token_usage = Some(usage_to_estimate(last));
                        }
                    }
                }
            }
            EventMsg::UserMessage(um) => {
                if let Some(text) = um.message {
                    if !text.trim().is_empty() {
                        let est = estimator.estimate(&text);
                        let chars = text.chars().count();
                        let preview = truncate_preview(&text);
                        self.push_segment_full(
                            ContextCategory::Runtime,
                            ContextSourceKind::UserPrompt,
                            "User Prompt".to_string(),
                            preview,
                            chars,
                            line,
                            est,
                            Confidence::Estimated,
                        );
                    }
                }
            }
            EventMsg::AgentMessage(am) => {
                if let Some(text) = am.message {
                    if !text.trim().is_empty() {
                        let est = estimator.estimate(&text);
                        let chars = text.chars().count();
                        let preview = truncate_preview(&text);
                        self.push_segment_full(
                            ContextCategory::Runtime,
                            ContextSourceKind::AssistantMessage,
                            "Assistant Message".to_string(),
                            preview,
                            chars,
                            line,
                            est,
                            Confidence::Estimated,
                        );
                    }
                }
            }
            EventMsg::Other => {}
        }
    }

    fn handle_response_item(
        &mut self,
        item: ResponseItem,
        line: usize,
        estimator: &dyn TokenEstimator,
    ) {
        match item {
            ResponseItem::Message(msg) => self.handle_message(msg, line, estimator),
            ResponseItem::Reasoning(r) => self.handle_reasoning(r, line, estimator),
            ResponseItem::FunctionCall(fc) => {
                let name = fc.name.unwrap_or_else(|| "<unknown>".to_string());
                let args = fc.arguments.unwrap_or_default();
                let est = estimator.estimate(&args);
                let chars = args.chars().count();
                let preview = truncate_preview(&args);
                self.push_segment_full(
                    ContextCategory::Runtime,
                    ContextSourceKind::FunctionCall,
                    format!("function_call: {name}"),
                    preview,
                    chars,
                    line,
                    est,
                    Confidence::Estimated,
                );
            }
            ResponseItem::FunctionCallOutput(out) => {
                let body = out.output.unwrap_or_default();
                let est = estimator.estimate(&body);
                let chars = body.chars().count();
                let preview = truncate_preview(&body);
                self.push_segment_full(
                    ContextCategory::Runtime,
                    ContextSourceKind::FunctionCallOutput,
                    "function_call_output".to_string(),
                    preview,
                    chars,
                    line,
                    est,
                    Confidence::Estimated,
                );
            }
            ResponseItem::Other => {}
        }
    }

    fn handle_message(&mut self, msg: Message, line: usize, estimator: &dyn TokenEstimator) {
        let combined = combine_text(&msg.content);
        if combined.trim().is_empty() {
            return;
        }
        match msg.role.as_str() {
            "developer" => {
                if !self.seen_developer_block {
                    self.seen_developer_block = true;
                    let blocks = classifier::split_developer_blocks(&combined);
                    if blocks.is_empty() {
                        let est = estimator.estimate(&combined);
                        let chars = combined.chars().count();
                        self.push_segment_full(
                            ContextCategory::Configuration,
                            ContextSourceKind::BaseInstructions,
                            "Developer Message".to_string(),
                            truncate_preview(&combined),
                            chars,
                            line,
                            est,
                            Confidence::Estimated,
                        );
                    } else {
                        for block in blocks {
                            let est = estimator.estimate(&block.body);
                            let chars = block.body.chars().count();
                            self.push_segment_full(
                                ContextCategory::Configuration,
                                block.kind,
                                block.label,
                                truncate_preview(&block.body),
                                chars,
                                line,
                                est,
                                Confidence::Estimated,
                            );
                        }
                    }
                } else {
                    let est = estimator.estimate(&combined);
                    let chars = combined.chars().count();
                    self.push_segment_full(
                        ContextCategory::Runtime,
                        ContextSourceKind::AssistantMessage,
                        "Developer Message".to_string(),
                        truncate_preview(&combined),
                        chars,
                        line,
                        est,
                        Confidence::Estimated,
                    );
                }
            }
            "user" => {
                let is_project_doc =
                    !self.seen_project_doc && classifier::looks_like_project_doc(&combined);
                if is_project_doc {
                    self.seen_project_doc = true;
                    let est = estimator.estimate(&combined);
                    let chars = combined.chars().count();
                    self.push_segment_full(
                        ContextCategory::Configuration,
                        ContextSourceKind::ProjectInstructions,
                        "Project Instructions (AGENTS.md)".to_string(),
                        truncate_preview(&combined),
                        chars,
                        line,
                        est,
                        Confidence::Estimated,
                    );
                } else {
                    let est = estimator.estimate(&combined);
                    let chars = combined.chars().count();
                    self.push_segment_full(
                        ContextCategory::Runtime,
                        ContextSourceKind::UserPrompt,
                        "User Prompt".to_string(),
                        truncate_preview(&combined),
                        chars,
                        line,
                        est,
                        Confidence::Estimated,
                    );
                }
            }
            "assistant" => {
                let est = estimator.estimate(&combined);
                let chars = combined.chars().count();
                self.push_segment_full(
                    ContextCategory::Runtime,
                    ContextSourceKind::AssistantMessage,
                    "Assistant Message".to_string(),
                    truncate_preview(&combined),
                    chars,
                    line,
                    est,
                    Confidence::Estimated,
                );
            }
            _ => {}
        }
    }

    fn handle_reasoning(&mut self, r: Reasoning, line: usize, _estimator: &dyn TokenEstimator) {
        if r.encrypted_content.is_some() {
            // Encrypted reasoning blob: visible-as-unknown (PRD §7.2 / §13.1.17).
            self.push_segment_full(
                ContextCategory::Unknown,
                ContextSourceKind::EncryptedReasoning,
                "Encrypted Reasoning".to_string(),
                "<encrypted>".to_string(),
                0,
                line,
                TokenEstimate::unknown(),
                Confidence::Unknown,
            );
        }
    }

    fn ensure_turn(
        &mut self,
        id: String,
        model: Option<String>,
        user_instructions: Option<String>,
    ) {
        if self.turns.iter().any(|t| t.id == id) {
            if let Some(turn) = self.turns.iter_mut().find(|t| t.id == id) {
                if turn.model.is_none() {
                    turn.model = model;
                }
                if turn.user_instructions.is_none() {
                    turn.user_instructions = user_instructions;
                }
            }
            self.current_turn_id = Some(id);
            return;
        }
        let index = self.turns.len() + 1;
        self.turns.push(Turn {
            id: id.clone(),
            index,
            started_at: None,
            completed_at: None,
            model,
            user_instructions,
            segment_ids: Vec::new(),
            last_token_usage: None,
        });
        self.current_turn_id = Some(id);
    }

    fn current_turn_mut(&mut self) -> Option<&mut Turn> {
        let id = self.current_turn_id.clone()?;
        self.turns.iter_mut().find(|t| t.id == id)
    }

    #[allow(clippy::too_many_arguments)]
    fn push_segment(
        &mut self,
        category: ContextCategory,
        source_kind: ContextSourceKind,
        label: String,
        preview: String,
        line: usize,
        tokens: TokenEstimate,
        confidence: Confidence,
    ) {
        let chars = preview.chars().count();
        self.push_segment_full(
            category,
            source_kind,
            label,
            preview,
            chars,
            line,
            tokens,
            confidence,
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn push_segment_full(
        &mut self,
        category: ContextCategory,
        source_kind: ContextSourceKind,
        label: String,
        preview: String,
        full_len_chars: usize,
        line: usize,
        tokens: TokenEstimate,
        confidence: Confidence,
    ) {
        let id = self.segments.len();
        let turn_id = self.current_turn_id.clone();
        let segment = ContextSegment {
            id,
            turn_id: turn_id.clone(),
            category,
            source_kind,
            label,
            preview,
            full_len_chars,
            source_ref: SourceRef {
                file: self.source_path.clone(),
                line: Some(line),
            },
            tokens,
            confidence,
        };
        self.segments.push(segment);
        if let Some(tid) = turn_id {
            if let Some(turn) = self.turns.iter_mut().find(|t| t.id == tid) {
                turn.segment_ids.push(id);
            }
        }
    }

    fn finalize(self, warnings: Vec<ParseWarning>) -> Session {
        let session_id = self.session_id.unwrap_or_else(|| "<unknown>".to_string());
        let graph = SessionGraph {
            root_id: session_id.clone(),
            is_delegated_child: self.is_delegated_child,
            subagent_label: self.subagent_label,
            children: Vec::new(),
        };
        let session_totals = self.final_token_usage.map(usage_to_estimate);
        Session {
            id: session_id,
            agent: AgentKind::Codex,
            started_at: self.started_at,
            cwd: self.cwd,
            source_path: self.source_path,
            source: self.source,
            model_provider: self.model_provider,
            cli_version: self.cli_version,
            turns: self.turns,
            segments: self.segments,
            session_totals,
            graph,
            warnings,
        }
    }
}

fn combine_text(parts: &[ContentPart]) -> String {
    let mut out = String::new();
    for p in parts {
        if let Some(t) = &p.text {
            out.push_str(t);
        }
    }
    out
}

fn truncate_preview(text: &str) -> String {
    truncate_chars(text.trim(), PREVIEW_LIMIT_CHARS)
}

fn usage_to_estimate(u: TokenUsage) -> TokenEstimate {
    let total = u.total_tokens.unwrap_or_else(|| {
        u.input_tokens.unwrap_or(0)
            + u.output_tokens.unwrap_or(0)
            + u.reasoning_output_tokens.unwrap_or(0)
    });
    TokenEstimate {
        tokens: total,
        input: u.input_tokens,
        cached_input: u.cached_input_tokens,
        output: u.output_tokens,
        reasoning_output: u.reasoning_output_tokens,
        confidence: Confidence::Observed,
    }
}
