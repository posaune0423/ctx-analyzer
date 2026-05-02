//! Empty / partial state messages (UI.md §24).

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

use crate::cli::tui::theme::Theme;

pub fn render(area: Rect, buf: &mut Buffer, theme: &Theme, message: &str) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border.warning);
    let inner = block.inner(area);
    block.render(area, buf);
    let lines: Vec<Line<'_>> = message
        .lines()
        .map(|l| Line::styled(l.to_string(), theme.text.normal))
        .collect();
    Paragraph::new(lines).render(inner, buf);
}
