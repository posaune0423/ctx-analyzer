//! Locate Codex artefacts on disk. All path strings come from
//! `crate::constants::codex` so the spec, code, and tests share one
//! source of truth.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Context;
use serde::Deserialize;

use crate::constants::codex::{
    CODEX_DEFAULT_HOME_DIR, CODEX_HOME_ENV, CODEX_ROLLOUT_GLOB, CODEX_SESSION_INDEX_FILE,
    CODEX_STATE_DB_FILE,
};
use crate::infra::json;

/// Resolve `$CODEX_HOME`, falling back to `~/.codex` and finally to
/// `./.codex` (last resort, for sandboxed environments without `$HOME`).
pub fn codex_home() -> PathBuf {
    if let Ok(home) = std::env::var(CODEX_HOME_ENV) {
        if !home.is_empty() {
            return PathBuf::from(home);
        }
    }
    dirs::home_dir()
        .map(|h| h.join(CODEX_DEFAULT_HOME_DIR))
        .unwrap_or_else(|| PathBuf::from(CODEX_DEFAULT_HOME_DIR))
}

/// Path to the append-only thread-name index. Caller is responsible for
/// the actual scan (post-MVP — `docs/specs/codex.md` §1).
pub fn session_index_path() -> PathBuf {
    codex_home().join(CODEX_SESSION_INDEX_FILE)
}

/// Path to the SQLite metadata cache. Caller is responsible for opening
/// in `SQLITE_OPEN_READONLY` mode (post-MVP).
pub fn state_db_path() -> PathBuf {
    codex_home().join(CODEX_STATE_DB_FILE)
}

/// All rollout files the local Codex install has produced. Empty Vec
/// when the home directory does not exist.
pub fn list_rollouts() -> anyhow::Result<Vec<PathBuf>> {
    let home = codex_home();
    if !home.exists() {
        return Ok(Vec::new());
    }
    let pattern = home.join(CODEX_ROLLOUT_GLOB);
    let pattern_str = pattern
        .to_str()
        .context("codex_home path is not valid UTF-8")?;
    let mut paths = Vec::new();
    for p in glob::glob(pattern_str)
        .context("invalid glob pattern")?
        .flatten()
    {
        paths.push(p);
    }
    Ok(paths)
}

#[derive(Debug, Deserialize)]
struct SessionIndexEntry {
    id: String,
    thread_name: String,
}

/// Map `session_meta.id` → latest `thread_name` from Codex's append-only
/// `session_index.jsonl`. Forward scan: later lines win (same as re-reading
/// the file and inserting in order).
pub fn load_thread_name_map(path: &Path) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let Ok(text) = std::fs::read_to_string(path) else {
        return map;
    };
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Ok(entry) = serde_json::from_str::<SessionIndexEntry>(trimmed) else {
            continue;
        };
        map.insert(entry.id, entry.thread_name);
    }
    map
}

/// Read the first JSONL record of a rollout and return `(session_id, cwd)` when
/// it is a `session_meta` line.
pub fn read_rollout_session_meta(path: &Path) -> anyhow::Result<Option<(String, Option<PathBuf>)>> {
    let Some(line) = json::read_jsonl_line(path, 1)? else {
        return Ok(None);
    };
    let env: crate::adapters::codex::raw::Envelope = serde_json::from_str(&line)
        .map_err(|e| anyhow::anyhow!("failed to parse rollout header: {e}"))?;
    match env.payload {
        crate::adapters::codex::raw::Payload::SessionMeta(m) => {
            Ok(Some((m.id, m.cwd.map(PathBuf::from))))
        }
        _ => Ok(None),
    }
}
