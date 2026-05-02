//! Frame composition (`docs/development/wireframe/`).

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::Widget;
use ratatui::Frame;

use super::app::{AppState, Overlay, Stage};
use super::components::{
    context_accordion, footer_help, header, session_list, summary_bar, turn_list,
};
use super::overlays::{help_modal, preview_modal, warning_modal};

pub fn draw(frame: &mut Frame, state: &mut AppState) {
    state.clear_click_regions();
    let area = frame.area();

    match state.stage {
        Stage::SessionList => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Min(1),
                    Constraint::Length(1),
                ])
                .split(area);
            ratatui::widgets::Paragraph::new(session_list::header_line(&state.theme))
                .render(chunks[0], frame.buffer_mut());
            session_list::render(
                chunks[1],
                frame.buffer_mut(),
                &state.theme,
                &state.sessions,
                state.session_list_cursor,
            );
            footer_help::render(chunks[2], frame.buffer_mut(), state);
        }
        Stage::TurnList => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Min(1),
                    Constraint::Length(1),
                ])
                .split(area);
            if let Some(ld) = state.loaded.as_ref() {
                ratatui::widgets::Paragraph::new(turn_list::header_line(&state.theme))
                    .render(chunks[0], frame.buffer_mut());
                turn_list::render(
                    chunks[1],
                    frame.buffer_mut(),
                    &state.theme,
                    &ld.session,
                    &ld.turn_summaries,
                    ld.turn_list_cursor,
                );
            }
            footer_help::render(chunks[2], frame.buffer_mut(), state);
        }
        Stage::Breakdown => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Min(1),
                    Constraint::Length(1),
                ])
                .split(area);
            header::render(chunks[0], frame.buffer_mut(), state);
            summary_bar::render(chunks[1], frame.buffer_mut(), state);
            context_accordion::render(chunks[2], frame.buffer_mut(), state);
            footer_help::render(chunks[3], frame.buffer_mut(), state);
        }
    }

    if let Some((preview, scroll)) = match &state.overlay {
        Some(Overlay::Preview(p)) => Some((p.clone(), state.preview_scroll)),
        _ => None,
    } {
        preview_modal::render(area, frame.buffer_mut(), state, &preview, scroll);
        return;
    }
    match &state.overlay {
        Some(Overlay::Help) => help_modal::render(area, frame.buffer_mut(), &state.theme),
        Some(Overlay::Warning(msg)) => {
            warning_modal::render(area, frame.buffer_mut(), &state.theme, msg)
        }
        Some(Overlay::Preview(_)) | None => {}
    }
}
