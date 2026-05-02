//! Codex adapter (initial agent — see `docs/specs/codex.md`).
//!
//! Submodules mirror ARCHITECTURE.md §7:
//! - `discovery/` — find rollout files on disk
//! - `raw/`       — serde-side mirror of the Codex JSONL schema
//! - `parsers/`   — JSONL → typed `Envelope` stream (lenient on errors)
//! - `classifiers/` — split tagged developer blocks; project-doc detection
//! - `mappers/`   — typed events → normalised domain `Session`
//! - `graph/`     — parent / child / forked session reconstruction (post-MVP)

pub mod classifiers;
pub mod discovery;
pub mod graph;
pub mod mappers;
pub mod parsers;
pub mod raw;
pub mod rollout_preview;

use std::path::{Path, PathBuf};

use crate::domain::Session;
use crate::ports::{AgentAdapter, TokenEstimator};

#[derive(Debug, Default, Clone, Copy)]
pub struct CodexAdapter;

impl CodexAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl AgentAdapter for CodexAdapter {
    fn name(&self) -> &'static str {
        "codex"
    }

    fn discover(&self) -> anyhow::Result<Vec<PathBuf>> {
        discovery::list_rollouts()
    }

    fn parse_file(&self, path: &Path, estimator: &dyn TokenEstimator) -> anyhow::Result<Session> {
        let (envelopes, warnings) = parsers::read_jsonl(path)?;
        Ok(mappers::map_to_session(
            path, envelopes, warnings, estimator,
        ))
    }
}
