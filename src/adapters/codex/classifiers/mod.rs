//! Helpers that classify free-form Codex text blocks into specific
//! `ContextSourceKind`s using the markers observed in real rollouts.
//!
//! Tag table and project-doc prefix live in
//! `crate::constants::codex` so adding a new inline marker is a
//! one-place edit.

use crate::constants::codex::{DEVELOPER_BLOCK_TAGS, PROJECT_DOC_PREFIX};
use crate::domain::ContextSourceKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeveloperBlock {
    pub kind: ContextSourceKind,
    pub label: String,
    pub body: String,
}

/// Split the first `developer` message into its tagged sub-blocks plus a
/// fallback `BaseInstructions` block for any leftover text.
pub fn split_developer_blocks(text: &str) -> Vec<DeveloperBlock> {
    let mut blocks: Vec<(usize, DeveloperBlock)> = Vec::new();
    let mut consumed_ranges: Vec<(usize, usize)> = Vec::new();

    for tag in DEVELOPER_BLOCK_TAGS {
        let mut search_from = 0;
        while let Some(rel_open) = text[search_from..].find(tag.open) {
            let start = search_from + rel_open;
            let after_open = start + tag.open.len();
            let Some(rel_close) = text[after_open..].find(tag.close) else {
                break;
            };
            let close_start = after_open + rel_close;
            let close_end = close_start + tag.close.len();
            let body = text[after_open..close_start].trim().to_string();
            blocks.push((
                start,
                DeveloperBlock {
                    kind: tag.kind,
                    label: tag.label.to_string(),
                    body,
                },
            ));
            consumed_ranges.push((start, close_end));
            search_from = close_end;
        }
    }

    let leftover = strip_consumed(text, &consumed_ranges);
    let leftover_trim = leftover.trim();
    if !leftover_trim.is_empty() {
        blocks.push((
            usize::MAX,
            DeveloperBlock {
                kind: ContextSourceKind::BaseInstructions,
                label: "Developer Base Instructions".to_string(),
                body: leftover_trim.to_string(),
            },
        ));
    }

    blocks.sort_by_key(|(pos, _)| *pos);
    blocks.into_iter().map(|(_, b)| b).collect()
}

fn strip_consumed(text: &str, ranges: &[(usize, usize)]) -> String {
    if ranges.is_empty() {
        return text.to_string();
    }
    let mut sorted = ranges.to_vec();
    sorted.sort_by_key(|r| r.0);
    let mut out = String::with_capacity(text.len());
    let mut cursor = 0;
    for (start, end) in sorted {
        if start > cursor {
            out.push_str(&text[cursor..start]);
        }
        cursor = end.max(cursor);
    }
    if cursor < text.len() {
        out.push_str(&text[cursor..]);
    }
    out
}

/// True when the text begins with the AGENTS.md project-doc marker
/// emitted by Codex as the first user-role response item.
pub fn looks_like_project_doc(text: &str) -> bool {
    text.trim_start().starts_with(PROJECT_DOC_PREFIX)
}

/// True when the text is the synthetic environment metadata block injected by
/// Codex at the head of some rollouts.
pub fn looks_like_environment_metadata(text: &str) -> bool {
    let trimmed = text.trim_start();
    trimmed.starts_with("<environment_context>")
        || trimmed.starts_with("<cwd>")
        || trimmed.starts_with("<shell>")
}
