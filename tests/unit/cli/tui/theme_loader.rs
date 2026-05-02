//! Theme loader (UI.md §12).
//!
//! Resolution order: CLI flag → project file → user file → built-in.
//! Invalid theme paths fall back to the built-in default and surface a
//! `load_error` diagnostic for the WarningModal (UI.md §24).

use std::fs;

use ctx_analyzer::cli::tui::theme::loader::{resolve, ThemeOptions};

#[test]
fn no_options_returns_builtin_with_no_error() {
    let theme = resolve(ThemeOptions::default());
    assert!(theme.load_error.is_none());
}

#[test]
fn cli_named_default_returns_builtin() {
    let theme = resolve(ThemeOptions {
        cli: Some("default"),
        ..Default::default()
    });
    assert!(theme.load_error.is_none());
}

#[test]
fn cli_invalid_path_falls_back_with_error() {
    let theme = resolve(ThemeOptions {
        cli: Some("/tmp/__ctx_analyzer_no_such_theme.toml"),
        ..Default::default()
    });
    assert!(theme.load_error.is_some(), "should record fallback reason");
}

#[test]
fn project_theme_file_is_loaded_when_present() {
    let dir = tempdir();
    let project = dir.join(".ctx-analyzer");
    fs::create_dir_all(&project).unwrap();
    fs::write(
        project.join("theme.toml"),
        ctx_analyzer::cli::tui::theme::builtin::DEFAULT_THEME_TOML,
    )
    .unwrap();
    let theme = resolve(ThemeOptions {
        cli: None,
        workspace_root: Some(&dir),
    });
    assert!(theme.load_error.is_none());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn invalid_project_theme_falls_back() {
    let dir = tempdir();
    let project = dir.join(".ctx-analyzer");
    fs::create_dir_all(&project).unwrap();
    fs::write(project.join("theme.toml"), b"this is = not [valid\ntoml").unwrap();
    let theme = resolve(ThemeOptions {
        cli: None,
        workspace_root: Some(&dir),
    });
    assert!(theme.load_error.is_some());
    let _ = fs::remove_dir_all(&dir);
}

fn tempdir() -> std::path::PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("ctx-analyzer-theme-test-{nanos}"));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}
