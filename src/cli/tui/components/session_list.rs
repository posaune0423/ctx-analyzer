//! Session List stage (`docs/development/wireframe/session-list.md`).

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget,
};

use crate::application::usecases::list_workspace_sessions::SessionListItem;
use crate::cli::tui::theme::Theme;
use crate::utils::format::relative_time_ago;
use crate::utils::text::truncate_chars;

const CWD_MAX: usize = 32;
const CONV_MAX: usize = 48;

pub fn render(
    area: Rect,
    buf: &mut Buffer,
    theme: &Theme,
    sessions: &[SessionListItem],
    cursor: usize,
) {
    if sessions.is_empty() {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme.border.normal)
            .title(Span::styled(" Select Session ", theme.text.title));
        let msg = Paragraph::new(Line::from(vec![Span::styled(
            "No Codex sessions found under $CODEX_HOME",
            theme.text.subtitle,
        )]))
        .block(block);
        msg.render(area, buf);
        return;
    }

    let items: Vec<ListItem<'_>> = sessions
        .iter()
        .map(|s| {
            let created = relative_time_ago(s.created_at);
            let updated = relative_time_ago(s.modified_at);
            let cwd = s
                .cwd
                .as_ref()
                .map(|p| truncate_middle(&p.display().to_string(), CWD_MAX))
                .unwrap_or_else(|| "—".into());
            let conv = s
                .conversation
                .as_deref()
                .map(|c| truncate_chars(c, CONV_MAX))
                .unwrap_or_else(|| "—".into());
            ListItem::new(Line::from(vec![
                Span::styled(format!("{created:>14}  {updated:>14}  "), theme.text.muted),
                Span::styled(
                    format!("{cwd:<w$}  ", cwd = cwd, w = CWD_MAX),
                    theme.text.path,
                ),
                Span::styled(conv, theme.text.normal),
            ]))
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border.normal)
        .title(Span::styled(" Select Session ", theme.text.title));

    let list = List::new(items)
        .block(block)
        .highlight_style(theme.row.selected)
        .highlight_symbol("> ");

    let mut state = ListState::default();
    if !sessions.is_empty() {
        state.select(Some(cursor.min(sessions.len() - 1)));
    }
    StatefulWidget::render(list, area, buf, &mut state);
}

fn truncate_middle(text: &str, max: usize) -> String {
    if max == 0 {
        return String::new();
    }
    let chars: Vec<char> = text.chars().collect();
    if chars.len() <= max {
        return text.to_string();
    }
    if max == 1 {
        return "…".to_string();
    }
    let head = max / 2;
    let tail = max - head - 1;
    let head_s: String = chars.iter().take(head).collect();
    let tail_s: String = chars
        .iter()
        .rev()
        .take(tail)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    format!("{head_s}…{tail_s}")
}

/// Column header line rendered above the list when the parent allocates a
/// `Length(1)` strip.
pub fn header_line(theme: &Theme) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("{:>14}  {:>14}  ", "Created", "Updated"),
            theme.text.muted,
        ),
        Span::styled(format!("{:<w$}  ", "CWD", w = CWD_MAX), theme.text.title),
        Span::styled("Conversation", theme.text.title),
    ])
}
