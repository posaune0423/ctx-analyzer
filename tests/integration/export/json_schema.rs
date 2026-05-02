//! Integration test: JSON export schema invariants
//! (`docs/specs/export-json.md`).

use std::path::PathBuf;

use ctx_analyzer::adapters::codex::CodexAdapter;
use ctx_analyzer::application::usecases::export_json::JsonExporter;
use ctx_analyzer::infra::CharsPer4Estimator;
use ctx_analyzer::ports::{AgentAdapter, Exporter};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/codex/rollout-with-subagent/ctx.jsonl")
}

#[test]
fn json_export_has_stable_top_level_keys() {
    let adapter = CodexAdapter::new();
    let estimator = CharsPer4Estimator;
    let session = adapter.parse_file(&fixture_path(), &estimator).unwrap();
    let value = JsonExporter.export(&session);
    let obj = value.as_object().expect("export is JSON object");
    for key in [
        "schema_version",
        "agent",
        "session",
        "totals",
        "categories",
        "turns",
        "warnings",
    ] {
        assert!(obj.contains_key(key), "missing top-level key: {key}");
    }
    assert_eq!(obj["schema_version"].as_str(), Some("0.1"));
    assert_eq!(obj["agent"].as_str(), Some("codex"));
}

#[test]
fn totals_object_uses_canonical_key_names() {
    let adapter = CodexAdapter::new();
    let estimator = CharsPer4Estimator;
    let session = adapter.parse_file(&fixture_path(), &estimator).unwrap();
    let value = JsonExporter.export(&session);
    let totals = &value["totals"];
    assert_eq!(totals["total_tokens"].as_u64(), Some(1_513_937));
    assert_eq!(totals["input_tokens"].as_u64(), Some(1_503_435));
    assert_eq!(totals["output_tokens"].as_u64(), Some(10_502));
    assert_eq!(totals["confidence"].as_str(), Some("Observed"));
}

#[test]
fn categories_object_lists_all_five_buckets() {
    let adapter = CodexAdapter::new();
    let estimator = CharsPer4Estimator;
    let session = adapter.parse_file(&fixture_path(), &estimator).unwrap();
    let value = JsonExporter.export(&session);
    let categories = value["categories"].as_object().unwrap();
    for k in ["system", "configuration", "runtime", "delegated", "unknown"] {
        assert!(categories.contains_key(k), "missing category: {k}");
    }
}
