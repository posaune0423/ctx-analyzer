use ctx_analyzer::domain::SessionGraph;

#[test]
fn root_constructor_yields_isolated_graph() {
    let g = SessionGraph::root("abc");
    assert_eq!(g.root_id, "abc");
    assert!(!g.is_delegated_child);
    assert!(g.subagent_label.is_none());
    assert!(g.children.is_empty());
}
