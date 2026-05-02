//! Key → KeyAction dispatch (`docs/development/wireframe/key-bindings.md`).

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    SessionList,
    TurnList,
    Breakdown,
    Preview,
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyAction {
    Noop,
    Quit,
    PopStage,
    ToggleHelp,
    Refresh,
    StartSearch,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Confirm,
    OpenSessionList,
    OpenTurnList,
    SessionGraphPlaceholder,
    OpenPreview,
    OpenInEditor,
    ToggleDetail,
    ToggleViewFilter,
    CloseOverlay,
}

pub fn dispatch(mode: Mode, event: KeyEvent) -> KeyAction {
    let plain = event.modifiers == KeyModifiers::NONE || event.modifiers == KeyModifiers::SHIFT;

    if event.modifiers.contains(KeyModifiers::CONTROL) && event.code == KeyCode::Char('c') {
        return KeyAction::Quit;
    }

    match mode {
        Mode::SessionList if plain => match event.code {
            KeyCode::Char('q') | KeyCode::Esc => KeyAction::Quit,
            KeyCode::Char('?') => KeyAction::ToggleHelp,
            KeyCode::Up | KeyCode::Char('k') => KeyAction::MoveUp,
            KeyCode::Down | KeyCode::Char('j') => KeyAction::MoveDown,
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Enter => KeyAction::Confirm,
            _ => KeyAction::Noop,
        },
        Mode::TurnList if plain => match event.code {
            KeyCode::Char('q') | KeyCode::Esc => KeyAction::PopStage,
            KeyCode::Char('?') => KeyAction::ToggleHelp,
            KeyCode::Up | KeyCode::Char('k') => KeyAction::MoveUp,
            KeyCode::Down | KeyCode::Char('j') => KeyAction::MoveDown,
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Enter => KeyAction::Confirm,
            KeyCode::Char('s') => KeyAction::OpenSessionList,
            _ => KeyAction::Noop,
        },
        Mode::Breakdown if plain => match event.code {
            KeyCode::Char('q') | KeyCode::Esc => KeyAction::PopStage,
            KeyCode::Char('?') => KeyAction::ToggleHelp,
            KeyCode::Char('r') => KeyAction::Refresh,
            KeyCode::Char('/') => KeyAction::StartSearch,
            KeyCode::Up | KeyCode::Char('k') => KeyAction::MoveUp,
            KeyCode::Down | KeyCode::Char('j') => KeyAction::MoveDown,
            KeyCode::Left | KeyCode::Char('h') => KeyAction::MoveLeft,
            KeyCode::Right | KeyCode::Char('l') => KeyAction::MoveRight,
            KeyCode::Enter => KeyAction::Confirm,
            KeyCode::Char(' ') => KeyAction::OpenPreview,
            KeyCode::Char('s') => KeyAction::OpenSessionList,
            KeyCode::Char('t') => KeyAction::OpenTurnList,
            KeyCode::Char('g') => KeyAction::SessionGraphPlaceholder,
            KeyCode::Char('p') => KeyAction::OpenPreview,
            KeyCode::Char('o') => KeyAction::OpenInEditor,
            KeyCode::Char('d') => KeyAction::ToggleDetail,
            KeyCode::Char('m') => KeyAction::ToggleViewFilter,
            _ => KeyAction::Noop,
        },
        Mode::Preview if plain => match event.code {
            KeyCode::Esc | KeyCode::Char('q') => KeyAction::CloseOverlay,
            KeyCode::Char('o') => KeyAction::OpenInEditor,
            KeyCode::Up | KeyCode::Char('k') => KeyAction::MoveUp,
            KeyCode::Down | KeyCode::Char('j') => KeyAction::MoveDown,
            _ => KeyAction::Noop,
        },
        Mode::Help if plain => match event.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') => KeyAction::CloseOverlay,
            _ => KeyAction::Noop,
        },
        _ => KeyAction::Noop,
    }
}
