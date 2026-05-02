//! JSON / JSONL helpers shared across adapters and the preview usecase.

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Read a single line from a JSONL file. `line_1based` is **1-based** (the
/// first physical line of the file is `1`), matching `SourceRef.line` in the
/// domain model.
///
/// Empty lines are **not** skipped — line numbers map to raw file lines as
/// produced by `BufRead::lines` (same as the Codex parser).
pub fn read_jsonl_line(path: &Path, line_1based: usize) -> io::Result<Option<String>> {
    if line_1based == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "line_1based must be >= 1",
        ));
    }
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for (idx, raw_line) in reader.lines().enumerate() {
        let n = idx + 1;
        if n == line_1based {
            return Ok(Some(raw_line?));
        }
    }
    Ok(None)
}
