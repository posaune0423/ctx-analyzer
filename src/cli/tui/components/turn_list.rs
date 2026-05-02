//! Turn List stage (`docs/development/wireframe/turn-list.md`).
//! Redesigned for modern TUI aesthetic, matching Session List structure.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget, Widget};

use crate::application::usecases::list_session_turns::TurnSummary;
use crate::cli::tui::theme::Theme;
use crate::domain::Session;
use crate::utils::format::{fmt_thousands, relative_time_utc};
use crate::utils::text::{pad_to_width, truncate_chars};

const PROMPT_MAX: usize = 64;

pub fn render(
    area: Rect,
    buf: &mut Buffer,
    theme: &Theme,
    _session: &Session,
    summaries: &[TurnSummary],
    cursor: usize,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border.normal);

    if summaries.is_empty() {
        ratatui::widgets::Paragraph::new(Line::from(Span::styled(
            "No interactions recorded for this session.",
            theme.text.subtitle,
        )))
        .block(block)
        .render(area, buf);
        return;
    }

    let items: Vec<ListItem<'_>> = summaries
        .iter()
        .map(|t| {
            let index = format!("{:>2}", t.index);
            let when = relative_time_utc(t.started_at);
            let model = t.model.as_deref().unwrap_or("—");
            let prompt = t
                .prompt_preview
                .as_deref()
                .map(|p| p.replace('\n', " "))
                .map(|p| truncate_chars(&p, PROMPT_MAX))
                .unwrap_or_else(|| "(no user prompt recorded)".to_string());
            let tokens_str = format!("~{}", fmt_thousands(t.total_tokens));

            ListItem::new(Line::from(vec![
                Span::styled(format!("{index}  "), theme.text.muted),
                Span::styled(format!("{when:<14}  "), theme.text.muted),
                Span::styled(
                    format!("{:<16}  ", truncate_chars(model, 16)),
                    theme.text.badge,
                ),
                Span::styled(pad_to_width(&prompt, PROMPT_MAX), theme.text.normal),
                Span::raw("  "),
                Span::styled(format!("{tokens_str:>12}"), theme.text.normal),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(theme.row.selected)
        .highlight_symbol("> ");

    let mut state = ListState::default();
    if !summaries.is_empty() {
        state.select(Some(cursor.min(summaries.len() - 1)));
    }
    StatefulWidget::render(list, area, buf, &mut state);
}

pub fn header_line(theme: &Theme) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("  {:>2}  {:<14}  ", "#", "When"), theme.text.muted),
        Span::styled(format!("{:<16}  ", "Model"), theme.text.title),
        Span::styled(format!("{:<64}  ", "Conversation"), theme.text.title),
        Span::styled(format!("{:>12}", "Tokens"), theme.text.title),
    ])
}
