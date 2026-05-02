//! Theme system for the TUI (UI.md §12-§14).
//!
//! Theme files use TOML and define a semantic palette plus styled tokens.
//! Components reference tokens by name (`token.high`, `context.runtime`,
//! `row.selected`, …) and never hard-code colours.

pub mod builtin;
pub mod loader;

use std::collections::BTreeMap;

use ratatui::style::{Color, Modifier, Style};
use serde::Deserialize;

use crate::application::view_models::context_breakdown::Severity;
use crate::domain::ContextCategory;

/// Resolved theme used by all TUI components.
#[derive(Debug, Clone)]
pub struct Theme {
    pub palette: Palette,
    pub text: TextStyles,
    pub tokens: TokenStyles,
    pub contexts: ContextStyles,
    pub row: RowStyles,
    pub border: BorderStyles,
    /// Diagnostic for the user when a custom theme failed to load
    /// (UI.md §24). `None` when the active theme loaded cleanly.
    pub load_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Palette {
    pub foreground: Color,
    pub muted: Color,
    pub accent: Color,
    pub success: Color,
    pub warning: Color,
    pub critical: Color,
    pub unknown: Color,
    pub border: Color,
    pub selection_fg: Color,
    pub selection_bg: Color,
}

#[derive(Debug, Clone)]
pub struct TextStyles {
    pub normal: Style,
    pub muted: Style,
    pub title: Style,
    pub subtitle: Style,
    pub path: Style,
    pub badge: Style,
}

#[derive(Debug, Clone)]
pub struct StyledSymbol {
    pub style: Style,
    pub symbol: &'static str,
}

#[derive(Debug, Clone)]
pub struct TokenStyles {
    pub low: StyledSymbol,
    pub medium: StyledSymbol,
    pub high: StyledSymbol,
    pub critical: StyledSymbol,
    pub unknown: StyledSymbol,
}

#[derive(Debug, Clone)]
pub struct ContextStyles {
    pub system_prompt: Style,
    pub project_doc: Style,
    pub rules: Style,
    pub skills: Style,
    pub mcp: Style,
    pub apps: Style,
    pub plugins: Style,
    pub user_prompt: Style,
    pub tool_call: Style,
    pub assistant_message: Style,
    pub unknown: Style,
}

#[derive(Debug, Clone)]
pub struct RowStyles {
    pub selected: Style,
    pub expanded: Style,
    pub collapsed: Style,
}

#[derive(Debug, Clone)]
pub struct BorderStyles {
    pub normal: Style,
    pub focused: Style,
    pub warning: Style,
    pub critical: Style,
}

impl Theme {
    /// Theme used when no override is supplied.
    pub fn builtin() -> Theme {
        Theme::parse(builtin::DEFAULT_THEME_TOML).expect("builtin theme is valid")
    }

    /// Resolve the severity → (`Style`, symbol) used in the accordion
    /// (UI.md §10).
    pub fn token_for(&self, severity: Severity) -> &StyledSymbol {
        match severity {
            Severity::Low => &self.tokens.low,
            Severity::Medium => &self.tokens.medium,
            Severity::High => &self.tokens.high,
            Severity::Critical => &self.tokens.critical,
            Severity::Unknown => &self.tokens.unknown,
        }
    }

    pub fn context_style(&self, cat: ContextCategory) -> Style {
        match cat {
            ContextCategory::SystemPrompt => self.contexts.system_prompt,
            ContextCategory::ProjectDoc => self.contexts.project_doc,
            ContextCategory::Rules => self.contexts.rules,
            ContextCategory::Skills => self.contexts.skills,
            ContextCategory::Mcp => self.contexts.mcp,
            ContextCategory::Apps => self.contexts.apps,
            ContextCategory::Plugins => self.contexts.plugins,
            ContextCategory::UserPrompt => self.contexts.user_prompt,
            ContextCategory::ToolCall => self.contexts.tool_call,
            ContextCategory::AssistantMessage => self.contexts.assistant_message,
            ContextCategory::Unknown => self.contexts.unknown,
        }
    }

