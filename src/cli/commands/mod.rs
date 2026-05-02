use std::path::PathBuf;

use clap::{Parser, Subcommand};

pub mod compare;
pub mod doctor;
pub mod export;
pub mod inspect;

#[derive(Debug, Parser)]
#[command(
    name = "ctx-analyzer",
    version,
    about = "Local-first context analyzer for coding agents (Codex MVP)"
)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Open the interactive TUI for a Codex session (UI.md).
    Inspect {
        #[arg(long)]
        file: Option<PathBuf>,
        #[arg(long)]
        session: Option<String>,
        /// Theme name (`default`) or path to a custom theme.toml.
        #[arg(long)]
        theme: Option<String>,
    },
    /// Export the session as agent-agnostic JSON (schema_version 0.1).
    Export {
        #[arg(long)]
        file: Option<PathBuf>,
        #[arg(long)]
        session: Option<String>,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Report environment / discovered rollouts; optionally validate a
    /// specific file.
    Doctor {
        #[arg(long)]
        file: Option<PathBuf>,
    },
}
