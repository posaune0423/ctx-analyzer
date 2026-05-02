use serde::Serialize;

/// Coding-agent kind. The MVP only ships the Codex variant; new variants
/// are added as new adapters are wired in (Claude Code, Cursor, Gemini CLI,
/// OpenCode — see PRD §10.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentKind {
    Codex,
}

impl AgentKind {
    pub fn label(self) -> &'static str {
        match self {
            AgentKind::Codex => "Codex",
        }
    }
}
