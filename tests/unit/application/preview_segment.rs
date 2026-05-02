//! Preview Modal data shape (UI.md §18). Resolves a `SegmentRow` to a
//! `SegmentPreview` (label / source path / body / truncation flag).

use std::path::PathBuf;

use ctx_analyzer::application::usecases::preview_segment;
use ctx_analyzer::application::view_models::context_breakdown::{SegmentRow, Severity};
use ctx_analyzer::domain::session_graph::SessionGraph;
use ctx_analyzer::domain::{AgentKind, Confidence, Session, SourceRef, TokenEstimate};

/// Row id not present in [`minimal_session`] — exercises the fallback path
/// (label/source from row, empty body).
const ORPHAN_SEGMENT_ID: usize = 9_999;

fn minimal_session() -> Session {
    Session {
        id: "test".into(),
        agent: AgentKind::Codex,
        started_at: None,
        cwd: None,
        source_path: PathBuf::from("/tmp/rollout-abc.jsonl"),
        source: Default::default(),
        model_provider: None,
        cli_version: None,
        turns: vec![],
        segments: vec![],
        session_totals: None,
        graph: SessionGraph::root("test"),
        warnings: vec![],
    }
}

fn row_with(file: &str, line: Option<usize>) -> SegmentRow {
    SegmentRow {
        segment_id: ORPHAN_SEGMENT_ID,
        label: "Permissions Instructions".into(),
        tokens: TokenEstimate::estimated(3),
        severity: Severity::Low,
        confidence: Confidence::Estimated,
        source_ref: SourceRef {
            file: PathBuf::from(file),
            line,
        },
        turn_id: Some("turn-1".into()),
    }
}

#[test]
fn jsonl_rollout_ref_returns_empty_body_when_segment_missing_from_session() {
    // No matching `ContextSegment`: preview body stays empty (path still shown).
    let row = row_with("/tmp/rollout-abc.jsonl", Some(42));
    let session = minimal_session();
    let p = preview_segment::for_row(&row, &session);
    assert_eq!(p.label, "Permissions Instructions");
    assert_eq!(p.source_path.as_deref(), Some("/tmp/rollout-abc.jsonl"));
    assert!(p.body.is_empty());
    assert!(!p.truncated);
}

#[test]
fn empty_source_path_returns_no_path() {
    let row = row_with("", None);
    let session = minimal_session();
    let p = preview_segment::for_row(&row, &session);
    assert!(p.source_path.is_none());
    assert!(p.body.is_empty());
}

#[test]
fn missing_text_file_falls_back_to_empty_body() {
    // Non-existent path: function should not panic; body stays empty.
    let row = row_with("/tmp/__definitely_does_not_exist_ctx_analyzer.txt", None);
    let session = minimal_session();
    let p = preview_segment::for_row(&row, &session);
    assert_eq!(
        p.source_path.as_deref(),
        Some("/tmp/__definitely_does_not_exist_ctx_analyzer.txt")
    );
    assert!(p.body.is_empty());
}
