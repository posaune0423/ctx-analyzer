use std::path::Path;

use crate::adapters::codex::{discovery, CodexAdapter};
use crate::application::usecases::estimate_tokens;
use crate::ports::AgentAdapter;

pub fn run(file: Option<&Path>) -> anyhow::Result<()> {
    let home = discovery::codex_home();
    println!("ctx-analyzer doctor");
    println!("  codex home  : {}", home.display());
    println!("  exists      : {}", home.exists());
    let rollouts = discovery::list_rollouts().unwrap_or_default();
    println!("  rollouts    : {} file(s)", rollouts.len());
    for p in rollouts.iter().take(5) {
        println!("    - {}", p.display());
    }
    if rollouts.len() > 5 {
        println!("    … {} more", rollouts.len() - 5);
    }
    if let Some(p) = file {
        let adapter = CodexAdapter::new();
        let estimator = estimate_tokens::default_estimator();
        match adapter.parse_file(p, &estimator) {
            Ok(session) => {
                println!();
                println!("  --file      : {}", p.display());
                println!("    session id  : {}", session.id);
                println!("    turns       : {}", session.turns.len());
                println!("    segments    : {}", session.segments.len());
                println!("    warnings    : {}", session.warnings.len());
                for w in session.warnings.iter().take(5) {
                    println!("      line {}: {}", w.line, w.message);
                }
            }
            Err(e) => println!("  --file parse error: {e:#}"),
        }
    }
    Ok(())
}
