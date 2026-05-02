use std::path::{Path, PathBuf};

use crate::domain::Session;
use crate::ports::token_estimator::TokenEstimator;

/// Adapter port for one coding agent (Codex, Claude Code, …).
///
/// Implementations live under `src/adapters/<agent>/` and must keep
/// agent-specific paths / schema / tags out of the domain layer
/// (ARCHITECTURE.md §7).
pub trait AgentAdapter {
    fn name(&self) -> &'static str;

    /// Enumerate session artefacts the adapter knows how to read on this
    /// machine. May return an empty Vec if nothing is installed.
    fn discover(&self) -> anyhow::Result<Vec<PathBuf>>;

    /// Parse one artefact into the normalised `Session` model. Token counts
    /// are produced via the supplied `TokenEstimator`.
    fn parse_file(&self, path: &Path, estimator: &dyn TokenEstimator) -> anyhow::Result<Session>;
}
