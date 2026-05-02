use std::process::ExitCode;

use clap::Parser;
use ctx_analyzer::cli::commands::Cli;

fn main() -> ExitCode {
    let cli = Cli::parse();
    match ctx_analyzer::cli::run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("ctx-analyzer: {err:#}");
            ExitCode::from(1)
        }
    }
}
