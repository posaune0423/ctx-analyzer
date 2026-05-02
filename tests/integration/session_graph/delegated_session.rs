//! Integration test: a Codex rollout whose `session_meta.source.subagent`
//! is set is recovered as a delegated child session
//! (`docs/specs/session-graph.md` §3).

use std::path::PathBuf;

use ctx_analyzer::adapters::codex::CodexAdapter;
use ctx_analyzer::domain::{ContextCategory, ContextSourceKind};
use ctx_analyzer::infra::CharsPer4Estimator;
use ctx_analyzer::ports::AgentAdapter;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/codex/rollout-with-subagent/ctx.jsonl")
}

#[test]
fn subagent_rollout_is_recovered_as_delegated_child() {
    let adapter = CodexAdapter::new();
    let estimator = CharsPer4Estimator;
    let session = adapter.parse_file(&fixture_path(), &estimator).unwrap();

    assert!(session.graph.is_delegated_child);
    assert_eq!(session.graph.subagent_label.as_deref(), Some("review"));
    assert_eq!(session.graph.root_id, session.id);
    assert!(
        session.graph.children.is_empty(),
        "MVP: cross-file linking deferred"
    );

    let delegated_marker_count = session
        .segments
        .iter()
        .filter(|s| {
            s.category == ContextCategory::Delegated
                && s.source_kind == ContextSourceKind::SubagentMarker
        })
        .count();
    assert_eq!(
        delegated_marker_count, 1,
        "exactly one SubagentMarker emitted for a delegated rollout"
    );
}
