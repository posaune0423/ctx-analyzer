use std::io::Write;

use crate::domain::Session;

use super::{Breakdown, Severity};

/// Render a `Breakdown` as the plain-text accordion used by the
/// `ctx-analyzer inspect` command (UI.md §6).
pub fn render(session: &Session, breakdown: &Breakdown, w: &mut dyn Write) -> std::io::Result<()> {
    writeln!(w, "ctx-analyzer · {}", session.agent.label())?;
    writeln!(
        w,
        "  session  : {}{}",
        session.id,
        session
            .source
            .subagent
            .as_deref()
            .map(|s| format!("  (subagent: {s})"))
            .unwrap_or_default()
    )?;
    if let Some(cwd) = &session.cwd {
        writeln!(w, "  cwd      : {}", cwd.display())?;
    }
    writeln!(w, "  source   : {}", session.source_path.display())?;
    writeln!(
        w,
        "  turns    : {}  segments: {}",
        session.turns.len(),
        session.segments.len()
    )?;
    if let Some(tot) = session.session_totals {
        writeln!(
            w,
            "  observed : total {} (input {} / cached {} / output {} / reasoning {}) [confidence: {:?}]",
            fmt_num(tot.tokens),
            opt_num(tot.input),
            opt_num(tot.cached_input),
            opt_num(tot.output),
            opt_num(tot.reasoning_output),
            tot.confidence,
        )?;
    }
    writeln!(
        w,
        "  estimated breakdown total: {}",
        fmt_num(breakdown.total_tokens)
    )?;
    if !session.warnings.is_empty() {
        writeln!(
            w,
            "  warnings : {} parse warning(s)",
            session.warnings.len()
        )?;
    }
    writeln!(w)?;

    for section in &breakdown.sections {
        if section.total_tokens == 0 && section.groups.is_empty() {
            continue;
        }
        let sev = Severity::from_total(section.total_tokens);
        writeln!(
            w,
            "▾ {} ~{}  {}",
            section.category.label(),
            fmt_num(section.total_tokens),
            sev.symbol()
        )?;
        for group in &section.groups {
            let gsev = Severity::from_total(group.total_tokens);
            writeln!(
                w,
                "  ▸ {}  ~{}  ({} segment{})  {}",
                group.kind.label(),
                fmt_num(group.total_tokens),
                group.rows.len(),
                if group.rows.len() == 1 { "" } else { "s" },
                gsev.symbol()
            )?;
            for row in &group.rows {
                writeln!(
                    w,
                    "      · {:<40}  ~{:>8}  {}",
                    truncate_label(&row.label, 40),
                    fmt_num(row.tokens.tokens),
                    row.severity.symbol(),
                )?;
            }
        }
        writeln!(w)?;
    }
    Ok(())
}

fn fmt_num(n: u64) -> String {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let mut out = String::new();
    let len = bytes.len();
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 && (len - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(*b as char);
    }
    out
}

fn opt_num(o: Option<u64>) -> String {
    o.map(fmt_num).unwrap_or_else(|| "-".to_string())
}

fn truncate_label(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if i + 1 >= max {
            out.push('…');
            break;
        }
        out.push(ch);
    }
    out
}
