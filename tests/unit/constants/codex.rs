//! Centralised Codex-specific filesystem constants. Keeps the adapter,
//! discovery code, and docs/specs/codex.md in sync from one source.

use ctx_analyzer::constants::codex::{
    CODEX_DEFAULT_HOME_DIR, CODEX_HOME_ENV, CODEX_ROLLOUT_FILE_PREFIX, CODEX_SESSIONS_SUBDIR,
    CODEX_SESSION_INDEX_FILE, CODEX_STATE_DB_FILE, DEVELOPER_BLOCK_TAGS, PROJECT_DOC_PREFIX,
};

#[test]
fn codex_paths_match_spec() {
    assert_eq!(CODEX_DEFAULT_HOME_DIR, ".codex");
    assert_eq!(CODEX_HOME_ENV, "CODEX_HOME");
    assert_eq!(CODEX_SESSIONS_SUBDIR, "sessions");
    assert_eq!(CODEX_ROLLOUT_FILE_PREFIX, "rollout-");
    assert_eq!(CODEX_SESSION_INDEX_FILE, "session_index.jsonl");
    assert_eq!(CODEX_STATE_DB_FILE, "state.db");
}

#[test]
fn project_doc_prefix_matches_observed_marker() {
    assert_eq!(PROJECT_DOC_PREFIX, "# AGENTS.md instructions for ");
}

#[test]
fn developer_block_tags_cover_four_observed_kinds() {
    let names: Vec<&str> = DEVELOPER_BLOCK_TAGS.iter().map(|t| t.label).collect();
    assert!(names.contains(&"Permissions Instructions"));
    assert!(names.contains(&"Apps Instructions"));
    assert!(names.contains(&"Skills Instructions"));
    assert!(names.contains(&"Plugins Instructions"));
    assert_eq!(DEVELOPER_BLOCK_TAGS.len(), 4);
}
