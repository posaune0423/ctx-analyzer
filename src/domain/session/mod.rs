use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::domain::agent::AgentKind;
use crate::domain::context::ContextSegment;
use crate::domain::session_graph::SessionGraph;
use crate::domain::token::TokenEstimate;
use crate::domain::turn::Turn;

/// One coding-agent session (PRD §10.3).
#[derive(Debug, Clone, Serialize)]
pub struct Session {
    pub id: String,
    pub agent: AgentKind,
    pub started_at: Option<DateTime<Utc>>,
    pub cwd: Option<PathBuf>,
    pub source_path: PathBuf,
    pub source: SessionSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cli_version: Option<String>,
    pub turns: Vec<Turn>,
    pub segments: Vec<ContextSegment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_totals: Option<TokenEstimate>,
    pub graph: SessionGraph,
    pub warnings: Vec<ParseWarning>,
}

/// Provenance metadata about how this session was launched. Set when the
/// agent itself records "this is a subagent" (Codex's `source.subagent`).
#[derive(Debug, Clone, Default, Serialize)]
pub struct SessionSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subagent: Option<String>,
}

/// Non-fatal parse error encountered while ingesting a session artefact.
/// Surfaced via `Session.warnings` so the UI can show partial results
/// (PRD §13.2).
#[derive(Debug, Clone, Serialize)]
pub struct ParseWarning {
    pub line: usize,
    pub message: String,
}
