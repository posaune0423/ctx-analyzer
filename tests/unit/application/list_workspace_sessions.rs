//! Session list: rollouts scoped to the inspect project (git root + directory tree).

use std::path::PathBuf;
use std::sync::Mutex;

use ctx_analyzer::application::usecases::list_workspace_sessions::{
    list_for_launch_dir, SessionListItem,
};
use ctx_analyzer::constants::codex::CODEX_HOME_ENV;
use ctx_analyzer::domain::AgentKind;

/// `list_for_launch_dir` sets `CODEX_HOME`; serialize those tests.
static CODEX_HOME_TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn list_for_launch_dir_returns_ok_without_codex_home() {
    let _lock = CODEX_HOME_TEST_LOCK.lock().unwrap();
    let prev = std::env::var_os(CODEX_HOME_ENV);
    std::env::remove_var(CODEX_HOME_ENV);
    let dir = std::env::temp_dir();
    let result = list_for_launch_dir(&dir);
    match prev {
        Some(v) => std::env::set_var(CODEX_HOME_ENV, v),
        None => std::env::remove_var(CODEX_HOME_ENV),
    }
    assert!(result.is_ok(), "{result:?}");
}

#[test]
fn list_for_launch_dir_finds_rollout_matching_cwd_and_first_prompt() {
    let _lock = CODEX_HOME_TEST_LOCK.lock().unwrap();
    let base = std::env::temp_dir().join(format!("ctx-analyzer-codex-home-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let sessions_dir = base.join("sessions/2026/05/02");
    std::fs::create_dir_all(&sessions_dir).unwrap();
    let rollout = sessions_dir.join("rollout-listtest.jsonl");

    let launch = std::env::temp_dir().join(format!("ctx-analyzer-launch-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&launch);
    std::fs::create_dir_all(&launch).unwrap();

    let cwd = launch.to_str().unwrap().replace('\\', "/");
    let line1 = serde_json::json!({
        "type": "session_meta",
        "payload": {
            "id": "unit-list-id",
            "cwd": cwd
        }
    });
    let line2 = serde_json::json!({
        "type": "response_item",
        "payload": {
            "type": "message",
            "role": "user",
            "content": [{ "type": "text", "text": "First user ask" }]
        }
    });
    std::fs::write(&rollout, format!("{line1}\n{line2}\n")).unwrap();

    let prev = std::env::var_os(CODEX_HOME_ENV);
    std::env::set_var(CODEX_HOME_ENV, &base);
    let list = list_for_launch_dir(&launch).unwrap();
    match prev {
        Some(v) => std::env::set_var(CODEX_HOME_ENV, v),
        None => std::env::remove_var(CODEX_HOME_ENV),
    }
    let _ = std::fs::remove_dir_all(&base);
    let _ = std::fs::remove_dir_all(&launch);

    assert_eq!(list.len(), 1);
    assert_eq!(list[0].first_prompt.as_deref(), Some("First user ask"));
    assert_eq!(list[0].path, rollout);
}

#[test]
fn list_includes_rollout_when_meta_cwd_is_parent_of_launch_dir() {
    let _lock = CODEX_HOME_TEST_LOCK.lock().unwrap();
    let base = std::env::temp_dir().join(format!("ctx-analyzer-tree-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(base.join("subdir")).unwrap();
    std::fs::create_dir(base.join(".git")).unwrap(); // ADDED .git
    let launch = base.join("subdir");

    let codex = std::env::temp_dir().join(format!("ctx-codex-tree-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&codex);
    let sessions_dir = codex.join("sessions/2026/05/03");
    std::fs::create_dir_all(&sessions_dir).unwrap();
    let rollout = sessions_dir.join("rollout-tree.jsonl");

    let base_canon = std::fs::canonicalize(&base).unwrap();
    let cwd = base_canon.to_str().unwrap().replace('\\', "/");
    let line1 = serde_json::json!({
        "type": "session_meta",
        "payload": { "id": "tree-id", "cwd": cwd }
    });
    let line2 = serde_json::json!({
        "type": "response_item",
        "payload": {
            "type": "message",
            "role": "user",
            "content": [{ "type": "text", "text": "nested cwd" }]
        }
    });
    std::fs::write(&rollout, format!("{line1}\n{line2}\n")).unwrap();

    let prev = std::env::var_os(CODEX_HOME_ENV);
    std::env::set_var(CODEX_HOME_ENV, &codex);
    let list = list_for_launch_dir(&launch).unwrap();
    match prev {
        Some(v) => std::env::set_var(CODEX_HOME_ENV, v),
        None => std::env::remove_var(CODEX_HOME_ENV),
    }
    let _ = std::fs::remove_dir_all(&codex);
    let _ = std::fs::remove_dir_all(&base);

    assert_eq!(list.len(), 1, "{list:?}");
    assert_eq!(list[0].first_prompt.as_deref(), Some("nested cwd"));
}

#[test]
fn session_list_item_fields() {
    let item = SessionListItem {
        agent: AgentKind::Codex,
        session_id: "id".into(),
        path: PathBuf::from("/tmp/rollout-abc.jsonl"),
        modified_at: None,
        created_at: None,
        first_prompt: Some("hi".into()),
        short_id: "abc".into(),
        subagent_label: None,
        model_provider: None,
    };
    assert_eq!(item.agent, AgentKind::Codex);
    assert_eq!(item.short_id, "abc");
    assert_eq!(item.first_prompt.as_deref(), Some("hi"));
}
