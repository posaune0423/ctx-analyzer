//! Top-level analysis usecase: pick the right adapter, parse the
//! resolved file, return a normalised `Session`.

use std::path::Path;

use anyhow::Context;

use crate::adapters::codex::CodexAdapter;
use crate::domain::Session;
use crate::ports::{AgentAdapter, TokenEstimator};

pub fn run(path: &Path, estimator: &dyn TokenEstimator) -> anyhow::Result<Session> {
    let adapter = CodexAdapter::new();
    adapter
        .parse_file(path, estimator)
        .with_context(|| format!("failed to parse {}", path.display()))
}
