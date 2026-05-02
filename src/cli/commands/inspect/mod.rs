use std::io::{self, Write};
use std::path::Path;

use crate::application::usecases::{analyze_workspace, build_context_breakdown, estimate_tokens};
use crate::application::view_models::context_breakdown::text_render;

pub fn run(file: Option<&Path>, session: Option<&str>) -> anyhow::Result<()> {
    let estimator = estimate_tokens::default_estimator();
    let session = analyze_workspace::run(file, session, &estimator)?;
    let breakdown = build_context_breakdown::group(&session);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    text_render::render(&session, &breakdown, &mut handle)?;
    handle.flush()?;
    Ok(())
}
