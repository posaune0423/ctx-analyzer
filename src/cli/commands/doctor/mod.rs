use std::path::Path;

use crate::adapters::codex::{discovery, CodexAdapter};
use crate::application::usecases::estimate_tokens;
use crate::ports::AgentAdapter;

pub fn run(project: Option<&Path>, agent: Option<&str>) -> anyhow::Result<()> {
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

    // Instead of taking a specific file, doctor can just resolve the latest session
    // for the project/agent to test the pipeline.
    let estimator = estimate_tokens::default_estimator();
    if let Ok(session_path) =
        crate::application::usecases::discover_sessions::resolve_path(project, agent, None)
    {
        let adapter = CodexAdapter::new();
        match adapter.parse_file(&session_path, &estimator) {
            Ok(session) => {
                println!();
                println!("  resolved target : {}", session_path.display());
                println!("    session id  : {}", session.id);
                println!("    turns       : {}", session.turns.len());
                println!("    segments    : {}", session.segments.len());
                println!("    warnings    : {}", session.warnings.len());
                for w in session.warnings.iter().take(5) {
                    println!("      line {}: {}", w.line, w.message);
                }
            }
            Err(e) => println!("  target parse error: {e:#}"),
        }
    }
    Ok(())
}
