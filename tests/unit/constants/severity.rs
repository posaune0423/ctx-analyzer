//! Severity thresholds (UI.md §10) — single source of truth shared by
//! the breakdown view-model and the renderer.

use ctx_analyzer::constants::severity::{SEVERITY_CRITICAL, SEVERITY_HIGH, SEVERITY_MEDIUM};

#[test]
fn thresholds_match_ui_spec() {
    assert_eq!(SEVERITY_MEDIUM, 1_000, "1k boundary");
    assert_eq!(SEVERITY_HIGH, 5_000, "5k boundary");
    assert_eq!(SEVERITY_CRITICAL, 15_000, "15k boundary");
}

const _: () = {
    assert!(SEVERITY_MEDIUM < SEVERITY_HIGH);
    assert!(SEVERITY_HIGH < SEVERITY_CRITICAL);
};
