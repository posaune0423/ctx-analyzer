//! Helpers that classify free-form Codex text blocks into specific
//! `ContextSourceKind`s using the markers observed in real rollouts.

use crate::domain::ContextSourceKind;

const PROJECT_DOC_PREFIX: &str = "# AGENTS.md instructions for ";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeveloperBlock {
    pub kind: ContextSourceKind,
    pub label: String,
    pub body: String,
}

const TAGS: &[(&str, &str, ContextSourceKind, &str)] = &[
    (
        "<permissions instructions>",
        "</permissions instructions>",
        ContextSourceKind::PermissionsInstructions,
        "Permissions Instructions",
    ),
    (
        "<apps_instructions>",
        "</apps_instructions>",
        ContextSourceKind::AppsInstructions,
        "Apps Instructions",
    ),
    (
        "<skills_instructions>",
        "</skills_instructions>",
        ContextSourceKind::SkillsInstructions,
        "Skills Instructions",
    ),
    (
        "<plugins_instructions>",
        "</plugins_instructions>",
        ContextSourceKind::PluginsInstructions,
        "Plugins Instructions",
    ),
];

/// Split the first `developer` message into its tagged sub-blocks plus a
/// fallback `BaseInstructions` block for any leftover text.
pub fn split_developer_blocks(text: &str) -> Vec<DeveloperBlock> {
    let mut blocks: Vec<(usize, DeveloperBlock)> = Vec::new();
    let mut consumed_ranges: Vec<(usize, usize)> = Vec::new();

    for (open, close, kind, label) in TAGS {
        let mut search_from = 0;
        while let Some(rel_open) = text[search_from..].find(open) {
            let start = search_from + rel_open;
            let after_open = start + open.len();
            let Some(rel_close) = text[after_open..].find(close) else {
                break;
            };
            let close_start = after_open + rel_close;
            let close_end = close_start + close.len();
            let body = text[after_open..close_start].trim().to_string();
            blocks.push((
                start,
                DeveloperBlock {
                    kind: *kind,
                    label: (*label).to_string(),
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

pub fn looks_like_project_doc(text: &str) -> bool {
    text.trim_start().starts_with(PROJECT_DOC_PREFIX)
}
