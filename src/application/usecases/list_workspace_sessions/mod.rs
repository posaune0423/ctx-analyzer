//! Enumerate session artefacts available for the workspace.
//!
//! Used by the Session List stage (`docs/development/wireframe/session-list.md`).
//! MVP only knows Codex rollouts; a future multi-adapter version will dispatch via
//! `detect_agents`.

use std::path::PathBuf;
use std::time::SystemTime;

use crate::adapters::codex::discovery;
use crate::constants::codex::CODEX_SESSION_INDEX_FILE;
use crate::domain::AgentKind;

#[derive(Debug, Clone)]
pub struct SessionListItem {
    pub agent: AgentKind,
    pub path: PathBuf,
    pub modified_at: Option<SystemTime>,
    /// Best-effort “created” time (`Metadata::created` when the platform
    /// supports it; otherwise `modified`).
    pub created_at: Option<SystemTime>,
    /// First-line `session_meta.cwd` when parseable.
    pub cwd: Option<PathBuf>,
    /// Latest `thread_name` from `session_index.jsonl` for this rollout's
    /// `session_meta.id`, when present.
    pub conversation: Option<String>,
    /// Best-effort short id derived from the file name (`rollout-<id>.jsonl`).
    pub short_id: String,
}

/// Return Codex rollouts known to the local install, newest **modified** first.
pub fn list() -> anyhow::Result<Vec<SessionListItem>> {
    let home = discovery::codex_home();
    let index_path = home.join(CODEX_SESSION_INDEX_FILE);
    let names = discovery::load_thread_name_map(&index_path);

    let mut entries: Vec<SessionListItem> = discovery::list_rollouts()?
        .into_iter()
        .map(|path| {
            let modified_at = std::fs::metadata(&path).and_then(|m| m.modified()).ok();
            let created_at = std::fs::metadata(&path)
                .ok()
                .and_then(|m| m.created().ok().or_else(|| m.modified().ok()));
            let short_id = path
                .file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.trim_start_matches("rollout-").to_string())
                .unwrap_or_else(|| path.display().to_string());

            let (conversation, cwd) = match discovery::read_rollout_session_meta(&path) {
                Ok(Some((id, c))) => (names.get(&id).cloned(), c),
                _ => (None, None),
            };

            SessionListItem {
                agent: AgentKind::Codex,
                path,
                modified_at,
                created_at,
                cwd,
                conversation,
                short_id,
            }
        })
        .collect();
    entries.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
    Ok(entries)
}
