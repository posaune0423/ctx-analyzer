//! Warning modal (UI.md §24) — used to surface theme-load errors and
//! similar non-fatal issues.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap};

use crate::cli::tui::theme::Theme;

pub fn render(area: Rect, buf: &mut Buffer, theme: &Theme, message: &str) {
    let inner = centered(area, 60, 10);
    Clear.render(inner, buf);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border.warning)
        .title(Line::styled(" Warning ", theme.text.title));

    Paragraph::new(message.to_string())
        .style(theme.text.normal)
        .wrap(Wrap { trim: true })
        .block(block)
        .render(inner, buf);
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
