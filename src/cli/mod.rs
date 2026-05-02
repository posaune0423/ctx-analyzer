//! CLI bootstrap and command routing (ARCHITECTURE.md §9).

pub mod commands;

use commands::{Cli, Commands};

/// Dispatch a parsed `Cli` to its subcommand. Returns `Err` if any
/// subcommand failed; `main` maps that to exit code 1.
pub fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.cmd {
        Commands::Inspect { file, session } => {
            commands::inspect::run(file.as_deref(), session.as_deref())
        }
        Commands::Export { file, session, out } => {
            commands::export::run(file.as_deref(), session.as_deref(), out.as_deref())
        }
        Commands::Doctor { file } => commands::doctor::run(file.as_deref()),
    }
}
