use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

use crate::application::usecases::{analyze_workspace, estimate_tokens, export_json};
use crate::ports::Exporter;

pub fn run(file: Option<&Path>, session: Option<&str>, out: Option<&Path>) -> anyhow::Result<()> {
    let estimator = estimate_tokens::default_estimator();
    let session = analyze_workspace::run(file, session, &estimator)?;
    let value = export_json::JsonExporter.export(&session);
    let serialized = serde_json::to_string_pretty(&value)?;
    match out {
        Some(p) => {
            let mut w = BufWriter::new(File::create(p)?);
            w.write_all(serialized.as_bytes())?;
            w.write_all(b"\n")?;
            w.flush()?;
        }
        None => {
            let stdout = io::stdout();
            let mut h = stdout.lock();
            h.write_all(serialized.as_bytes())?;
            h.write_all(b"\n")?;
            h.flush()?;
        }
    }
    Ok(())
}
