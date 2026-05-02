//! Codex-specific filesystem markers and inline text tags.
//!
//! Path layout source: `references/codex/codex-rs/rollout/src/recorder.rs`
//! and `session_index.rs`. See `docs/specs/codex.md`.

use crate::domain::ContextSourceKind;

/// Default subdirectory of the user's home dir when `$CODEX_HOME` is unset.
pub const CODEX_DEFAULT_HOME_DIR: &str = ".codex";

/// Environment variable that, when set, overrides the Codex home location.
pub const CODEX_HOME_ENV: &str = "CODEX_HOME";

/// Subdirectory under `<codex_home>/` containing per-day rollout JSONLs.
/// Codex stores them as `<codex_home>/sessions/YYYY/MM/DD/rollout-*.jsonl`.
pub const CODEX_SESSIONS_SUBDIR: &str = "sessions";

/// Filename prefix shared by every rollout file.
pub const CODEX_ROLLOUT_FILE_PREFIX: &str = "rollout-";

/// File extension shared by every rollout file.
pub const CODEX_ROLLOUT_FILE_EXT: &str = "jsonl";

/// Glob (relative to `<codex_home>/`) that finds every rollout file.
pub const CODEX_ROLLOUT_GLOB: &str = "sessions/**/rollout-*.jsonl";

/// Append-only thread-name index. Per-line shape:
/// `{"id": ThreadId, "thread_name": "...", "updated_at": "RFC3339"}`.
/// Resolution scans from EOF backwards; **last entry wins** per id.
/// Source: `references/codex/codex-rs/rollout/src/session_index.rs`.
pub const CODEX_SESSION_INDEX_FILE: &str = "session_index.jsonl";

/// SQLite cache (read-only from `ctx-analyzer`'s side) holding thread
/// metadata + backfill state. Performance optimisation, not a fallback.
/// Source: `references/codex/codex-rs/core/state_db.rs`.
pub const CODEX_STATE_DB_FILE: &str = "state.db";

/// Detected via `text.trim_start().starts_with(...)` on the first
/// `user`-role response_item message. Marks an inline AGENTS.md project
/// instruction block.
pub const PROJECT_DOC_PREFIX: &str = "# AGENTS.md instructions for ";

/// Tagged inline blocks appearing in the first `developer`-role
/// response_item message. Each block is split off as its own
/// Configuration segment by the classifier.
pub struct DeveloperBlockTag {
    pub open: &'static str,
    pub close: &'static str,
    pub kind: ContextSourceKind,
    pub label: &'static str,
}

pub const DEVELOPER_BLOCK_TAGS: &[DeveloperBlockTag] = &[
    DeveloperBlockTag {
        open: "<permissions instructions>",
        close: "</permissions instructions>",
        kind: ContextSourceKind::PermissionsInstructions,
        label: "Permissions Instructions",
    },
    DeveloperBlockTag {
        open: "<apps_instructions>",
        close: "</apps_instructions>",
        kind: ContextSourceKind::AppsInstructions,
        label: "Apps Instructions",
    },
    DeveloperBlockTag {
        open: "<skills_instructions>",
        close: "</skills_instructions>",
        kind: ContextSourceKind::SkillsInstructions,
        label: "Skills Instructions",
    },
    DeveloperBlockTag {
        open: "<plugins_instructions>",
        close: "</plugins_instructions>",
        kind: ContextSourceKind::PluginsInstructions,
        label: "Plugins Instructions",
    },
];
