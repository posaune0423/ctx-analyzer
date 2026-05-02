//! Top-level analysis usecase: pick the right adapter, parse the
//! resolved file, return a normalised `Session`.

use std::path::Path;

use anyhow::Context;

use crate::adapters::codex::CodexAdapter;
use crate::application::usecases::discover_sessions;
use crate::domain::Session;
use crate::ports::{AgentAdapter, TokenEstimator};

pub fn run(
    file: Option<&Path>,
    session: Option<&str>,
    estimator: &dyn TokenEstimator,
) -> anyhow::Result<Session> {
    let path = discover_sessions::resolve_path(file, session)?;
    let adapter = CodexAdapter::new();
    adapter
        .parse_file(&path, estimator)
        .with_context(|| format!("failed to parse {}", path.display()))
}
