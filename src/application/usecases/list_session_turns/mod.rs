//! Turn List summaries (`docs/development/wireframe/turn-list.md`).

use chrono::{DateTime, Utc};

use crate::constants::preview::LABEL_TRUNCATE_CHARS;
use crate::domain::{ContextSourceKind, Session};
use crate::utils::text::truncate_chars;

/// Char budget for the "User Prompt" column in the Turn List wireframe.
pub const TURN_PROMPT_PREVIEW_CHARS: usize = 60;

#[derive(Debug, Clone)]
pub struct TurnSummary {
    pub index: usize,
    pub turn_id: String,
    pub prompt_preview: Option<String>,
    pub total_tokens: u64,
    pub has_subagent: bool,
    pub started_at: Option<DateTime<Utc>>,
}

pub fn summarize(session: &Session) -> Vec<TurnSummary> {
    session
        .turns
        .iter()
        .map(|turn| {
            let mut total_tokens = 0u64;
            let mut has_subagent = false;
            let mut prompt_preview: Option<String> = None;

            for seg_id in &turn.segment_ids {
                let Some(seg) = session.segments.iter().find(|s| s.id == *seg_id) else {
                    continue;
                };
                total_tokens = total_tokens.saturating_add(seg.tokens.tokens);
                if matches!(seg.source_kind, ContextSourceKind::SubagentMarker) {
                    has_subagent = true;
                }
                if prompt_preview.is_none()
                    && matches!(seg.source_kind, ContextSourceKind::UserPrompt)
                {
                    prompt_preview = Some(truncate_chars(
                        &collapse_whitespace(&seg.preview),
                        TURN_PROMPT_PREVIEW_CHARS,
                    ));
                }
            }

            TurnSummary {
                index: turn.index,
                turn_id: turn.id.clone(),
                prompt_preview,
                total_tokens,
                has_subagent,
                started_at: turn.started_at,
            }
        })
        .collect()
}

fn collapse_whitespace(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last_was_space = false;
    for ch in s.chars() {
        if ch.is_whitespace() {
            if !last_was_space && !out.is_empty() {
                out.push(' ');
            }
            last_was_space = true;
        } else {
            out.push(ch);
            last_was_space = false;
        }
    }
    while out.ends_with(' ') {
        out.pop();
    }
    out
}

const _: () = assert!(TURN_PROMPT_PREVIEW_CHARS >= LABEL_TRUNCATE_CHARS);
