//! `SessionSourceReader` port — abstraction over an on-disk session
//! artefact (e.g. a Codex JSONL rollout, a Claude Code transcript directory).
//!
//! Reserved for post-MVP work; the MVP collapses discovery + parsing into
//! `AgentAdapter::discover()` + `AgentAdapter::parse_file()`. When SQLite
//! state-DB ingestion lands, a separate reader trait will keep file vs DB
//! IO behind one interface.

use std::path::PathBuf;

pub trait SessionSourceReader {
    fn list_artifacts(&self) -> anyhow::Result<Vec<PathBuf>>;
}
