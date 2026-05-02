//! Preview Modal (`docs/development/wireframe/preview-modal.md`).

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap};

use crate::application::usecases::preview_segment::SegmentPreview;
use crate::cli::tui::app::{AppState, EditorTarget};

pub fn render(
    area: Rect,
    buf: &mut Buffer,
    state: &mut AppState,
    preview: &SegmentPreview,
    scroll: usize,
) {
    Clear.render(area, buf);

    let mut lines: Vec<Line<'_>> = Vec::new();
    if let Some(path) = &preview.source_path {
        state.register_click(
            Rect {
                x: area.x,
                y: area.y.saturating_add(1),
                width: area.width,
                height: 4,
            },
            EditorTarget {
                path: std::path::PathBuf::from(path),
                line: preview.source_line,
            },
        );
        let theme = &state.theme;
        let mut loc = match preview.source_line {
            Some(n) => format!("{path}:{n}"),
            None => path.clone(),
        };
        // Strip newlines to prevent unwanted line breaks (user reported)
        loc = loc.replace('\n', " ").replace('\r', "");

        // Truncate if path is extremely long to prevent wrapping from pushing body down
        let max_meta_width = area.width.saturating_sub(12) as usize;
        let display_loc = crate::utils::text::truncate_chars(&loc, max_meta_width);

        lines.push(Line::from(vec![
            Span::styled("Source: ", theme.text.muted),
            Span::styled(display_loc, theme.text.path),
        ]));
    } else {
        let theme = &state.theme;
        lines.push(Line::from(Span::styled(
            "This segment has no source file.",
            theme.text.subtitle,
        )));
    }
    let theme = &state.theme;
    let label = preview.label.replace('\n', " ").replace('\r', "");
    let max_label_width = area.width.saturating_sub(12) as usize;
    let display_label = crate::utils::text::truncate_chars(&label, max_label_width);

    lines.push(Line::from(vec![
        Span::styled("Label : ", theme.text.muted),
        Span::styled(display_label, theme.text.normal),
    ]));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "----------------------------------------",
        theme.text.muted,
    )));

    let body_lines: Vec<&str> = preview.body.lines().collect();
    let visible = area.height.saturating_sub(8) as usize;
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
        "↑↓ scroll · o open in editor · esc close",
        theme.text.muted,
    )));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border.focused)
        .title(Span::styled(" Preview ", theme.text.title));

    Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .block(block)
        .render(area, buf);
}
