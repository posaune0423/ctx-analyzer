use serde::Serialize;

use crate::domain::confidence::Confidence;
use crate::domain::context::category::ContextCategory;
use crate::domain::context::source::{ContextSourceKind, SourceRef};
use crate::domain::token::TokenEstimate;

/// A single, token-measurable, source-traceable piece of context.
///
/// PRD §10.8 / §13.1 #13–#17.
#[derive(Debug, Clone, Serialize)]
pub struct ContextSegment {
    pub id: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_id: Option<String>,
    pub category: ContextCategory,
    pub source_kind: ContextSourceKind,
    pub label: String,
    pub preview: String,
    pub full_len_chars: usize,
    pub source_ref: SourceRef,
    pub tokens: TokenEstimate,
    pub confidence: Confidence,
}
