//! Filesystem helpers (`fs/` per ARCHITECTURE.md §8).
//!
//! Kept agent-agnostic: read-only, no source mutation. Currently a thin
//! shim around `std::fs`; expanded as `analyze_workspace` grows.

use std::path::Path;

/// True when `path` exists and is a regular file readable by the process.
pub fn is_readable_file(path: &Path) -> bool {
    std::fs::metadata(path)
        .map(|m| m.is_file())
        .unwrap_or(false)
}
