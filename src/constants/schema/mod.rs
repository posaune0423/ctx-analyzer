//! JSON-export schema constants (`docs/specs/export-json.md`).

/// Top-level `schema_version` field of every export. Bump when the
/// schema makes a breaking change.
pub const SCHEMA_VERSION: &str = "0.1";

/// `agent` field value emitted for the Codex adapter.
pub const AGENT_KEY_CODEX: &str = "codex";
