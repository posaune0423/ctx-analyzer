//! `EditorLauncher` port — open a source file in the user's editor
//! (UI.md §19 keybinding `o`). Reserved for post-MVP TUI work.

use std::path::Path;

pub trait EditorLauncher {
    fn open(&self, path: &Path, line: Option<usize>) -> anyhow::Result<()>;
}
