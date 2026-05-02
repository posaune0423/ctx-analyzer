//! Editor launcher (UI.md §19).
//!
//! Resolution order:
//! 1. `$CTX_ANALYZER_EDITOR`
//! 2. `$VISUAL`
//! 3. `$EDITOR`
//! 4. `cursor` → `code` → `zed` → `nvim` → `vim`

use std::path::Path;
use std::process::Command;

const FALLBACK_EDITORS: &[&str] = &["cursor", "code", "zed", "nvim", "vim"];

pub fn launch(path: &Path, line: Option<usize>) -> anyhow::Result<()> {
    let editor = pick_editor()?;
    let mut cmd = Command::new(&editor.bin);
    for arg in &editor.args {
        cmd.arg(arg);
    }
    match line {
        Some(n) if matches!(editor.kind, EditorKind::PlusLine) => {
            cmd.arg(format!("+{n}"));
            cmd.arg(path.display().to_string());
        }
        Some(n) if matches!(editor.kind, EditorKind::ColonLine) => {
            cmd.arg(format!("{}:{n}", path.display()));
        }
        _ => {
            cmd.arg(path.display().to_string());
        }
    }
    cmd.status()?;
    Ok(())
}

struct ResolvedEditor {
    bin: String,
    args: Vec<String>,
    kind: EditorKind,
}

enum EditorKind {
    /// `vim +12 file.rs`
    PlusLine,
    /// `code --goto file.rs:12`
    ColonLine,
    Plain,
}

fn pick_editor() -> anyhow::Result<ResolvedEditor> {
    for env in ["CTX_ANALYZER_EDITOR", "VISUAL", "EDITOR"] {
        if let Ok(value) = std::env::var(env) {
            if !value.trim().is_empty() {
                return Ok(parse_command(&value));
            }
        }
    }
    for cand in FALLBACK_EDITORS {
        if which(cand) {
            return Ok(parse_command(cand));
        }
    }
    Err(anyhow::anyhow!(
        "no editor found; set $EDITOR or $CTX_ANALYZER_EDITOR"
    ))
}

fn parse_command(raw: &str) -> ResolvedEditor {
    let mut parts = raw.split_whitespace();
    let bin = parts.next().unwrap_or("").to_string();
    let mut args: Vec<String> = parts.map(str::to_string).collect();
    let kind = match bin.as_str() {
        "vim" | "nvim" | "vi" => EditorKind::PlusLine,
        "code" | "cursor" | "code-insiders" => {
            args.insert(0, "--goto".to_string());
            EditorKind::ColonLine
        }
        "zed" => EditorKind::ColonLine,
        _ => EditorKind::Plain,
    };
    ResolvedEditor { bin, args, kind }
}

fn which(bin: &str) -> bool {
    let path = std::env::var_os("PATH").unwrap_or_default();
    for entry in std::env::split_paths(&path) {
        let candidate = entry.join(bin);
        if candidate.is_file() {
            return true;
        }
    }
    false
}
