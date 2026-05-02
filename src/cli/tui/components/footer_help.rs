//! Footer help (`docs/development/wireframe/key-bindings.md`).

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

use crate::cli::tui::app::AppState;
use crate::cli::tui::keymap::Mode;

pub fn render(area: Rect, buf: &mut Buffer, state: &AppState) {
    let theme = &state.theme;
    let hint = match state.mode() {
        Mode::SessionList => "↑↓ select · enter open · ? help · q quit",
        Mode::TurnList => "↑↓ select · enter inspect · s sessions · ? help · q back",
        Mode::Breakdown => "↑↓ move · ←→ expand · enter/p/space preview · l drill · o open · m mode · t turns · s sess · ? help · q back",
        Mode::Preview => "↑↓ scroll · o open · esc close",
        Mode::Help => "esc close",
    };

    let mut spans: Vec<Span<'_>> = vec![Span::styled(hint, theme.text.muted)];
    if let Some(msg) = &state.footer_message {
        spans.push(Span::raw("   "));
        spans.push(Span::styled(msg.clone(), theme.text.subtitle));
    }

    Paragraph::new(Line::from(spans)).render(area, buf);
}
