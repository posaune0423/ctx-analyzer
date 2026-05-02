//! Help overlay (`docs/development/wireframe/help-modal.md`).

use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Clear, Row, Table, Widget};

use crate::cli::tui::theme::Theme;

const HELP: &[(&str, &str)] = &[
    ("q / esc", "quit or go back one stage"),
    ("?", "toggle this help"),
    ("↑↓ / jk", "move"),
    ("←→ / hl", "collapse / expand (breakdown)"),
    ("l / enter", "on leaf row: open preview"),
    ("space / p", "preview segment"),
    ("o", "open source in editor"),
    ("s", "session list"),
    ("t", "turn list (from breakdown)"),
    ("m", "toggle cumulative / delta view"),
    ("d", "toggle row detail"),
    ("r", "refresh breakdown"),
    ("g", "session graph (coming soon)"),
    ("/", "search (coming soon)"),
];

pub fn render(area: Rect, buf: &mut Buffer, theme: &Theme) {
    let inner = centered(area, 64, 24);
    Clear.render(inner, buf);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border.focused)
        .title(Span::styled(" Help ", theme.text.title));

    let rows: Vec<Row<'_>> = HELP
        .iter()
        .map(|(k, label)| {
            Row::new(vec![
                Cell::from(Span::styled(*k, theme.text.badge)),
                Cell::from(Span::styled(*label, theme.text.normal)),
            ])
        })
        .collect();

    let table = Table::new(rows, [Constraint::Length(18), Constraint::Min(20)])
        .block(block)
        .header(Row::new(vec![
            Cell::from(Line::from(Span::styled("Key", theme.text.title))),
            Cell::from(Line::from(Span::styled("Action", theme.text.title))),
        ]));

    Widget::render(table, inner, buf);
}

fn centered(area: Rect, w: u16, h: u16) -> Rect {
    let w = w.min(area.width);
    let h = h.min(area.height);
    Rect {
        x: area.x + (area.width.saturating_sub(w)) / 2,
        y: area.y + (area.height.saturating_sub(h)) / 2,
        width: w,
        height: h,
    }
}
