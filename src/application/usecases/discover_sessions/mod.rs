//! Locate session artefacts to analyse.
//!
//! The MVP only knows how to find Codex rollouts; once a second adapter
//! lands this module will consult `detect_agents` first.

use std::path::{Path, PathBuf};

use anyhow::anyhow;

use crate::adapters::codex::discovery;

/// Resolve which file `inspect` / `export` should operate on.
///
/// Resolution order:
/// 1. If `file` is supplied, use it as-is.
/// 2. Else list `~/.codex/sessions/**/rollout-*.jsonl`. If `session` is
///    given, keep only filenames containing that id substring.
/// 3. From the surviving set, pick the most-recently-modified file.
pub fn resolve_path(file: Option<&Path>, session: Option<&str>) -> anyhow::Result<PathBuf> {
    if let Some(p) = file {
        return Ok(p.to_path_buf());
    }
    let mut paths = discovery::list_rollouts()?;
    if let Some(sid) = session {
        paths.retain(|p| {
            p.file_name()
                .and_then(|f| f.to_str())
                .is_some_and(|name| name.contains(sid))
        });
    }
    if paths.is_empty() {
        return Err(anyhow!(
            "no Codex rollout files found (looked under {})",
            discovery::codex_home().display()
        ));
    }
    paths.sort_by_key(|p| std::fs::metadata(p).and_then(|m| m.modified()).ok());
    paths
        .pop()
        .ok_or_else(|| anyhow!("no rollout files matched"))
}
