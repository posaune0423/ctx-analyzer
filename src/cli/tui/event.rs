//! Crossterm-driven event loop.

use std::io;

use anyhow::Context as _;
use crossterm::event::{self, Event, KeyEventKind, MouseButton, MouseEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use super::app::{AppCommand, AppState};
use super::keymap;
use super::open_editor;
use super::render;

pub fn run(state: &mut AppState) -> anyhow::Result<()> {
    if !is_tty() {
        anyhow::bail!("ctx-analyzer inspect requires a TTY");
    }

    let _guard = TerminalGuard::enter().context("failed to set up terminal")?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    while !state.should_quit {
        terminal.draw(|f| render::draw(f, state))?;

        if !event::poll(std::time::Duration::from_millis(250))? {
            continue;
        }
        let evt = event::read()?;
        match evt {
            Event::Key(k) if k.kind == KeyEventKind::Press => {
                let action = keymap::dispatch(state.mode(), k);
                let cmd = state.apply(action);
                if let AppCommand::OpenEditor(target) = cmd {
                    handle_editor(&mut terminal, state, target)?;
                }
            }
            Event::Mouse(m) if m.kind == MouseEventKind::Down(MouseButton::Left) => {
                let cmd = state.handle_mouse_click(m.column, m.row);
                if let AppCommand::OpenEditor(target) = cmd {
                    handle_editor(&mut terminal, state, target)?;
                }
            }
            Event::Resize(_, _) => {}
            _ => {}
        }
    }

    Ok(())
}

fn handle_editor(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut AppState,
    target: Option<super::app::EditorTarget>,
) -> anyhow::Result<()> {
    let Some(target) = target else {
        state.footer_message = Some("This segment has no source file.".into());
        return Ok(());
    };
    leave_screen()?;
    let result = open_editor::launch(&target.path, target.line);
    enter_screen()?;
    terminal.clear()?;
    if let Err(err) = result {
        state.footer_message = Some(format!("editor: {err}"));
    }
    Ok(())
}

struct TerminalGuard;

impl TerminalGuard {
    fn enter() -> anyhow::Result<Self> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        Ok(TerminalGuard)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

fn enter_screen() -> anyhow::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    Ok(())
}

fn leave_screen() -> anyhow::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn is_tty() -> bool {
    use std::io::IsTerminal;
    io::stdout().is_terminal() && io::stdin().is_terminal()
}
