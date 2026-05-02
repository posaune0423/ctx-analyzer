//! Theme resolution (UI.md §12).
//!
//! Order of precedence:
//! 1. CLI option (`--theme <name-or-path>`)
//! 2. Project theme: `<workspace>/.ctx-analyzer/theme.toml`
//! 3. User theme: `~/.config/ctx-analyzer/theme.toml`
//! 4. Built-in `default`
//!
//! Any I/O or parse failure is non-fatal: the loader returns the built-in
//! theme with `load_error` populated so the UI can show a `WarningModal`
//! once (UI.md §24).

use std::path::{Path, PathBuf};

use super::Theme;

#[derive(Debug, Clone, Default)]
pub struct ThemeOptions<'a> {
    /// `--theme` argument verbatim. Either a built-in theme name or a
    /// path to a custom `.toml`.
    pub cli: Option<&'a str>,
    /// Project root used to resolve `.ctx-analyzer/theme.toml`.
    pub workspace_root: Option<&'a Path>,
}

pub fn resolve(opts: ThemeOptions<'_>) -> Theme {
    // 1. CLI override.
    if let Some(arg) = opts.cli {
        if let Some(theme) = try_named(arg) {
            return theme;
        }
        return load_path(Path::new(arg))
            .unwrap_or_else(|err| fallback(format!("--theme {arg}: {err}")));
    }
    // 2. Project theme.
    if let Some(root) = opts.workspace_root {
        let path = root.join(".ctx-analyzer").join("theme.toml");
        if path.exists() {
            return load_path(&path)
                .unwrap_or_else(|err| fallback(format!("{}: {err}", path.display())));
        }
    }
    // 3. User theme.
    if let Some(user) = user_theme_path() {
        if user.exists() {
            return load_path(&user)
                .unwrap_or_else(|err| fallback(format!("{}: {err}", user.display())));
        }
    }
    // 4. Built-in.
    Theme::builtin()
}

fn try_named(name: &str) -> Option<Theme> {
    match name {
        "default" => Some(Theme::builtin()),
        _ => None,
    }
}

fn load_path(path: &Path) -> Result<Theme, String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    Theme::parse(&text)
}

fn user_theme_path() -> Option<PathBuf> {
    dirs::config_dir().map(|c| c.join("ctx-analyzer").join("theme.toml"))
}

fn fallback(message: String) -> Theme {
    let mut theme = Theme::builtin();
    theme.load_error = Some(message);
    theme
}
