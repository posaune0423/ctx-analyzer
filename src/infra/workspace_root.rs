//! Resolve the local project root used to scope `inspect` session lists.
//!
//! Uses a git work tree (walk upward for `.git`) when present; otherwise the
//! launch directory. Session membership uses path-tree overlap so
//! `session_meta.cwd` can be the repo root, a subfolder, or the parent of the
//! current working directory.

use std::path::{Path, PathBuf};

/// Directory that anchors `inspect` for this process (typically `env::current_dir()`).
///
/// Walks upward from `start` (if it is a file, starts from its parent) until a
/// `.git` entry exists, then canonicalizes that directory. If none is found,
/// canonicalizes `start` when it exists on disk, else returns `start` as given.
pub fn resolve_inspect_project_root(start: &Path) -> PathBuf {
    let anchor_dir: PathBuf = if start.is_file() {
        start
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| start.to_path_buf())
    } else {
        start.to_path_buf()
    };

    let mut dir = anchor_dir.clone();
    loop {
        if dir.join(".git").exists() {
            return std::fs::canonicalize(&dir).unwrap_or(dir);
        }
        if !dir.pop() {
            break;
        }
    }

    if anchor_dir.exists() {
        std::fs::canonicalize(&anchor_dir).unwrap_or(anchor_dir)
    } else {
        anchor_dir
    }
}

fn normalize_existing(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

/// True when `session_cwd` is the inspect project or lies on the same directory
/// branch as `project_root` (one is a prefix path of the other, component-wise).
///
/// `session_meta.cwd` from Codex may be the repository root while the user runs
/// `ctx-analyzer` from a subdirectory, or the reverse.
pub fn session_cwd_in_project(session_cwd: &Path, project_root: &Path) -> bool {
    let s = normalize_existing(session_cwd);
    let p = normalize_existing(project_root);
    s.strip_prefix(&p).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn session_in_project_when_session_is_repo_root_and_launch_is_subdir() {
        let base = std::env::temp_dir().join(format!("ctx-ws-root-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("pkg")).unwrap();
        fs::create_dir(base.join(".git")).unwrap();

        let proj = std::fs::canonicalize(&base).unwrap();
        let pkg = std::fs::canonicalize(base.join("pkg")).unwrap();

        let root = resolve_inspect_project_root(&pkg);
        assert_eq!(root, proj);
        assert!(session_cwd_in_project(&proj, &root));
        assert!(session_cwd_in_project(&pkg, &root));
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn same_branch_without_git_uses_canonical_launch() {
        let base = std::env::temp_dir().join(format!("ctx-ws-nogit-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("deep")).unwrap();
        let deep = std::fs::canonicalize(base.join("deep")).unwrap();
        let root = resolve_inspect_project_root(&deep);
        assert_eq!(root, deep);
        let parent = deep.parent().unwrap();
        assert!(!session_cwd_in_project(parent, &deep));
        assert!(session_cwd_in_project(&deep, parent));
        let _ = fs::remove_dir_all(&base);
    }
}
