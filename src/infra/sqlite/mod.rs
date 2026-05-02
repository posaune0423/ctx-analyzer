//! Read-only SQLite access. Reserved for `~/.codex/state.db` ingestion
//! (post-MVP). Per ARCHITECTURE.md §8: SQLite must be opened in
//! `SQLITE_OPEN_READONLY` mode — no writes to agent state.
