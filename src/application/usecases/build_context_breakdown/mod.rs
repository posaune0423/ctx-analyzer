//! Group a parsed `Session`'s segments into the
//! `view_models::context_breakdown::Breakdown` view-model
//! (UI.md §6 / §9).

use std::collections::BTreeMap;

use crate::application::view_models::context_breakdown::{
    Breakdown, CategorySection, SegmentRow, Severity,
};
use crate::domain::{ContextCategory, ContextSegment, Session};

pub fn group(session: &Session) -> Breakdown {
    group_filtered(session, |_| true)
}

/// Like `group`, but only keep segments whose `turn_id` matches one of
/// the given turn ids (used by `TurnSelector` to show "Turn N 時点" of
/// the breakdown — UI.md §17).
pub fn group_until_turn(session: &Session, turn_idx: usize) -> Breakdown {
    let allowed: std::collections::HashSet<&str> = session
        .turns
        .iter()
        .take(turn_idx + 1)
        .map(|t| t.id.as_str())
        .collect();
    group_filtered(session, |seg| match seg.turn_id.as_deref() {
        Some(id) => allowed.contains(id),
        None => true,
    })
}

/// Segments that are **new at** turn `turn_idx`: session-level rows
/// (`turn_id == None`) plus every segment whose `turn_id` equals the id of
/// `session.turns[turn_idx]` (`docs/development/wireframe/context-breakdown.md`,
/// delta mode).
pub fn group_delta_for_turn(session: &Session, turn_idx: usize) -> Breakdown {
    let Some(turn) = session.turns.get(turn_idx) else {
        return group_filtered(session, |_| false);
    };
    let tid = turn.id.as_str();
    group_filtered(session, |seg| match seg.turn_id.as_deref() {
        None => true,
        Some(id) => id == tid,
    })
}

fn group_filtered<F: Fn(&ContextSegment) -> bool>(session: &Session, keep: F) -> Breakdown {
    let mut by_cat: BTreeMap<ContextCategory, Vec<&ContextSegment>> = BTreeMap::new();
    for seg in &session.segments {
        if !keep(seg) {
            continue;
        }
        by_cat.entry(seg.category).or_default().push(seg);
    }
    let mut sections = Vec::new();
    let mut grand_total = 0u64;
    for category in ContextCategory::ordered() {
        let segs = by_cat.remove(&category).unwrap_or_default();
        let mut cat_total = 0u64;
        let mut rows = Vec::new();
        for seg in segs {
            cat_total += seg.tokens.tokens;
            rows.push(SegmentRow {
                segment_id: seg.id,
                label: seg.label.clone(),
                tokens: seg.tokens,
                severity: Severity::from_tokens(&seg.tokens),
                confidence: seg.confidence,
                source_ref: seg.source_ref.clone(),
                turn_id: seg.turn_id.clone(),
            });
        }

        grand_total += cat_total;
        sections.push(CategorySection {
            category,
            agent: session.agent,
            total_tokens: cat_total,
            rows,
        });
    }
    Breakdown {
        sections,
        total_tokens: grand_total,
    }
}
