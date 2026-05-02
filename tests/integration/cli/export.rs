use std::path::PathBuf;

use assert_cmd::Command;

fn setup_mock_codex_home() -> (tempfile::TempDir, PathBuf) {
    let tmp = tempfile::tempdir().unwrap();
    let sessions_dir = tmp.path().join("sessions/2026/04/09");
    std::fs::create_dir_all(&sessions_dir).unwrap();
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/codex/rollout-with-subagent/ctx.jsonl");
    std::fs::copy(&fixture_path, sessions_dir.join("rollout-abc.jsonl")).unwrap();
    // The fixture has cwd: "/Users/asumayamada/Work/velvett-io/unigacha-contracts"
    let project_dir = PathBuf::from("/Users/asumayamada/Work/velvett-io/unigacha-contracts");
    (tmp, project_dir)
}

#[test]
fn export_with_file_outputs_valid_json_with_expected_total() {
    let (tmp, project_dir) = setup_mock_codex_home();
    let assert = Command::cargo_bin("ctx-analyzer")
        .unwrap()
        .env("CODEX_HOME", tmp.path())
        .args(["export", "--project"])
        .arg(&project_dir)
        .assert()
        .success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("stdout must be valid JSON");
    assert_eq!(
        value["totals"]["total_tokens"].as_u64(),
        Some(1_513_937),
        "totals.total_tokens did not match"
    );
    assert_eq!(value["agent"].as_str(), Some("codex"));
    assert_eq!(value["schema_version"].as_str(), Some("0.1"));
}

#[test]
fn export_with_out_writes_a_file() {
    let (tmp_home, project_dir) = setup_mock_codex_home();
    let tmp_out = std::env::temp_dir().join(format!(
        "ctx-analyzer-export-test-{}.json",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&tmp_out);
    Command::cargo_bin("ctx-analyzer")
        .unwrap()
        .env("CODEX_HOME", tmp_home.path())
        .args(["export", "--project"])
        .arg(&project_dir)
        .arg("--out")
        .arg(&tmp_out)
        .assert()
        .success();
    let body = std::fs::read_to_string(&tmp_out).expect("output file written");
    let value: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(value["totals"]["total_tokens"].as_u64(), Some(1_513_937));
    let _ = std::fs::remove_file(&tmp_out);
}
