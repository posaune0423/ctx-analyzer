//! Locate Codex artefacts on disk. All path strings come from
//! `crate::constants::codex` so the spec, code, and tests share one
//! source of truth.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use anyhow::Context;
use serde::Deserialize;

use crate::adapters::codex::classifiers;
use crate::adapters::codex::raw::{ContentPart, Envelope, EventMsg, Payload, ResponseItem};
use crate::constants::codex::{
    CODEX_DEFAULT_HOME_DIR, CODEX_HOME_ENV, CODEX_ROLLOUT_GLOB, CODEX_SESSION_INDEX_FILE,
    CODEX_STATE_DB_FILE,
};
use crate::infra::json;
use crate::infra::workspace_root;

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
    let env: Envelope = serde_json::from_str(&line)
        .map_err(|e| anyhow::anyhow!("failed to parse rollout header: {e}"))?;
    match env.payload {
        Payload::SessionMeta(m) => Ok(Some((m.id, m.cwd.map(PathBuf::from)))),
        _ => Ok(None),
    }
}

/// Single-pass read: line 1 must be `session_meta` and `session_meta.cwd` must
/// fall under the same directory tree as `project_root` (see
/// [`workspace_root::session_cwd_in_project`]). Remaining lines are scanned for
/// the first user prompt (`response_item` role `user`, or `event_msg`
/// `user_message`).
///
/// Returns `Ok(None)` when meta is missing, cwd does not belong to the project, or
/// the first line is not `session_meta`.
pub fn scan_rollout_for_session_list(
    path: &Path,
    project_root: &Path,
) -> anyhow::Result<Option<RolloutSessionListScan>> {
    let file = File::open(path).with_context(|| format!("open {}", path.display()))?;
    let mut reader = BufReader::new(file);
    let mut first_line = String::new();
    if reader.read_line(&mut first_line)? == 0 {
        return Ok(None);
    }
    let env: Envelope = match serde_json::from_str(first_line.trim()) {
        Ok(e) => e,
        Err(_) => return Ok(None),
    };
    let Payload::SessionMeta(m) = env.payload else {
        return Ok(None);
    };
    let cwd = m.cwd.map(PathBuf::from);
    let Some(sc) = cwd.as_deref() else {
        return Ok(None);
    };
    if !workspace_root::session_cwd_in_project(sc, project_root) {
        return Ok(None);
    }
    let first_prompt = scan_reader_for_first_prompt(&mut reader)?;
    Ok(Some(RolloutSessionListScan { first_prompt }))
}

/// First user-visible prompt text for a session list row.
#[derive(Debug, Clone)]
pub struct RolloutSessionListScan {
    pub first_prompt: Option<String>,
}

fn scan_reader_for_first_prompt<R: BufRead>(reader: &mut R) -> anyhow::Result<Option<String>> {
    /// Skip pathological lines without parsing multi-megabyte JSON.
    const MAX_LINE_BYTES: usize = 16 * 1024 * 1024;
    for line in reader.lines() {
        let line = line?;
        if line.len() > MAX_LINE_BYTES {
            continue;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Ok(env) = serde_json::from_str::<Envelope>(trimmed) else {
            continue;
        };
        match env.payload {
            Payload::ResponseItem(ResponseItem::Message(msg)) if msg.role == "user" => {
                let combined = combine_content_parts(&msg.content);
                let t = combined.trim();
                if !t.is_empty() && !classifiers::looks_like_project_doc(&combined) {
                    return Ok(Some(combined));
                }
            }
            Payload::EventMsg(EventMsg::UserMessage(um)) => {
                if let Some(m) = um.message {
                    let t = m.trim();
                    if !t.is_empty() {
                        return Ok(Some(m));
                    }
                }
            }
            _ => {}
        }
    }
    Ok(None)
}

fn combine_content_parts(parts: &[ContentPart]) -> String {
    let mut out = String::new();
    for p in parts {
        if let Some(t) = &p.text {
            out.push_str(t);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn scan_rollout_returns_none_when_cwd_does_not_match_launch_dir() {
        let tmp = std::env::temp_dir().join(format!("ctx-scan-mismatch-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let other = std::env::temp_dir().join(format!("ctx-scan-other-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&other);
        std::fs::create_dir_all(&other).unwrap();

        let rollout = tmp.join("x.jsonl");
        let line1 = serde_json::json!({
            "type": "session_meta",
            "payload": {
                "id": "id",
                "cwd": other.to_str().unwrap().replace('\\', "/")
            }
        });
        let mut f = File::create(&rollout).unwrap();
        writeln!(f, "{}", line1).unwrap();
        drop(f);

        let got = scan_rollout_for_session_list(
            &rollout,
            &crate::infra::workspace_root::resolve_inspect_project_root(&tmp),
        )
        .unwrap();
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::remove_dir_all(&other);
        assert!(got.is_none());
    }
}
