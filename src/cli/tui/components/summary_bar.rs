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

    // Group granular categories for summary display
    let summary_groups = [
        (
            vec![
                ContextCategory::SystemPrompt,
                ContextCategory::ProjectDoc,
                ContextCategory::Rules,
            ],
            "Rules",
        ),
        (
            vec![
                ContextCategory::Skills,
                ContextCategory::Mcp,
                ContextCategory::Apps,
                ContextCategory::Plugins,
            ],
            "Config",
        ),
        (
            vec![
                ContextCategory::UserPrompt,
                ContextCategory::ToolCall,
                ContextCategory::AssistantMessage,
            ],
            "Runtime",
        ),
        (vec![ContextCategory::Unknown], "Unknown"),
    ];

    for (cats, label) in summary_groups {
        let mut group_total = 0u64;
        for &cat in &cats {
            group_total += bd
                .sections
                .iter()
                .find(|s| s.category == cat)
                .map(|s| s.total_tokens)
                .unwrap_or(0);
        }

        if group_total == 0 {
            continue;
        }

        // Use the style of the first category in the group as a proxy for the group style
        let style = theme.context_style(cats[0]);
        spans.push(Span::styled(label, style));
        spans.push(Span::raw(" "));
        spans.extend(token_label(theme, group_total));
        spans.push(Span::raw("   "));
    }

    Paragraph::new(Line::from(spans)).render(area, buf);
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
