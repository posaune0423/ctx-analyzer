use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn doctor_runs_and_mentions_codex_home() {
    Command::cargo_bin("ctx-analyzer")
        .unwrap()
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("codex home"));
}
