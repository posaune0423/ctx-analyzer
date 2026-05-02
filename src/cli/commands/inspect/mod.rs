use std::path::Path;

use crate::cli::tui;

pub fn run(file: Option<&Path>, session: Option<&str>, theme: Option<&str>) -> anyhow::Result<()> {
    tui::run_inspect(file, session, theme)
}
