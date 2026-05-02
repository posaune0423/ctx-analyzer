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

const PROMPT_MAX: usize = 72;

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
            .border_style(theme.border.normal);
        let msg = Paragraph::new(Line::from(vec![Span::styled(
            "No sessions for this project (check $CODEX_HOME and session_meta.cwd).",
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
            let agent = match &s.subagent_label {
                Some(sub) => format!("{}:{}", s.agent.label(), sub),
                None => s.agent.label().to_string(),
            };
            let model = s.model_provider.as_deref().unwrap_or("—");
            let prompt = s
                .first_prompt
                .as_deref()
                .map(|c| c.replace('\n', " "))
                .map(|c| truncate_chars(&c, PROMPT_MAX))
                .unwrap_or_else(|| "—".into());

            ListItem::new(Line::from(vec![
                Span::styled(format!("{created:>14}  {updated:>14}  "), theme.text.muted),
                Span::styled(
                    format!("{:<16}  ", truncate_chars(&agent, 16)),
                    theme.text.badge,
                ),
                Span::styled(
                    format!("{:<10}  ", truncate_chars(model, 10)),
                    theme.text.normal,
                ),
                Span::styled(
                    format!("{:<10}  ", truncate_chars(&s.short_id, 10)),
                    theme.text.muted,
                ),
                Span::styled(prompt, theme.text.normal),
            ]))
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border.normal);

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

/// Column header line rendered above the list when the parent allocates a
/// `Length(1)` strip.
pub fn header_line(theme: &Theme) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("{:>14}  {:>14}  ", "Created", "Updated"),
            theme.text.muted,
        ),
        Span::styled(format!("{:<16}  ", "Agent:Subagent"), theme.text.title),
        Span::styled(format!("{:<10}  ", "Model"), theme.text.title),
        Span::styled(format!("{:<10}  ", "ID"), theme.text.title),
        Span::styled("Conversation", theme.text.title),
    ])
}
