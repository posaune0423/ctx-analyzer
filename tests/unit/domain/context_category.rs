use ctx_analyzer::domain::{AgentKind, ContextCategory};

#[test]
fn ordered_returns_all_eleven_buckets_in_pipeline_order() {
    let order = ContextCategory::ordered();
    assert_eq!(order.len(), 11);
    assert_eq!(order[0], ContextCategory::SystemPrompt);
    assert_eq!(order[1], ContextCategory::ProjectDoc);
    assert_eq!(order[10], ContextCategory::Unknown);
}

#[test]
fn label_is_human_readable_and_agent_specific() {
    assert_eq!(
        ContextCategory::ProjectDoc.label(AgentKind::Codex),
        "AGENTS.md"
    );
    assert_eq!(
        ContextCategory::ProjectDoc.label(AgentKind::ClaudeCode),
        "CLAUDE.md"
    );
    assert_eq!(
        ContextCategory::ProjectDoc.label(AgentKind::Gemini),
        "GEMINI.md"
    );
    assert_eq!(
        ContextCategory::SystemPrompt.label(AgentKind::Codex),
        "system prompt"
    );
}
