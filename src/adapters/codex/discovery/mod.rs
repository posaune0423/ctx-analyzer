use std::path::PathBuf;

use anyhow::Context;

pub fn codex_home() -> PathBuf {
    if let Ok(home) = std::env::var("CODEX_HOME") {
        if !home.is_empty() {
            return PathBuf::from(home);
        }
    }
    dirs::home_dir()
        .map(|h| h.join(".codex"))
        .unwrap_or_else(|| PathBuf::from(".codex"))
}

pub fn list_rollouts() -> anyhow::Result<Vec<PathBuf>> {
    let home = codex_home();
    if !home.exists() {
        return Ok(Vec::new());
    }
    let pattern = home.join("sessions/**/rollout-*.jsonl");
    let pattern_str = pattern
        .to_str()
        .context("codex_home path is not valid UTF-8")?;
    let mut paths = Vec::new();
    for entry in glob::glob(pattern_str).context("invalid glob pattern")? {
        match entry {
            Ok(p) => paths.push(p),
            Err(_) => continue,
        }
    }
    Ok(paths)
}
