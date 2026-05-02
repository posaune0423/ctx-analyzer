//! Key → KeyAction dispatch (wireframe `key-bindings.md`).
//!
//! Pure layer: `(Mode, KeyEvent) -> KeyAction`, no terminal IO.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ctx_analyzer::cli::tui::keymap::{dispatch, KeyAction, Mode};

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

#[test]
fn q_and_ctrl_c_quit_from_session_list() {
    assert_eq!(
        dispatch(Mode::SessionList, key(KeyCode::Char('q'))),
        KeyAction::Quit
    );
    let ctrl_c = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
    assert_eq!(dispatch(Mode::SessionList, ctrl_c), KeyAction::Quit);
}

#[test]
fn q_pops_stage_from_turn_and_breakdown() {
    assert_eq!(
        dispatch(Mode::TurnList, key(KeyCode::Char('q'))),
        KeyAction::PopStage
    );
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::Char('q'))),
        KeyAction::PopStage
    );
}

#[test]
fn navigation_arrows_and_vim_keys_in_breakdown() {
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::Up)),
        KeyAction::MoveUp
    );
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::Down)),
        KeyAction::MoveDown
    );
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::Char('k'))),
        KeyAction::MoveUp
    );
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::Char('j'))),
        KeyAction::MoveDown
    );
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::Left)),
        KeyAction::MoveLeft
    );
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::Right)),
        KeyAction::MoveRight
    );
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::Char('h'))),
        KeyAction::MoveLeft
    );
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::Char('l'))),
        KeyAction::MoveRight
    );
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::Enter)),
        KeyAction::Confirm
    );
}

#[test]
fn breakdown_shortcuts_for_lists_preview_and_detail() {
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::Char('s'))),
        KeyAction::OpenSessionList
    );
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::Char('t'))),
        KeyAction::OpenTurnList
    );
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::Char('p'))),
        KeyAction::OpenPreview
    );
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::Char(' '))),
        KeyAction::OpenPreview
    );
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::Char('o'))),
        KeyAction::OpenInEditor
    );
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::Char('?'))),
        KeyAction::ToggleHelp
    );
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::Char('r'))),
        KeyAction::Refresh
    );
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::Char('d'))),
        KeyAction::ToggleDetail
    );
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::Char('m'))),
        KeyAction::ToggleViewFilter
    );
}

#[test]
fn session_list_does_not_handle_breakdown_only_keys() {
    assert_eq!(
        dispatch(Mode::SessionList, key(KeyCode::Char('s'))),
        KeyAction::Noop
    );
    assert_eq!(
        dispatch(Mode::SessionList, key(KeyCode::Char('t'))),
        KeyAction::Noop
    );
}

#[test]
fn esc_closes_help_and_preview_overlays() {
    for mode in [Mode::Help, Mode::Preview] {
        assert_eq!(
            dispatch(mode, key(KeyCode::Esc)),
            KeyAction::CloseOverlay,
            "esc should close in {mode:?}"
        );
    }
}

#[test]
fn unknown_keys_are_noop_in_breakdown() {
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::Char('x'))),
        KeyAction::Noop
    );
    assert_eq!(
        dispatch(Mode::Breakdown, key(KeyCode::F(5))),
        KeyAction::Noop
    );
}
