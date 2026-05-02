use std::path::PathBuf;

use assert_cmd::Command;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/codex/rollout-with-subagent/ctx.jsonl")
}

#[test]
fn export_with_file_outputs_valid_json_with_expected_total() {
    let assert = Command::cargo_bin("ctx-analyzer")
        .unwrap()
        .args(["export", "--file"])
        .arg(fixture_path())
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
    let tmp = std::env::temp_dir().join(format!(
        "ctx-analyzer-export-test-{}.json",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&tmp);
    Command::cargo_bin("ctx-analyzer")
        .unwrap()
        .args(["export", "--file"])
        .arg(fixture_path())
        .arg("--out")
        .arg(&tmp)
        .assert()
        .success();
    let body = std::fs::read_to_string(&tmp).expect("output file written");
    let value: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(value["totals"]["total_tokens"].as_u64(), Some(1_513_937));
    let _ = std::fs::remove_file(&tmp);
}
