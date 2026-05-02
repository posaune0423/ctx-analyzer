use std::path::PathBuf;

use serde::Serialize;

/// The root directory `ctx-analyzer` was invoked against (PRD §10.1).
///
/// In the MVP this is just a path wrapper; once `analyze_workspace` learns
/// to detect *which* agent owns a workspace, this will hold per-agent
/// `WorkspaceLocator`s as well.
#[derive(Debug, Clone, Serialize)]
pub struct Workspace {
    pub root: PathBuf,
}

impl Workspace {
    pub fn at(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
}
