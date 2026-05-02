//! Header — structured metadata and project context.
//! Redesigned for modern TUI aesthetic with clear hierarchy and spacing.

use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Modifier;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

use crate::cli::tui::app::{AppState, Stage, ViewFilter};
use crate::utils::format::fmt_thousands;

pub fn render(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    let theme = &state.theme;

    // 1. Calculate the blocks
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(1), // Title line
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Project
            Constraint::Length(1), // Spacer
            Constraint::Length(3), // Session & Interaction (split)
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Stats
            Constraint::Length(1), // Bottom rule
        ])
        .split(area);

    // 2. Render App Title & Rule
    let title_spans = vec![
        Span::styled("  ctx-analyzer ", theme.text.title),
        Span::styled(format!("v{} ", env!("CARGO_PKG_VERSION")), theme.text.muted),
        Span::styled(
            "─".repeat(area.width.saturating_sub(20) as usize),
            theme.text.muted,
        ),
    ];
    Paragraph::new(Line::from(title_spans)).render(chunks[0], buf);

    // 3. Project Section
    let repo_path = crate::infra::workspace_root::resolve_inspect_project_root(&state.launch_dir);
    let mut cwd = repo_path.display().to_string();
    if let Some(ld) = state.loaded.as_ref() {
        if let Some(p) = &ld.session.cwd {
            cwd = p.display().to_string();
        }
    }
    render_labeled_line(chunks[2], buf, "PROJECT", &cwd, theme);

    // 4. Session & Interaction (Horizontal Split)
    let mid_chunks = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(chunks[4]);

    // Session Sub-block
    let mut agent_label = "Codex".to_string();
    let mut model_label = "—".to_string();
    let mut session_id = "—".to_string();
    if let Some(ld) = state.loaded.as_ref() {
        agent_label = ld.session.agent.label().to_string();
        if let Some(sub) = &ld.session.source.subagent {
            agent_label = format!("{agent_label} : {sub}");
        }
        model_label = ld
            .session
            .model_provider
            .clone()
            .unwrap_or_else(|| "—".to_string());
        session_id = ld.session.id.clone();
    }
    render_session_info(
        mid_chunks[0],
        buf,
        &agent_label,
        &model_label,
        &session_id,
        theme,
    );

    // Interaction Sub-block
    let mut turn_info = "—".to_string();
    let mut mode_info = "—".to_string();
    let mut prompt_preview = "—".to_string();
    if let Some(ld) = state.loaded.as_ref() {
        if state.stage == Stage::Breakdown {
            turn_info = format!("{} / {}", ld.active_turn_idx + 1, ld.session.turns.len());
            mode_info = match ld.view_filter {
                ViewFilter::Cumulative => "cumulative",
                ViewFilter::Delta => "scoped",
            }
            .to_string();
            prompt_preview = ld
                .turn_summaries
                .get(ld.active_turn_idx)
                .and_then(|s| s.prompt_preview.as_deref())
                .unwrap_or("(no user prompt)")
                .replace('\n', " ");
        }
    }
    render_interaction_info(
        mid_chunks[1],
        buf,
        &turn_info,
        &mode_info,
        &prompt_preview,
        theme,
    );

    // 5. Context Stats
    if let Some(ld) = state.loaded.as_ref() {
        let total = fmt_thousands(ld.breakdown.total_tokens);
        // Simplified config/runtime split for summary: sum up segments
        let mut config_total = 0u64;
        let mut runtime_total = 0u64;
        for sec in &ld.breakdown.sections {
            match sec.category {
                crate::domain::ContextCategory::UserPrompt
                | crate::domain::ContextCategory::ToolCall
                | crate::domain::ContextCategory::AssistantMessage => {
                    runtime_total += sec.total_tokens
                }
                _ => config_total += sec.total_tokens,
            }
        }
        let stats_content = format!(
            "Total ~{}  ( Config ~{}  Runtime ~{} )",
            total,
            fmt_thousands(config_total),
            fmt_thousands(runtime_total)
        );
        render_labeled_line(chunks[6], buf, "CONTEXT", &stats_content, theme);
    }

    // 6. Bottom Rule
    let bottom_rule = Span::styled(
        "  ".to_owned() + &"─".repeat(area.width.saturating_sub(4) as usize),
        theme.text.muted,
    );
    Paragraph::new(Line::from(bottom_rule)).render(chunks[7], buf);
}

fn render_labeled_line(
    area: Rect,
    buf: &mut Buffer,
    label: &str,
    value: &str,
    theme: &crate::cli::tui::theme::Theme,
) {
    let spans = vec![
        Span::raw("  "),
        Span::styled(
            format!("{:<10}", label),
            theme.text.subtitle.add_modifier(Modifier::BOLD),
        ),
        Span::styled(value, theme.text.normal.add_modifier(Modifier::BOLD)),
    ];
    Paragraph::new(Line::from(spans)).render(area, buf);
}

fn render_session_info(
    area: Rect,
    buf: &mut Buffer,
    agent: &str,
    model: &str,
    id: &str,
    theme: &crate::cli::tui::theme::Theme,
) {
    let label_style = theme.text.muted.add_modifier(Modifier::BOLD);
    let value_style = theme.text.normal.add_modifier(Modifier::BOLD);

    let lines = vec![
        Line::from(vec![
            Span::raw("  "),
            Span::styled("AGENT     ", label_style),
            Span::styled(agent, theme.text.badge),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("MODEL     ", label_style),
            Span::styled(model, value_style),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("SESSION   ", label_style),
            Span::styled(short_id(id), theme.text.muted),
        ]),
    ];
    Paragraph::new(lines).render(area, buf);
}

fn render_interaction_info(
    area: Rect,
    buf: &mut Buffer,
    turn: &str,
    mode: &str,
    prompt: &str,
    theme: &crate::cli::tui::theme::Theme,
) {
    let label_style = theme.text.muted.add_modifier(Modifier::BOLD);
    let value_style = theme.text.normal.add_modifier(Modifier::BOLD);

    let lines = vec![
        Line::from(vec![
            Span::styled("TURN    ", label_style),
            Span::styled(turn, value_style),
            Span::raw(" "),
            Span::styled(format!("({mode})"), theme.text.muted),
        ]),
        Line::from(vec![
            Span::styled("PROMPT  ", label_style),
            Span::styled(
                crate::utils::text::truncate_chars(prompt, 30),
                theme.text.normal,
            ),
        ]),
        Line::from(vec![Span::raw("")]), // Spacer
    ];
    Paragraph::new(lines).render(area, buf);
}

fn short_id(s: &str) -> String {
    if s.chars().count() <= 12 {
        s.to_string()
    } else {
        let head: String = s.chars().take(11).collect();
        format!("{head}…")
    }
}
