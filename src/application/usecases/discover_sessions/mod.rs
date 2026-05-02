//! Locate session artefacts to analyse.
//!
//! The MVP only knows how to find Codex rollouts; once a second adapter
//! lands this module will consult `detect_agents` first.

use std::path::{Path, PathBuf};

/// Resolve which file `inspect` / `export` should operate on.
///
/// Resolution order:
/// 1. If `file` is supplied, use it as-is.
/// 2. Else list `~/.codex/sessions/**/rollout-*.jsonl`. If `session` is
///    given, keep only filenames containing that id substring.
/// 3. From the surviving set, pick the most-recently-modified file.
pub fn resolve_path(
    project: Option<&Path>,
    agent: Option<&str>,
    session: Option<&str>,
) -> anyhow::Result<PathBuf> {
    let launch_dir = project
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    let mut sessions =
        crate::application::usecases::list_workspace_sessions::list_for_launch_dir(&launch_dir)?;

    if let Some(agent_str) = agent {
        let target = agent_str.to_lowercase();
        // In the future, match against AgentKind variants. For MVP, we only have Codex.
        if target != "codex" {
            return Err(anyhow::anyhow!("unsupported agent: {}", agent_str));
        }
        sessions.retain(|s| matches!(s.agent, crate::domain::AgentKind::Codex));
    }

    if let Some(sid) = session {
        let target = sid.to_lowercase();
        sessions.retain(|s| {
            s.short_id.to_lowercase().contains(&target)
                || s.session_id.to_lowercase().contains(&target)
                || s.path.to_string_lossy().to_lowercase().contains(&target)
        });
    }

    if sessions.is_empty() {
        return match session {
            Some(sid) => Err(anyhow::anyhow!(
                "no rollout files matched --session {}",
                sid
            )),
            None => Err(anyhow::anyhow!(
                "no agent sessions found for project (looked in {})",
                launch_dir.display()
            )),
        };
    }

    // list_for_launch_dir already sorts newest first.
    let target_session = sessions.first().expect("sessions is not empty").clone();
    Ok(target_session.path)
}
