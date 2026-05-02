use std::path::PathBuf;

use assert_cmd::Command;
use predicates::prelude::*;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/codex/rollout-with-subagent/ctx.jsonl")
}

#[test]
fn inspect_with_file_renders_categories_and_total() {
    Command::cargo_bin("ctx-analyzer")
        .unwrap()
        .args(["inspect", "--file"])
        .arg(fixture_path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Configuration Context"))
        .stdout(predicate::str::contains("Runtime Context"))
        .stdout(predicate::str::contains("Delegated Context"))
        .stdout(predicate::str::contains("1,513,937"));
}
