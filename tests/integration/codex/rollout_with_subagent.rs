//! Integration test: parse the bundled subagent rollout fixture and
//! assert end-to-end invariants for the Codex adapter.

use std::path::PathBuf;

use ctx_analyzer::adapters::codex::CodexAdapter;
use ctx_analyzer::domain::{Confidence, ContextCategory, ContextSourceKind};
use ctx_analyzer::infra::CharsPer4Estimator;
use ctx_analyzer::ports::AgentAdapter;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/codex/rollout-with-subagent/ctx.jsonl")
}

#[test]
fn parses_bundled_codex_subagent_rollout() {
    let path = fixture_path();
    assert!(path.exists(), "fixture not found at {}", path.display());
    let adapter = CodexAdapter::new();
    let estimator = CharsPer4Estimator;
    let session = adapter
        .parse_file(&path, &estimator)
        .expect("adapter parse_file");

    // Identity
    assert_eq!(session.id, "019d72f3-f339-71c3-810e-f714359dcdff");

    // Subagent / delegated child
    assert_eq!(session.source.subagent.as_deref(), Some("review"));
    assert!(session.graph.is_delegated_child);
    assert_eq!(session.graph.subagent_label.as_deref(), Some("review"));

    // Single turn
    assert_eq!(session.turns.len(), 1, "exactly one turn in fixture");
    let turn = &session.turns[0];
    assert_eq!(turn.id, "019d72f3-f851-7591-a9c1-e1b6484f3293");

    // Timestamp from task_started (line 2).
    let started_at = turn.started_at.expect("turn started_at populated");
    assert_eq!(started_at.to_rfc3339(), "2026-04-09T15:54:54.177+00:00");

    // Observed totals from final non-null token_count event (line 95).
    let totals = session
        .session_totals
        .expect("totals from final token_count event");
    assert_eq!(totals.tokens, 1_513_937);
    assert_eq!(totals.confidence, Confidence::Observed);
    assert_eq!(totals.input, Some(1_503_435));
    assert_eq!(totals.output, Some(10_502));
    assert_eq!(totals.reasoning_output, Some(8_207));

    // Categories present and populated
    let by_cat = |c: ContextCategory| session.segments.iter().filter(move |s| s.category == c);
    assert!(
        by_cat(ContextCategory::SystemPrompt).count() >= 1,
        "system prompt"
    );

    // Config categories are now granular
    assert!(by_cat(ContextCategory::SystemPrompt)
        .any(|s| s.source_kind == ContextSourceKind::PermissionsInstructions));
    assert!(
        by_cat(ContextCategory::Apps).any(|s| s.source_kind == ContextSourceKind::AppsInstructions)
    );
    assert!(by_cat(ContextCategory::Skills)
        .any(|s| s.source_kind == ContextSourceKind::SkillsInstructions));
    assert!(by_cat(ContextCategory::Plugins)
        .any(|s| s.source_kind == ContextSourceKind::PluginsInstructions));
    assert!(by_cat(ContextCategory::ProjectDoc)
        .any(|s| s.source_kind == ContextSourceKind::ProjectInstructions));

    assert!(
        by_cat(ContextCategory::ToolCall).any(|s| s.source_kind == ContextSourceKind::FunctionCall),
        "tool call missing function calls"
    );
    assert!(
        by_cat(ContextCategory::Unknown)
            .any(|s| s.source_kind == ContextSourceKind::EncryptedReasoning),
        "unknown missing encrypted reasoning"
    );

    // Source-ref invariants
    for seg in &session.segments {
        let file_str = seg.source_ref.file.to_string_lossy();
        assert!(file_str.ends_with("ctx.jsonl"));
        let line = seg.source_ref.line.expect("line populated");
        assert!((1..=96).contains(&line), "line {line} out of range");
    }

    // Reasoning segments have Unknown confidence
    for seg in &session.segments {
        if seg.source_kind == ContextSourceKind::EncryptedReasoning {
            assert_eq!(seg.confidence, Confidence::Unknown);
        }
    }
}
