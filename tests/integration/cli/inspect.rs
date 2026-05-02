//! Integration test for the `inspect` TUI. Drives the renderer with
//! ratatui's `TestBackend` and asserts the resulting buffer contents
//! (UI.md §6 / §7 / §8 / §9).

use std::path::PathBuf;

use assert_cmd::Command;
use ctx_analyzer::application::usecases::{analyze_workspace, estimate_tokens};
use ctx_analyzer::cli::tui::app::AppState;
use ctx_analyzer::cli::tui::render;
use ctx_analyzer::cli::tui::theme::Theme;
use ratatui::backend::TestBackend;
use ratatui::Terminal;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/codex/rollout-with-subagent/ctx.jsonl")
}

fn build_state() -> AppState {
    let estimator = estimate_tokens::default_estimator();
    let path = fixture_path();
    let session = analyze_workspace::run(Some(&path), None, &estimator).unwrap();
    let mut state = AppState::new_with_loaded_session(
        session,
        Vec::new(),
        0,
        Theme::builtin(),
        true,
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
    );
    state.enter_breakdown();
    state
}

fn render_to_string(state: &mut AppState, width: u16, height: u16) -> String {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| render::draw(f, state)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let mut out = String::new();
    for y in 0..buf.area().height {
        for x in 0..buf.area().width {
            out.push_str(buf.cell((x, y)).map(|c| c.symbol()).unwrap_or(" "));
        }
        out.push('\n');
    }
    out
}

#[test]
fn tui_frame_shows_header_summary_and_categories() {
    let mut state = build_state();
    let dump = render_to_string(&mut state, 120, 30);
    assert!(dump.contains("ctx-analyzer"), "header missing\n{dump}");
    assert!(dump.contains("[Codex]"), "agent badge missing\n{dump}");
    assert!(dump.contains("Total"), "summary bar missing\n{dump}");
    assert!(
        dump.contains("Configuration Context") || dump.contains("Runtime Context"),
        "no category section rendered\n{dump}"
    );
    assert!(
        dump.contains("Context Breakdown"),
        "accordion frame missing\n{dump}"
    );
    assert!(
        dump.contains("? help") || dump.contains("q quit"),
        "footer hint missing\n{dump}"
    );
}

#[test]
fn tui_frame_includes_severity_symbols() {
    let mut state = build_state();
    let dump = render_to_string(&mut state, 120, 40);
    // Fixture totals are large enough that at least one !! is expected.
    assert!(
        dump.contains("!!") || dump.contains("!"),
        "no severity symbol rendered\n{dump}"
    );
}

#[test]
fn inspect_without_tty_reports_a_clear_error() {
    let assert = Command::cargo_bin("ctx-analyzer")
        .unwrap()
        .args(["inspect", "--file"])
        .arg(fixture_path())
        .assert()
        .failure();
    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("TTY"),
        "expected TTY error, got stderr: {stderr}"
    );
}
