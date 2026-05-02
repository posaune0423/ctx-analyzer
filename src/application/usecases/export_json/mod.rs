//! Agent-agnostic JSON export. Schema is documented in
//! `docs/specs/export-json.md`.

use serde_json::{json, Value};

use crate::constants::schema::{AGENT_KEY_CODEX, SCHEMA_VERSION};
use crate::domain::{ContextCategory, ContextSegment, Session, TokenEstimate};
use crate::ports::Exporter;

#[derive(Debug, Default, Clone, Copy)]
pub struct JsonExporter;

impl Exporter for JsonExporter {
    fn export(&self, session: &Session) -> Value {
        let mut categories = serde_json::Map::new();
        for category in ContextCategory::ordered() {
            let segs: Vec<&ContextSegment> = session
                .segments
                .iter()
                .filter(|s| s.category == category)
                .collect();
            let total: u64 = segs.iter().map(|s| s.tokens.tokens).sum();
            categories.insert(
                category_key(category).to_string(),
                json!({
                    "total_tokens": total,
                    "segments": segs,
                }),
            );
        }
        json!({
            "schema_version": SCHEMA_VERSION,
            "agent": AGENT_KEY_CODEX,
            "session": {
                "id": session.id,
                "started_at": session.started_at,
                "cwd": session.cwd.as_ref().map(|p| p.display().to_string()),
                "source": {
                    "path": session.source_path.display().to_string(),
                    "subagent": session.source.subagent,
                },
                "model_provider": session.model_provider,
                "cli_version": session.cli_version,
                "is_delegated_child": session.graph.is_delegated_child,
            },
            "totals": session.session_totals.map(totals_value),
            "categories": categories,
            "turns": session.turns,
            "warnings": session.warnings,
        })
    }
}

fn totals_value(t: TokenEstimate) -> Value {
    json!({
        "total_tokens": t.tokens,
        "input_tokens": t.input,
        "cached_input_tokens": t.cached_input,
        "output_tokens": t.output,
        "reasoning_output_tokens": t.reasoning_output,
        "confidence": t.confidence,
    })
}

fn category_key(c: ContextCategory) -> &'static str {
    match c {
        ContextCategory::System => "system",
        ContextCategory::Configuration => "configuration",
        ContextCategory::Runtime => "runtime",
        ContextCategory::Delegated => "delegated",
        ContextCategory::Unknown => "unknown",
    }
}
