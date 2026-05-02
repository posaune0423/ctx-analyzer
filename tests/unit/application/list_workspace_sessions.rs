//! Session-Selector view-model: enumerate available rollouts with the
//! per-row metadata UI.md §16 needs (updated time / agent / cwd / turn
//! count / child count).

use ctx_analyzer::application::usecases::list_workspace_sessions::{list, SessionListItem};
use ctx_analyzer::domain::AgentKind;

#[test]
fn list_returns_a_vec_no_panic() {
    // We only assert the call shape — actual contents depend on the
    // host's `~/.codex` and we don't depend on real user state in unit
    // tests (`docs/ARCHITECTURE.md` §13 Test Policy).
    let result = list();
    assert!(result.is_ok(), "list() should not error: {result:?}");
}

#[test]
fn session_list_item_carries_codex_agent_label() {
    // Construct a row directly to exercise the public type's invariants.
    let item = SessionListItem {
        agent: AgentKind::Codex,
        path: std::path::PathBuf::from("/tmp/rollout-abc.jsonl"),
        modified_at: None,
        created_at: None,
        cwd: None,
        conversation: None,
        short_id: "abc".into(),
    };
    assert_eq!(item.agent, AgentKind::Codex);
    assert_eq!(item.short_id, "abc");
}
