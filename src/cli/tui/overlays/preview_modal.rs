//! Preview Modal (`docs/development/wireframe/preview-modal.md`).

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap};

use crate::application::usecases::preview_segment::SegmentPreview;
use crate::cli::tui::app::{AppState, EditorTarget};
use crate::constants::preview::PREVIEW_MODAL_VISIBLE_BODY_LINES;

pub fn render(
    area: Rect,
    buf: &mut Buffer,
    state: &mut AppState,
    preview: &SegmentPreview,
    scroll: usize,
) {
    let inner = centered(
        area,
        area.width.saturating_sub(8),
        area.height.saturating_sub(4),
    );
    Clear.render(inner, buf);

    let mut lines: Vec<Line<'_>> = Vec::new();
    if let Some(path) = &preview.source_path {
        state.register_click(
            Rect {
                x: inner.x,
                y: inner.y.saturating_add(1),
                width: inner.width,
                height: 4,
            },
            EditorTarget {
                path: std::path::PathBuf::from(path),
                line: preview.source_line,
            },
        );
        let theme = &state.theme;
        let loc = match preview.source_line {
            Some(n) => format!("{path}:{n}"),
            None => path.clone(),
        };
        lines.push(Line::from(vec![
            Span::styled("Source: ", theme.text.muted),
            Span::styled(loc, theme.text.path),
            Span::styled("  [click to open]", theme.text.muted),
        ]));
    } else {
        let theme = &state.theme;
        lines.push(Line::from(Span::styled(
            "This segment has no source file.",
            theme.text.subtitle,
        )));
    }
    let theme = &state.theme;
    lines.push(Line::from(vec![
        Span::styled("Label : ", theme.text.muted),
        Span::styled(preview.label.clone(), theme.text.normal),
    ]));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "----------------------------------------",
        theme.text.muted,
    )));

    let body_lines: Vec<&str> = preview.body.lines().collect();
    let visible = PREVIEW_MODAL_VISIBLE_BODY_LINES;
    let max_scroll = body_lines
        .len()
        .saturating_sub(visible.min(body_lines.len().max(1)));
    let scroll = scroll.min(max_scroll);
    if preview.body.is_empty() {
        lines.push(Line::from(Span::styled(
            "Preview unavailable for this segment.",
            theme.text.subtitle,
        )));
    } else {
        for line in body_lines.iter().skip(scroll).take(visible.max(1)) {
            lines.push(Line::from(Span::styled(
                line.to_string(),
                theme.text.normal,
            )));
        }
    }
    if preview.truncated {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "(truncated — body exceeded preview char limit)",
            theme.border.warning,
        )));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "↑↓ scroll · o open in editor · click path · esc close",
        theme.text.muted,
    )));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border.focused)
        .title(Span::styled(" Preview ", theme.text.title));

    Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .block(block)
        .render(inner, buf);
}

fn centered(area: Rect, w: u16, h: u16) -> Rect {
    let w = w.min(area.width);
    let h = h.min(area.height);
    Rect {
        x: area.x + (area.width.saturating_sub(w)) / 2,
        y: area.y + (area.height.saturating_sub(h)) / 2,
        width: w,
        height: h,
    }
}
