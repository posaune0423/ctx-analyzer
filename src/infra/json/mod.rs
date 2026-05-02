//! JSON / JSONL IO helpers. Currently the Codex parser uses `serde_json`
//! directly via `BufReader::lines`; this module is reserved for shared
//! helpers (e.g. typed line-iter, pretty-printer) once a second JSONL
//! source lands.
