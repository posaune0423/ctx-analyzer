use std::path::Path;

use crate::cli::tui;

pub fn run(
    project: Option<&Path>,
    agent: Option<&str>,
    session: Option<&str>,
    theme: Option<&str>,
) -> anyhow::Result<()> {
    tui::run_inspect(project, agent, session, theme)
}
