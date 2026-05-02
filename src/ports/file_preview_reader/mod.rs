//! `FilePreviewReader` port — bounded read of a file for the segment
//! preview pane (UI.md §18). Reserved for post-MVP TUI work.

use std::path::Path;

pub trait FilePreviewReader {
    fn read_preview(&self, path: &Path, max_bytes: usize) -> anyhow::Result<String>;
}
