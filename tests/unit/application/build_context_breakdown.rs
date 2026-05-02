//! Unit-level shape test: an empty session produces 5 empty category
//! sections (System / Configuration / Runtime / Delegated / Unknown).

use std::path::PathBuf;

use ctx_analyzer::application::usecases::build_context_breakdown;
use ctx_analyzer::application::view_models::context_breakdown::Severity;
use ctx_analyzer::domain::{AgentKind, ContextCategory, Session, SessionGraph, SessionSource};

fn empty_session() -> Session {
    Session {
        id: "test".into(),
        agent: AgentKind::Codex,
        started_at: None,
        cwd: None,
        source_path: PathBuf::from("nowhere.jsonl"),
        source: SessionSource::default(),
        model_provider: None,
        cli_version: None,
        turns: vec![],
        segments: vec![],
        session_totals: None,
        graph: SessionGraph::root("test"),
        warnings: vec![],
    }
}

#[test]
fn empty_session_yields_all_five_category_sections() {
    let bd = build_context_breakdown::group(&empty_session());
    let cats: Vec<_> = bd.sections.iter().map(|s| s.category).collect();
    assert_eq!(cats.len(), 5);
    assert!(cats.contains(&ContextCategory::System));
    assert!(cats.contains(&ContextCategory::Configuration));
    assert!(cats.contains(&ContextCategory::Runtime));
    assert!(cats.contains(&ContextCategory::Delegated));
    assert!(cats.contains(&ContextCategory::Unknown));
    assert_eq!(bd.total_tokens, 0);
}

#[test]
fn severity_thresholds_match_ui_spec() {
    assert_eq!(Severity::from_total(500), Severity::Low);
    assert_eq!(Severity::from_total(2_000), Severity::Medium);
    assert_eq!(Severity::from_total(8_000), Severity::High);
    assert_eq!(Severity::from_total(20_000), Severity::Critical);
    assert_eq!(Severity::High.symbol(), "!");
    assert_eq!(Severity::Critical.symbol(), "!!");
    assert_eq!(Severity::Unknown.symbol(), "?");
}
