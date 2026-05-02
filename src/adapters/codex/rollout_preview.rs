//! Extract human-readable text bodies from a single rollout JSONL line for
//! the Preview Modal. Codex-specific: lives under `adapters/codex/`.

use crate::adapters::codex::classifiers::{self, split_developer_blocks};
use crate::adapters::codex::raw::{
    ContentPart, Envelope, EventMsg, Message, Payload, ResponseItem, RolloutRecord,
};
use crate::domain::{ContextSegment, ContextSourceKind};

/// Best-effort extraction of the full text for `segment` from one JSONL
/// `line`. Returns `None` when the line does not parse or the payload does
/// not match the segment classification.
pub fn extract_body_from_line(line: &str, seg: &ContextSegment) -> Option<String> {
    let env: Envelope = serde_json::from_str::<RolloutRecord>(line)
        .ok()?
        .into_envelope();
    extract_from_envelope(&env, seg)
}

fn extract_from_envelope(env: &Envelope, seg: &ContextSegment) -> Option<String> {
    match &env.payload {
        Payload::SessionMeta(m) => {
            if matches!(seg.source_kind, ContextSourceKind::BaseInstructions) {
                return m.base_instructions.as_ref()?.text.clone();
            }
            None
        }
        Payload::TurnContext(tc) => {
            if matches!(seg.source_kind, ContextSourceKind::UserInstructions) {
                return tc.user_instructions.clone();
            }
            None
        }
        Payload::EventMsg(ev) => match ev {
            EventMsg::UserMessage(um)
                if matches!(seg.source_kind, ContextSourceKind::UserPrompt) =>
            {
                um.message.clone()
            }
            EventMsg::AgentMessage(am)
                if matches!(seg.source_kind, ContextSourceKind::AssistantMessage) =>
            {
                am.message.clone()
            }
            _ => None,
        },
        Payload::ResponseItem(ri) => extract_response_item(ri, seg),
        Payload::Compacted(_) => None,
        Payload::Other => None,
    }
}

fn extract_response_item(ri: &ResponseItem, seg: &ContextSegment) -> Option<String> {
    match ri {
        ResponseItem::Message(msg) => extract_message_for_segment(msg, seg),
        ResponseItem::FunctionCall(fc)
            if matches!(seg.source_kind, ContextSourceKind::FunctionCall) =>
        {
            fc.arguments.clone()
        }
        ResponseItem::FunctionCallOutput(out)
            if matches!(seg.source_kind, ContextSourceKind::FunctionCallOutput) =>
        {
            out.output.clone()
        }
        ResponseItem::Reasoning(r)
            if matches!(seg.source_kind, ContextSourceKind::EncryptedReasoning) =>
        {
            if r.encrypted_content.is_some() {
                Some("(encrypted reasoning · not decryptable locally)".to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn combine_parts(parts: &[ContentPart]) -> String {
    let mut out = String::new();
    for p in parts {
        if let Some(t) = &p.text {
            out.push_str(t);
        }
    }
    out
}

fn extract_message_for_segment(msg: &Message, seg: &ContextSegment) -> Option<String> {
    let combined = combine_parts(&msg.content);
    if combined.trim().is_empty() {
        return None;
    }
    match msg.role.as_str() {
        // Subsequent developer-role blobs after the first are stored as
        // `AssistantMessage` segments (`mappers/mod.rs`).
        "developer" if matches!(seg.source_kind, ContextSourceKind::AssistantMessage) => {
            Some(combined)
        }
        "developer" => extract_developer_message(&combined, seg),
        "user" => extract_user_message(&combined, seg),
        "assistant" => {
            if matches!(seg.source_kind, ContextSourceKind::AssistantMessage) {
                Some(combined)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn extract_developer_message(combined: &str, seg: &ContextSegment) -> Option<String> {
    let blocks = split_developer_blocks(combined);
    if blocks.is_empty() {
        if matches!(seg.source_kind, ContextSourceKind::BaseInstructions)
            && seg.label == "Developer Message"
        {
            return Some(combined.to_string());
        }
        return None;
    }
    for b in &blocks {
        if b.kind == seg.source_kind {
            return Some(b.body.clone());
        }
    }
    for b in &blocks {
        if b.label == seg.label {
            return Some(b.body.clone());
        }
    }
    None
}

fn extract_user_message(combined: &str, seg: &ContextSegment) -> Option<String> {
    if matches!(seg.source_kind, ContextSourceKind::ProjectInstructions) {
        if classifiers::looks_like_project_doc(combined) {
            return Some(combined.to_string());
        }
        return None;
    }
    if matches!(seg.source_kind, ContextSourceKind::UserPrompt) {
        if classifiers::looks_like_project_doc(combined) {
            return None;
        }
        return Some(combined.to_string());
    }
    None
}
