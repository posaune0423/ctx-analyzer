//! Terminal UI for `ctx-analyzer inspect` (UI.md).

pub mod app;
pub mod components;
pub mod event;
pub mod keymap;
pub mod open_editor;
pub mod overlays;
pub mod render;
pub mod theme;

use std::path::{Path, PathBuf};

use crate::application::usecases::{analyze_workspace, estimate_tokens, list_workspace_sessions};

/// Entry point invoked by `inspect::run`.
pub fn run_inspect(
    file: Option<&Path>,
    session: Option<&str>,
    theme_arg: Option<&str>,
) -> anyhow::Result<()> {
    let estimator = estimate_tokens::default_estimator();
    let workspace_root = std::env::current_dir().ok();
    let launch_dir = workspace_root.clone().unwrap_or_else(|| PathBuf::from("."));
    let theme = theme::loader::resolve(theme::loader::ThemeOptions {
        cli: theme_arg,
        workspace_root: workspace_root.as_deref(),
    });

    let sessions = list_workspace_sessions::list_for_launch_dir(&launch_dir).unwrap_or_default();
    let direct = file.is_some() || session.is_some();

    let mut app = if direct {
        let parsed = analyze_workspace::run(file, session, &estimator)?;
        let active_idx = sessions
            .iter()
            .position(|s| s.path == parsed.source_path)
            .unwrap_or(0);
        app::AppState::new_with_loaded_session(
            parsed,
            sessions,
            active_idx,
            theme,
            true,
            launch_dir.clone(),
        )
    } else {
        app::AppState::new_session_picker(sessions, theme, launch_dir)
    };

    event::run(&mut app)
}
