//! Diagnostic dump of one TUI frame against the bundled fixture.
//! Gated behind `CTX_TUI_DUMP=1` so it doesn't spam normal `cargo test`
//! output, but is available for visual regression / demo purposes.

use std::path::PathBuf;

use ctx_analyzer::application::usecases::{analyze_workspace, estimate_tokens};
use ctx_analyzer::cli::tui::app::AppState;
use ctx_analyzer::cli::tui::render;
use ctx_analyzer::cli::tui::theme::Theme;
use ratatui::backend::TestBackend;
use ratatui::Terminal;

#[test]
fn dump_frame_to_stdout() {
    if std::env::var("CTX_TUI_DUMP").ok().as_deref() != Some("1") {
        return;
    }
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/codex/rollout-with-subagent/ctx.jsonl");
    let estimator = estimate_tokens::default_estimator();
    let session = analyze_workspace::run(Some(&path), None, &estimator).unwrap();
    let mut state =
        AppState::new_with_loaded_session(session, Vec::new(), 0, Theme::builtin(), true);
    state.enter_breakdown();

    let backend = TestBackend::new(120, 36);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| render::draw(f, &mut state)).unwrap();
    let buf = terminal.backend().buffer().clone();

    println!("\n--- TUI frame (120x36) ---");
    for y in 0..buf.area().height {
        let mut line = String::new();
        for x in 0..buf.area().width {
            line.push_str(buf.cell((x, y)).map(|c| c.symbol()).unwrap_or(" "));
        }
        println!("{}", line.trim_end());
    }
    println!("--- end frame ---");
}
