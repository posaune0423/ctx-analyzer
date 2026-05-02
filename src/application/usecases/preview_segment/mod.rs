//! Resolve a `ContextSegment` to preview text for the Preview Modal
//! (`docs/development/wireframe/preview-modal.md`).
//!
//! Rollout segments: read the JSONL line at `source_ref.line` and extract
//! the payload text via `adapters::codex::rollout_preview`. Non-rollout
//! workspace text files (no line number) fall back to a clamped file read.

use std::path::Path;

use crate::adapters::codex::rollout_preview;
use crate::application::view_models::context_breakdown::SegmentRow;
use crate::constants::preview::PREVIEW_BODY_MAX_CHARS;
use crate::domain::ContextSegment;
use crate::infra::json;
use crate::utils::text::truncate_chars;

const MAX_PREVIEW_BYTES: usize = 16 * 1024;

#[derive(Debug, Clone)]
pub struct SegmentPreview {
    pub label: String,
    pub source_path: Option<String>,
    pub source_line: Option<usize>,
    pub body: String,
    pub truncated: bool,
}

pub fn for_segment(segment: &ContextSegment) -> SegmentPreview {
    let path = &segment.source_ref.file;
    if path.as_os_str().is_empty() {
        return SegmentPreview {
            label: segment.label.clone(),
            source_path: None,
            source_line: None,
            body: String::new(),
            truncated: false,
        };
    }

    let path_str = path.display().to_string();
    let line = segment.source_ref.line;

    if is_rollout_jsonl(path) {
        if let Some(n) = line {
            if let Ok(Some(raw)) = json::read_jsonl_line(path, n) {
                if let Some(body) = rollout_preview::extract_body_from_line(&raw, segment) {
                    return clamp_body(segment.label.clone(), path_str, Some(n), body);
                }
            }
        }
        let mut body = segment.preview.clone();
        if !body.is_empty() {
            body.push_str("\n\n(could not re-read JSONL line; showing cached preview)");
        }
        return clamp_body(segment.label.clone(), path_str, line, body);
    }

    if line.is_some() {
        return clamp_body(
            segment.label.clone(),
            path_str,
            line,
            segment.preview.clone(),
        );
    }

    let (body, truncated) = read_clamped(path).unwrap_or((String::new(), false));
    SegmentPreview {
        label: segment.label.clone(),
        source_path: Some(path_str),
        source_line: None,
        body,
        truncated,
    }
}

pub fn for_row(row: &SegmentRow, session: &crate::domain::Session) -> SegmentPreview {
    let Some(seg) = session.segments.iter().find(|s| s.id == row.segment_id) else {
        let path = &row.source_ref.file;
        if path.as_os_str().is_empty() {
            return SegmentPreview {
                label: row.label.clone(),
                source_path: None,
                source_line: None,
                body: String::new(),
                truncated: false,
            };
        }
        return SegmentPreview {
            label: row.label.clone(),
            source_path: Some(path.display().to_string()),
            source_line: row.source_ref.line,
            body: String::new(),
            truncated: false,
        };
    };
    for_segment(seg)
}

fn clamp_body(label: String, path: String, line: Option<usize>, body: String) -> SegmentPreview {
    let total_chars = body.chars().count();
    if total_chars <= PREVIEW_BODY_MAX_CHARS {
        return SegmentPreview {
            label,
            source_path: Some(path),
            source_line: line,
            body,
            truncated: false,
        };
    }
    let clipped = truncate_chars(&body, PREVIEW_BODY_MAX_CHARS);
    SegmentPreview {
        label,
        source_path: Some(path),
        source_line: line,
        body: clipped,
        truncated: true,
    }
}

fn is_rollout_jsonl(p: &Path) -> bool {
    matches!(
        p.extension().and_then(|e| e.to_str()),
        Some("jsonl" | "JSONL")
    )
}

fn read_clamped(path: &Path) -> std::io::Result<(String, bool)> {
    let bytes = std::fs::read(path)?;
    let truncated = bytes.len() > MAX_PREVIEW_BYTES;
    let slice = &bytes[..bytes.len().min(MAX_PREVIEW_BYTES)];
    let text = String::from_utf8_lossy(slice).into_owned();
    Ok((text, truncated))
}
