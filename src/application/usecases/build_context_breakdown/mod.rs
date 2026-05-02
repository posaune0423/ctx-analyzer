//! Group a parsed `Session`'s segments into the
//! `view_models::context_breakdown::Breakdown` view-model
//! (UI.md §6 / §9).

use std::collections::BTreeMap;

use crate::application::view_models::context_breakdown::{
    Breakdown, CategorySection, SegmentRow, Severity, SourceGroup,
};
use crate::domain::{ContextCategory, ContextSegment, ContextSourceKind, Session};

pub fn group(session: &Session) -> Breakdown {
    let mut by_cat: BTreeMap<ContextCategory, BTreeMap<ContextSourceKind, Vec<&ContextSegment>>> =
        BTreeMap::new();
    for seg in &session.segments {
        by_cat
            .entry(seg.category)
            .or_default()
            .entry(seg.source_kind)
            .or_default()
            .push(seg);
    }
    let mut sections = Vec::new();
    let mut grand_total = 0u64;
    for category in ContextCategory::ordered() {
        let groups_map = by_cat.remove(&category).unwrap_or_default();
        let mut groups = Vec::new();
        let mut cat_total = 0u64;
        for (kind, segs) in groups_map {
            let mut group_total = 0u64;
            let mut rows = Vec::new();
            for seg in segs {
                group_total += seg.tokens.tokens;
                rows.push(SegmentRow {
                    label: seg.label.clone(),
                    tokens: seg.tokens,
                    severity: Severity::from_tokens(&seg.tokens),
                    confidence: seg.confidence,
                    source_ref: seg.source_ref.clone(),
                    turn_id: seg.turn_id.clone(),
                });
            }
            cat_total += group_total;
            groups.push(SourceGroup {
                kind,
                total_tokens: group_total,
                rows,
            });
        }
        groups.sort_by(|a, b| b.total_tokens.cmp(&a.total_tokens));
        grand_total += cat_total;
        sections.push(CategorySection {
            category,
            total_tokens: cat_total,
            groups,
        });
    }
    Breakdown {
        sections,
        total_tokens: grand_total,
    }
}
