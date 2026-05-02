use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use anyhow::Context;

use crate::domain::session::ParseWarning;

use super::raw::{Envelope, RolloutRecord};

pub struct ParsedLine {
    pub line: usize,
    pub envelope: Envelope,
}

pub fn read_jsonl(path: &Path) -> anyhow::Result<(Vec<ParsedLine>, Vec<ParseWarning>)> {
    let file = File::open(path)
        .with_context(|| format!("failed to open rollout file: {}", path.display()))?;
    read_from_reader(BufReader::new(file))
}

pub fn read_from_reader<R: Read>(
    reader: BufReader<R>,
) -> anyhow::Result<(Vec<ParsedLine>, Vec<ParseWarning>)> {
    let mut envelopes = Vec::new();
    let mut warnings = Vec::new();
    for (idx, raw_line) in reader.lines().enumerate() {
        let line_no = idx + 1;
        let line = match raw_line {
            Ok(l) => l,
            Err(e) => {
                warnings.push(ParseWarning {
                    line: line_no,
                    message: format!("io error: {e}"),
                });
                continue;
            }
        };
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str::<RolloutRecord>(&line) {
            Ok(record) => envelopes.push(ParsedLine {
                line: line_no,
                envelope: record.into_envelope(),
            }),
            Err(e) => warnings.push(ParseWarning {
                line: line_no,
                message: format!("json parse error: {e}"),
            }),
        }
    }
    Ok((envelopes, warnings))
}
