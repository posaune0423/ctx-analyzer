//! Enumerate Codex rollout sessions for the **local project** shown in `inspect`.
//!
//! Scoping: resolve a project root from the directory where the CLI was started
//! (git work tree when `.git` exists, else that directory), then keep rollouts
//! whose `session_meta.cwd` lies on the same directory branch (repo root vs
//! subfolder, etc.). MVP only lists Codex JSONL rollouts under `$CODEX_HOME`.

use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::adapters::codex::discovery;
use crate::domain::AgentKind;
use crate::infra::workspace_root;

#[derive(Debug, Clone)]
pub struct SessionListItem {
    pub agent: AgentKind,
    pub path: PathBuf,
    pub modified_at: Option<SystemTime>,
    /// Best-effort “created” time (`Metadata::created` when the platform
    /// supports it; otherwise `modified`).
    pub created_at: Option<SystemTime>,
    /// First user prompt in the rollout (file order), for the list preview column.
    pub first_prompt: Option<String>,
    /// Best-effort short id derived from the file name (`rollout-<id>.jsonl`).
    pub short_id: String,
}

/// Codex rollouts for this inspect project, newest **modified** first.
///
/// `launch_dir` is normally `std::env::current_dir()` when the user runs `inspect`.
pub fn list_for_launch_dir(launch_dir: &Path) -> anyhow::Result<Vec<SessionListItem>> {
    let project_root = workspace_root::resolve_inspect_project_root(launch_dir);
    let mut entries: Vec<SessionListItem> = Vec::new();
    for path in discovery::list_rollouts()? {
        let Some(scan) = discovery::scan_rollout_for_session_list(&path, &project_root)? else {
            continue;
        };
        let modified_at = std::fs::metadata(&path).and_then(|m| m.modified()).ok();
        let created_at = std::fs::metadata(&path)
            .ok()
            .and_then(|m| m.created().ok().or_else(|| m.modified().ok()));
        let short_id = path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.trim_start_matches("rollout-").to_string())
            .unwrap_or_else(|| path.display().to_string());

        entries.push(SessionListItem {
            agent: AgentKind::Codex,
            path,
            modified_at,
            created_at,
            first_prompt: scan.first_prompt,
            short_id,
        });
    }
    entries.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
    Ok(entries)
}
