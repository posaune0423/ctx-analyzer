//! ContextAccordion — Section / Row tree (`AppState::visible_nodes`).
//! Redesigned for 2-level hierarchy (Category > Segments).

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget, Widget};

use crate::application::view_models::context_breakdown::{Breakdown, Severity};
use crate::cli::tui::app::{AppState, NodeKind, ViewFilter, VisibleNode};
use crate::cli::tui::theme::Theme;
use crate::constants::preview::LABEL_TRUNCATE_CHARS;
use crate::utils::format::fmt_thousands;
use crate::utils::text::truncate_chars;

pub fn render(area: Rect, buf: &mut Buffer, state: &AppState) {
    let theme = &state.theme;
    let Some(ld) = state.loaded.as_ref() else {
        return;
    };
    let cursor = ld.breakdown_cursor;
    let breakdown = &ld.breakdown;
    let view_filter = ld.view_filter;

    let nodes = state.visible_nodes();
    let detail = state.detail_mode;
    let items: Vec<ListItem<'_>> = nodes
        .iter()
        .enumerate()
        .map(|(idx, node)| build_item(*node, breakdown, theme, idx == cursor, detail, view_filter))
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border.normal)
        .title(Span::styled(" Context Breakdown ", theme.text.title));

    let list = List::new(items).block(block);
    let mut list_state = ListState::default();
    if !nodes.is_empty() {
        list_state.select(Some(cursor));
    }
    StatefulWidget::render(list, area, buf, &mut list_state);

    if nodes.is_empty() {
        let msg = Line::from(Span::styled(
            "No context segments to display.",
            theme.text.muted,
        ));
        let para = ratatui::widgets::Paragraph::new(msg);
        let inner = inner_area(area);
        para.render(inner, buf);
    }
}

fn inner_area(area: Rect) -> Rect {
    Rect {
        x: area.x + 2,
        y: area.y + 1,
        width: area.width.saturating_sub(4),
        height: area.height.saturating_sub(2),
    }
}

#[allow(clippy::too_many_arguments)]
fn build_item<'a>(
    node: VisibleNode,
    breakdown: &'a Breakdown,
    theme: &'a Theme,
    selected: bool,
    detail: bool,
    view_filter: ViewFilter,
) -> ListItem<'a> {
    let lines = match node.kind {
        NodeKind::Section => section_lines(node, breakdown, theme, selected),
        NodeKind::Row => row_lines(node, breakdown, theme, selected, detail, view_filter),
    };
    ListItem::new(lines)
}

fn section_lines<'a>(
    node: VisibleNode,
    breakdown: &'a Breakdown,
    theme: &'a Theme,
    selected: bool,
) -> Vec<Line<'a>> {
    let Some(section) = breakdown.sections.get(node.section) else {
        return Vec::new();
    };
    let marker = if section.rows.is_empty() { "·" } else { "▾" };
    let row_style = if selected {
        theme.row.selected
    } else {
        theme.row.expanded
    };
    let cat_style = theme.context_style(section.category);
    let token = theme.token_for(Severity::from_total(section.total_tokens));
    let mut spans: Vec<Span<'_>> = vec![
        Span::styled(format!("{marker} "), row_style),
        Span::styled(section.label(), apply_selection(cat_style, selected, theme)),
        Span::raw("  "),
        Span::styled(
            format!("~{}", fmt_thousands(section.total_tokens)),
            token.style,
        ),
    ];
    if !token.symbol.is_empty() {
        spans.push(Span::raw(" "));
        spans.push(Span::styled(token.symbol, token.style));
    }
    vec![Line::from(spans)]
}

#[allow(clippy::too_many_arguments)]
fn row_lines<'a>(
    node: VisibleNode,
    breakdown: &'a Breakdown,
    theme: &'a Theme,
    selected: bool,
    detail: bool,
    view_filter: ViewFilter,
) -> Vec<Line<'a>> {
    let r_idx = match node.row {
        Some(r) => r,
        None => return Vec::new(),
    };
    let Some(section) = breakdown.sections.get(node.section) else {
        return Vec::new();
    };
    let Some(row) = section.rows.get(r_idx) else {
        return Vec::new();
    };
    let token = theme.token_for(row.severity);
    let row_style = if selected {
        theme.row.selected
    } else {
        theme.text.normal
    };
    let label = truncate_chars(&row.label, LABEL_TRUNCATE_CHARS);
    let mut spans: Vec<Span<'_>> = vec![
        Span::styled("  · ", row_style),
        Span::styled(label, apply_selection(theme.text.normal, selected, theme)),
        Span::raw("  "),
        Span::styled(
            format!("~{}", fmt_thousands(row.tokens.tokens)),
            token.style,
        ),
    ];
    if !token.symbol.is_empty() {
        spans.push(Span::raw(" "));
        spans.push(Span::styled(token.symbol, token.style));
    }

    if row.turn_id.is_none() {
        spans.push(Span::raw("  "));
        let tag = if view_filter == ViewFilter::Delta {
            "[inherited]"
        } else {
            "[session-level]"
        };
        spans.push(Span::styled(tag, theme.text.muted));
    }

    let mut lines = vec![Line::from(spans)];

    if detail && selected {
        let path = format!("    path: {}", row.source_ref.file.display());
        lines.push(Line::from(Span::styled(path, theme.text.path)));
        let meta = format!(
            "    category: {} · confidence: {:?}",
            section.label(),
            row.confidence
        );
        lines.push(Line::from(Span::styled(meta, theme.text.muted)));
    }

    lines
}

fn apply_selection(base: Style, selected: bool, theme: &Theme) -> Style {
    if selected {
        theme.row.selected
    } else {
        base
    }
}
