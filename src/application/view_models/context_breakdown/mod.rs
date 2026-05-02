//! Display-friendly view model for the context breakdown screen
//! (UI.md §6 / §9 / §10).

use crate::constants::severity::{
    SEVERITY_CRITICAL, SEVERITY_HIGH, SEVERITY_MEDIUM, SYMBOL_CRITICAL, SYMBOL_HIGH, SYMBOL_UNKNOWN,
};
use crate::domain::{Confidence, ContextCategory, ContextSourceKind, SourceRef, TokenEstimate};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
    Unknown,
}

impl Severity {
    pub fn symbol(self) -> &'static str {
        match self {
            Severity::Low | Severity::Medium => "",
            Severity::High => SYMBOL_HIGH,
            Severity::Critical => SYMBOL_CRITICAL,
            Severity::Unknown => SYMBOL_UNKNOWN,
        }
    }

    pub fn from_tokens(estimate: &TokenEstimate) -> Severity {
        if matches!(estimate.confidence, Confidence::Unknown) {
            return Severity::Unknown;
        }
        Severity::from_total(estimate.tokens)
    }

    pub fn from_total(tokens: u64) -> Severity {
        match tokens {
            t if t < SEVERITY_MEDIUM => Severity::Low,
            t if t < SEVERITY_HIGH => Severity::Medium,
            t if t < SEVERITY_CRITICAL => Severity::High,
            _ => Severity::Critical,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Breakdown {
    pub sections: Vec<CategorySection>,
    pub total_tokens: u64,
}

#[derive(Debug, Clone)]
pub struct CategorySection {
    pub category: ContextCategory,
    pub total_tokens: u64,
    pub groups: Vec<SourceGroup>,
}

#[derive(Debug, Clone)]
pub struct SourceGroup {
    pub kind: ContextSourceKind,
    pub total_tokens: u64,
    pub rows: Vec<SegmentRow>,
}

#[derive(Debug, Clone)]
pub struct SegmentRow {
    /// Index into `Session.segments` (`ContextSegment.id`).
    pub segment_id: usize,
    pub label: String,
    pub tokens: TokenEstimate,
    pub severity: Severity,
    pub confidence: Confidence,
    pub source_ref: SourceRef,
    pub turn_id: Option<String>,
}
