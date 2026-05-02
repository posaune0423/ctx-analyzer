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
    project: Option<&Path>,
    agent: Option<&str>,
    session: Option<&str>,
    theme_arg: Option<&str>,
) -> anyhow::Result<()> {
    let estimator = estimate_tokens::default_estimator();
    let workspace_root = project
        .map(|p| p.to_path_buf())
        .or_else(|| std::env::current_dir().ok());
    let launch_dir = workspace_root.clone().unwrap_or_else(|| PathBuf::from("."));
    let theme = theme::loader::resolve(theme::loader::ThemeOptions {
        cli: theme_arg,
        workspace_root: workspace_root.as_deref(),
    });

    let mut sessions =
        list_workspace_sessions::list_for_launch_dir(&launch_dir).unwrap_or_default();

    if let Some(agent_str) = agent {
        let target = agent_str.to_lowercase();
        sessions
            .retain(|s| target == "codex" && matches!(s.agent, crate::domain::AgentKind::Codex));
    }

    // If a specific session ID was provided, open it directly.
    // Otherwise, start at the Session List picker.
    let mut app = if session.is_some() && !sessions.is_empty() {
        let session_path =
            crate::application::usecases::discover_sessions::resolve_path(project, agent, session)?;
        let parsed = analyze_workspace::run(&session_path, &estimator)?;
        let active_idx = sessions
            .iter()
            .position(|s| s.path == parsed.source_path)
            .unwrap_or(0);
        app::AppState::new_with_loaded_session(
            parsed,
            sessions,
            active_idx,
            theme,
            true, // launched_with_path = true means 'back' from turn list might quit or go back to session list depending on preference, but here we explicitly requested a session.
            launch_dir.clone(),
        )
    } else {
        app::AppState::new_session_picker(sessions, theme, launch_dir)
    };

    event::run(&mut app)
}
