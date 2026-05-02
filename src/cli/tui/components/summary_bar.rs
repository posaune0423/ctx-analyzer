//! SummaryBar — category token totals (`UI.md` §8).

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Modifier;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

use crate::application::view_models::context_breakdown::Severity;
use crate::cli::tui::app::AppState;
use crate::cli::tui::theme::Theme;
use crate::domain::ContextCategory;
use crate::utils::format::fmt_thousands;

pub fn render(area: Rect, buf: &mut Buffer, state: &AppState) {
    let theme = &state.theme;
    let Some(ld) = state.loaded.as_ref() else {
        return;
    };
    let bd = &ld.breakdown;

    let mut spans: Vec<Span<'_>> = Vec::new();
    spans.push(Span::styled("Total", theme.text.title));
    spans.push(Span::raw(" "));
    spans.extend(token_label(theme, bd.total_tokens));
    spans.push(Span::raw("   "));

    for cat in [
        ContextCategory::System,
        ContextCategory::Configuration,
        ContextCategory::Runtime,
        ContextCategory::Delegated,
        ContextCategory::Unknown,
    ] {
        let total = bd
            .sections
            .iter()
            .find(|s| s.category == cat)
            .map(|s| s.total_tokens)
            .unwrap_or(0);
        if total == 0 {
            continue;
        }
        let label = short_label(cat);
        spans.push(Span::styled(label, theme.context_style(cat)));
        spans.push(Span::raw(" "));
        spans.extend(token_label(theme, total));
        spans.push(Span::raw("   "));
    }

    Paragraph::new(Line::from(spans)).render(area, buf);
}

fn short_label(cat: ContextCategory) -> &'static str {
    match cat {
        ContextCategory::System => "System",
        ContextCategory::Configuration => "Config",
        ContextCategory::Runtime => "Runtime",
        ContextCategory::Delegated => "Delegated",
        ContextCategory::Unknown => "Unknown",
    }
}

fn token_label<'a>(theme: &'a Theme, total: u64) -> Vec<Span<'a>> {
    let severity = Severity::from_total(total);
    let token = theme.token_for(severity);
    let mut style = token.style;
    if !token.symbol.is_empty() {
        style = style.add_modifier(Modifier::BOLD);
    }
    let mut out = vec![Span::styled(format!("~{}", fmt_thousands(total)), style)];
    if !token.symbol.is_empty() {
        out.push(Span::raw(" "));
        out.push(Span::styled(token.symbol, style));
    }
    out
}
