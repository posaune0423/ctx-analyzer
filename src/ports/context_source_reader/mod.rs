//! `ContextSourceReader` port — abstraction over instruction / configuration
//! / capability sources living *outside* the session artefact (e.g.
//! `~/.codex/config.toml`, `AGENTS.md`, MCP definition files).
//!
//! Reserved for post-MVP work; the MVP recovers these from inline blocks
//! inside the rollout itself (see `docs/specs/codex.md` §4 classifier).

use std::path::PathBuf;

pub trait ContextSourceReader {
    fn discover(&self) -> anyhow::Result<Vec<PathBuf>>;
}
