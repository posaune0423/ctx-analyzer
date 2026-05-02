//! Cross-file session-graph reconstruction (post-MVP).
//!
//! In the MVP the per-rollout `SessionGraph` is built inline by the Codex
//! mapper (`is_delegated_child` from `source.subagent`). This usecase
//! will, in a later iteration, consume a *set* of rollouts and link
//! parent → child sessions across files.

use crate::domain::SessionGraph;

/// Placeholder: returns an isolated single-session graph unchanged.
pub fn link_children(graph: SessionGraph) -> SessionGraph {
    graph
}
