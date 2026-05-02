use ctx_analyzer::domain::ContextCategory;

#[test]
fn ordered_returns_all_five_buckets_in_pipeline_order() {
    let order = ContextCategory::ordered();
    assert_eq!(order.len(), 5);
    assert_eq!(order[0], ContextCategory::System);
    assert_eq!(order[1], ContextCategory::Configuration);
    assert_eq!(order[2], ContextCategory::Runtime);
    assert_eq!(order[3], ContextCategory::Delegated);
    assert_eq!(order[4], ContextCategory::Unknown);
}

#[test]
fn label_is_human_readable() {
    assert_eq!(
        ContextCategory::Configuration.label(),
        "Configuration Context"
    );
}