    /// Parse a TOML theme; on any error fall back to `builtin()` and
    /// stash the diagnostic in `load_error` (UI.md §24).
    pub fn parse(input: &str) -> Result<Theme, String> {
        let raw: RawTheme = toml::from_str(input).map_err(|e| e.to_string())?;
        raw.resolve()
    }
}

// ---------------------------------------------------------------------------
// TOML schema
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct RawTheme {
    palette: BTreeMap<String, String>,
    #[serde(default)]
    text: BTreeMap<String, String>,
    #[serde(default)]
    token: BTreeMap<String, RawStyledSymbol>,
    #[serde(default)]
    context: BTreeMap<String, RawStyledSymbol>,
    #[serde(default)]
    row: BTreeMap<String, String>,
    #[serde(default)]
    border: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct RawStyledSymbol {
    style: String,
    #[serde(default)]
    symbol: Option<String>,
}

impl RawTheme {
    fn resolve(self) -> Result<Theme, String> {
        let palette = resolve_palette(&self.palette)?;
        let text = resolve_text(&self.text, &palette)?;
        let tokens = resolve_tokens(&self.token, &palette)?;
        let contexts = resolve_contexts(&self.context, &palette)?;
        let row = resolve_row(&self.row, &palette)?;
        let border = resolve_border(&self.border, &palette)?;
        Ok(Theme {
            palette,
            text,
            tokens,
            contexts,
            row,
            border,
            load_error: None,
        })
    }
}

fn resolve_palette(raw: &BTreeMap<String, String>) -> Result<Palette, String> {
    let pick = |k: &str| -> Result<Color, String> {
        raw.get(k)
            .ok_or_else(|| format!("palette.{k} missing"))
            .and_then(|v| parse_color(v))
    };
    Ok(Palette {
        foreground: pick("foreground")?,
        muted: pick("muted")?,
        accent: pick("accent")?,
        success: pick("success").unwrap_or(Color::Green),
        warning: pick("warning")?,
        critical: pick("critical")?,
        unknown: pick("unknown")?,
        border: pick("border")?,
        selection_fg: pick("selection_fg")?,
        selection_bg: pick("selection_bg")?,
    })
}

fn resolve_text(raw: &BTreeMap<String, String>, palette: &Palette) -> Result<TextStyles, String> {
    let s = |k: &str| -> Style {
        raw.get(k)
            .map(|v| parse_style(v, palette))
            .unwrap_or_default()
    };
    Ok(TextStyles {
        normal: s("normal"),
        muted: s("muted"),
        title: s("title"),
        subtitle: s("subtitle"),
        path: s("path"),
        badge: s("badge"),
    })
}

fn resolve_tokens(
    raw: &BTreeMap<String, RawStyledSymbol>,
    palette: &Palette,
) -> Result<TokenStyles, String> {
    let s = |k: &str, default_symbol: &'static str| -> StyledSymbol {
        match raw.get(k) {
            Some(rs) => StyledSymbol {
                style: parse_style(&rs.style, palette),
                symbol: leak_symbol(rs.symbol.as_deref(), default_symbol),
            },
            None => StyledSymbol {
                style: Style::default(),
                symbol: default_symbol,
            },
        }
    };
    Ok(TokenStyles {
        low: s("low", ""),
        medium: s("medium", ""),
        high: s("high", "!"),
        critical: s("critical", "!!"),
        unknown: s("unknown", "?"),
    })
}

fn resolve_contexts(
    raw: &BTreeMap<String, RawStyledSymbol>,
    palette: &Palette,
) -> Result<ContextStyles, String> {
    let s = |k: &str| -> Style {
        raw.get(k)
            .map(|rs| parse_style(&rs.style, palette))
            .unwrap_or_default()
    };
    Ok(ContextStyles {
        system_prompt: s("system_prompt"),
        project_doc: s("project_doc"),
        rules: s("rules"),
        skills: s("skills"),
        mcp: s("mcp"),
        apps: s("apps"),
        plugins: s("plugins"),
        user_prompt: s("user_prompt"),
        tool_call: s("tool_call"),
        assistant_message: s("assistant_message"),
        unknown: s("unknown"),
    })
}

fn resolve_row(raw: &BTreeMap<String, String>, palette: &Palette) -> Result<RowStyles, String> {
    let s = |k: &str| -> Style {
        raw.get(k)
            .map(|v| parse_style(v, palette))
            .unwrap_or_default()
    };
    Ok(RowStyles {
        selected: s("selected"),
        expanded: s("expanded"),
        collapsed: s("collapsed"),
    })
}

fn resolve_border(
    raw: &BTreeMap<String, String>,
    palette: &Palette,
) -> Result<BorderStyles, String> {
    let s = |k: &str| -> Style {
        raw.get(k)
            .map(|v| parse_style(v, palette))
            .unwrap_or_default()
    };
    Ok(BorderStyles {
        normal: s("normal"),
        focused: s("focused"),
        warning: s("warning"),
        critical: s("critical"),
    })
}

fn leak_symbol(custom: Option<&str>, default: &'static str) -> &'static str {
    match custom {
        Some(s) => Box::leak(s.to_string().into_boxed_str()),
        None => default,
    }
}

// ---------------------------------------------------------------------------
// Color / Style parsing
// ---------------------------------------------------------------------------

fn parse_color(spec: &str) -> Result<Color, String> {
    Ok(match spec.trim() {
        "default" | "reset" => Color::Reset,
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "gray" | "grey" | "white" => Color::Gray,
        "dark_gray" | "dark_grey" => Color::DarkGray,
        "light_red" => Color::LightRed,
        "light_green" => Color::LightGreen,
        "light_yellow" => Color::LightYellow,
        "light_blue" => Color::LightBlue,
        "light_magenta" => Color::LightMagenta,
        "light_cyan" => Color::LightCyan,
        other => return Err(format!("unknown color: {other}")),
    })
}

fn parse_style(spec: &str, palette: &Palette) -> Style {
    let mut style = Style::default();
    for token in spec.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        match token {
            "bold" => style = style.add_modifier(Modifier::BOLD),
            "dim" => style = style.add_modifier(Modifier::DIM),
            "italic" => style = style.add_modifier(Modifier::ITALIC),
            "underline" => style = style.add_modifier(Modifier::UNDERLINED),
            "reversed" => style = style.add_modifier(Modifier::REVERSED),
            color => {
                if let Some(c) = palette_lookup(palette, color) {
                    style = if style.fg.is_some() {
                        style.bg(c)
                    } else {
                        style.fg(c)
                    };
                } else if let Ok(c) = parse_color(color) {
                    style = if style.fg.is_some() {
                        style.bg(c)
                    } else {
                        style.fg(c)
                    };
                }
            }
        }
    }
    style
}

fn palette_lookup(p: &Palette, name: &str) -> Option<Color> {
    Some(match name {
        "foreground" => p.foreground,
        "muted" => p.muted,
        "accent" => p.accent,
        "success" => p.success,
        "warning" => p.warning,
        "critical" => p.critical,
        "unknown" => p.unknown,
        "border" => p.border,
        "selection_fg" => p.selection_fg,
        "selection_bg" => p.selection_bg,
        _ => return None,
    })
}
