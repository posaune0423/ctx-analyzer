use ctx_analyzer::adapters::codex::classifiers::{looks_like_project_doc, split_developer_blocks};
use ctx_analyzer::domain::ContextSourceKind;

#[test]
fn splits_all_four_tagged_blocks_in_order() {
    let text = "preamble\n\
                <permissions instructions>\nperms body\n</permissions instructions>\n\
                middle text\n\
                <apps_instructions>\napps body\n</apps_instructions>\n\
                <skills_instructions>\nskills body\n</skills_instructions>\n\
                <plugins_instructions>\nplugins body\n</plugins_instructions>\n\
                trailing text";
    let blocks = split_developer_blocks(text);
    let kinds: Vec<_> = blocks.iter().map(|b| b.kind).collect();
    assert_eq!(
        kinds,
        vec![
            ContextSourceKind::PermissionsInstructions,
            ContextSourceKind::AppsInstructions,
            ContextSourceKind::SkillsInstructions,
            ContextSourceKind::PluginsInstructions,
            ContextSourceKind::BaseInstructions,
        ],
        "expected four tagged blocks in source order plus a leftover BaseInstructions"
    );
    let perms = blocks
        .iter()
        .find(|b| b.kind == ContextSourceKind::PermissionsInstructions)
        .unwrap();
    assert_eq!(perms.body, "perms body");
    let leftover = blocks
        .iter()
        .find(|b| b.kind == ContextSourceKind::BaseInstructions)
        .unwrap();
    assert!(!leftover.body.contains("<permissions"));
    assert!(!leftover.body.contains("</permissions"));
    assert!(leftover.body.contains("preamble"));
    assert!(leftover.body.contains("trailing text"));
}

#[test]
fn unmatched_open_tag_is_skipped_no_panic() {
    let text = "<permissions instructions>\nbody but no close";
    let blocks = split_developer_blocks(text);
    assert!(blocks
        .iter()
        .all(|b| b.kind != ContextSourceKind::PermissionsInstructions));
    assert!(blocks
        .iter()
        .any(|b| b.kind == ContextSourceKind::BaseInstructions));
}

#[test]
fn project_doc_prefix_detected() {
    assert!(looks_like_project_doc(
        "# AGENTS.md instructions for /repo/foo\n…"
    ));
    assert!(!looks_like_project_doc("Just a regular user prompt"));
}
