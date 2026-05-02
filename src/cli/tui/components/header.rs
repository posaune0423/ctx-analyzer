//! Header — workspace path / agent / session id / turn / view mode
//! (`docs/development/wireframe/context-breakdown.md`).

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

use crate::cli::tui::app::{AppState, Stage, ViewFilter};

const PATH_MAX: usize = 40;

pub fn render(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    let theme = &state.theme;
    let Some(ld) = state.loaded.as_ref() else {
        return;
    };
    let cwd = ld
        .session
        .cwd
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| ld.source_path.display().to_string());

    let (turn_label, view_tag) = if state.stage == Stage::Breakdown {
        let t = ld.active_turn_idx + 1;
        let n = ld.session.turns.len().max(1);
        let v = match ld.view_filter {
            ViewFilter::Cumulative => "cumulative",
            ViewFilter::Delta => "delta",
        };
        (format!("Turn {t}/{n}"), format!("[view: {v}]"))
    } else {
        let n = ld.session.turns.len();
        (format!("{n} turns"), String::new())
    };

    let mut spans = vec![
        Span::styled("ctx-analyzer", theme.text.title),
        Span::raw("  "),
        Span::styled(truncate_middle(&cwd, PATH_MAX), theme.text.path),
        Span::raw("  "),
        Span::styled(format!("[{}]", ld.session.agent.label()), theme.text.badge),
        Span::raw("  "),
        Span::styled(
            format!("[{}]", short_id(&ld.session.id)),
            theme.text.subtitle,
        ),
        Span::raw("  "),
        Span::styled(format!("[{turn_label}]"), theme.text.badge),
    ];
    if !view_tag.is_empty() {
        spans.push(Span::raw("  "));
        spans.push(Span::styled(view_tag, theme.text.muted));
    }

    Paragraph::new(Line::from(spans)).render(area, buf);

    if !cwd.is_empty() {
        state.register_click(
            Rect {
                x: area.x,
                y: area.y,
                width: area.width,
                height: 1,
            },
            crate::cli::tui::app::EditorTarget {
                path: ld
                    .session
                    .cwd
                    .clone()
                    .unwrap_or_else(|| ld.source_path.clone()),
                line: None,
            },
        );
    }
}

fn short_id(s: &str) -> String {
    if s.chars().count() <= 12 {
        s.to_string()
    } else {
        let head: String = s.chars().take(11).collect();
        format!("{head}…")
    }
}

fn truncate_middle(text: &str, max: usize) -> String {
    if max == 0 {
        return String::new();
    }
    let chars: Vec<char> = text.chars().collect();
    if chars.len() <= max {
        return text.to_string();
    }
    if max == 1 {
        return "…".to_string();
    }
    let head = max / 2;
    let tail = max - head - 1;
    let head_s: String = chars.iter().take(head).collect();
    let tail_s: String = chars
        .iter()
        .rev()
        .take(tail)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    format!("{head_s}…{tail_s}")
}
