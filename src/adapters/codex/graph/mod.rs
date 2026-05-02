//! Codex session graph reconstruction.
//!
//! MVP: the mapper sets `SessionGraph.is_delegated_child` based on
//! `session_meta.source.subagent`. Cross-file linking (matching a parent
//! rollout's `function_call` / `task delegation` to a child rollout file
//! in another path) is post-MVP — see `docs/specs/session-graph.md` §4.
