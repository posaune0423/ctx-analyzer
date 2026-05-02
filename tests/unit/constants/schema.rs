//! JSON-export schema constants — bumping `SCHEMA_VERSION` here is the
//! one-line release operation when the export shape changes.

use ctx_analyzer::constants::schema::{AGENT_KEY_CODEX, SCHEMA_VERSION};

#[test]
fn schema_version_is_pinned() {
    assert_eq!(SCHEMA_VERSION, "0.1");
}

#[test]
fn agent_key_for_codex_is_lowercase_snake() {
    assert_eq!(AGENT_KEY_CODEX, "codex");
}
