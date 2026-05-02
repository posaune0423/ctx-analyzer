//! CLI bootstrap and command routing (ARCHITECTURE.md §9).

pub mod commands;
pub mod tui;

use commands::{Cli, Commands};

/// Dispatch a parsed `Cli` to its subcommand. Returns `Err` if any
/// subcommand failed; `main` maps that to exit code 1.
pub fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.cmd {
        Commands::Inspect {
            project,
            agent,
            session,
            theme,
        } => commands::inspect::run(
            project.as_deref(),
            agent.as_deref(),
            session.as_deref(),
            theme.as_deref(),
        ),
        Commands::Export {
            project,
            agent,
            session,
            out,
        } => commands::export::run(
            project.as_deref(),
            agent.as_deref(),
            session.as_deref(),
            out.as_deref(),
        ),
        Commands::Doctor { project, agent } => {
            commands::doctor::run(project.as_deref(), agent.as_deref())
        }
    }
}
