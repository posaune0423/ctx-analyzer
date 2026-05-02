use std::sync::Mutex;

use ctx_analyzer::application::usecases::discover_sessions;
use ctx_analyzer::constants::codex::CODEX_HOME_ENV;

static CODEX_HOME_TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn resolve_path_filters_by_filename_or_session_metadata() {
    let _lock = CODEX_HOME_TEST_LOCK.lock().unwrap();
    let base = std::env::temp_dir().join(format!(
        "ctx-analyzer-discover-session-match-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&base);
    let sessions_dir = base.join("sessions/2026/05/03");
    std::fs::create_dir_all(&sessions_dir).unwrap();

    let filename_match = sessions_dir.join("rollout-abc.jsonl");
    let metadata_only = sessions_dir.join("rollout-2026-05-03.jsonl");

    let cwd = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .replace('\\', "/");

    let line1 = serde_json::json!({
        "type": "session_meta",
        "payload": {
            "id": "meta-id-123456",
            "cwd": cwd
        }
    });
    let line2 = serde_json::json!({
        "type": "response_item",
        "payload": {
            "type": "message",
            "role": "user",
            "content": [{ "type": "text", "text": "hello" }]
        }
    });
    std::fs::write(&filename_match, format!("{line1}\n{line2}\n")).unwrap();
    std::fs::write(&metadata_only, format!("{line1}\n{line2}\n")).unwrap();

    let prev = std::env::var_os(CODEX_HOME_ENV);
    std::env::set_var(CODEX_HOME_ENV, &base);

    let sid_filename = discover_sessions::resolve_path(None, None, Some("abc")).unwrap();
    let sid_meta = discover_sessions::resolve_path(None, None, Some("meta-id")).unwrap();
    let sid_exact = discover_sessions::resolve_path(None, None, Some("meta-id-123456")).unwrap();

    match prev {
        Some(v) => std::env::set_var(CODEX_HOME_ENV, v),
        None => std::env::remove_var(CODEX_HOME_ENV),
    }
    let _ = std::fs::remove_dir_all(&base);

    assert_eq!(sid_filename, filename_match);
    assert_eq!(sid_meta, metadata_only);
    assert_eq!(sid_exact, metadata_only);
}
