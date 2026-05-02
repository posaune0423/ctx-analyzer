//! Turn List stage (`docs/development/wireframe/turn-list.md`).

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget, Widget};

use crate::application::usecases::list_session_turns::TurnSummary;
use crate::cli::tui::theme::Theme;
use crate::domain::Session;
use crate::utils::format::{fmt_thousands, relative_time_utc};
use crate::utils::text::truncate_chars;

const PROMPT_MAX: usize = 72;

pub fn render(
    area: Rect,
    buf: &mut Buffer,
    theme: &Theme,
    session: &Session,
    summaries: &[TurnSummary],
    cursor: usize,
) {
    let title = title_line(session);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border.normal)
        .title(Span::styled(title, theme.text.title));

    if summaries.is_empty() {
        ratatui::widgets::Paragraph::new(Line::from(Span::styled(
            "No turns recorded for this session.",
            theme.text.subtitle,
        )))
        .block(block)
        .render(area, buf);
        return;
    }

    let items: Vec<ListItem<'_>> = summaries
        .iter()
        .map(|t| {
            let when = relative_time_utc(t.started_at);
            let prompt = t
                .prompt_preview
                .as_deref()
                .map(|p| truncate_chars(p, PROMPT_MAX))
                .unwrap_or_else(|| "(no user prompt recorded)".to_string());
            let warn = if t.has_subagent { " ⚠" } else { "" };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{:>3}  ", t.index), theme.text.muted),
                Span::styled(
                    format!("~{}  ", fmt_thousands(t.total_tokens)),
                    theme.text.normal,
                ),
                Span::styled(format!("{when:<14}  "), theme.text.muted),
                Span::styled(format!("{prompt}{warn}"), theme.text.normal),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(theme.row.selected)
        .highlight_symbol("> ");

    let mut state = ListState::default();
    state.select(Some(cursor.min(summaries.len() - 1)));
    StatefulWidget::render(list, area, buf, &mut state);
}

fn title_line(session: &Session) -> String {
    let sid = short_id(&session.id);
    let cwd = session
        .cwd
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| session.source_path.display().to_string());
    let n = session.turns.len();
    format!(" ctx-analyzer · {sid} · {cwd} · {n} turns ")
}

fn short_id(s: &str) -> String {
    if s.chars().count() <= 12 {
        s.to_string()
    } else {
        let head: String = s.chars().take(8).collect();
        let tail: String = s
            .chars()
            .rev()
            .take(4)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        format!("{head}…{tail}")
    }
}

pub fn header_line(theme: &Theme) -> Line<'static> {
    Line::from(vec![Span::styled(
        "  #    Tokens      When            User Prompt",
        theme.text.muted,
    )])
}
