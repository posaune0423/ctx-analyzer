//! Severity → ratatui Style mapping (UI.md §10).

use ctx_analyzer::application::view_models::context_breakdown::Severity;
use ctx_analyzer::cli::tui::theme::Theme;
use ratatui::style::Modifier;

#[test]
fn severity_high_uses_warning_with_bang() {
    let theme = Theme::builtin();
    let token = theme.token_for(Severity::High);
    assert_eq!(token.symbol, "!");
    assert!(token.style.add_modifier.contains(Modifier::BOLD));
}

#[test]
fn severity_critical_uses_double_bang() {
    let theme = Theme::builtin();
    let token = theme.token_for(Severity::Critical);
    assert_eq!(token.symbol, "!!");
    assert!(token.style.add_modifier.contains(Modifier::BOLD));
}

#[test]
fn severity_unknown_uses_question_mark() {
    let theme = Theme::builtin();
    let token = theme.token_for(Severity::Unknown);
    assert_eq!(token.symbol, "?");
}

#[test]
fn severity_low_has_no_symbol() {
    let theme = Theme::builtin();
    assert_eq!(theme.token_for(Severity::Low).symbol, "");
    assert_eq!(theme.token_for(Severity::Medium).symbol, "");
}
