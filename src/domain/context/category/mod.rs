use crate::domain::agent::AgentKind;
use serde::Serialize;

/// Granular context categories (PRD §12 refactored).
///
/// Agent-agnostic categories used for top-level grouping in the TUI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextCategory {
    SystemPrompt,
    ProjectDoc,
    Rules,
    Skills,
    Mcp,
    Apps,
    Plugins,
    UserPrompt,
    ToolCall,
    AssistantMessage,
    Unknown,
}

impl ContextCategory {
    pub fn label(self, agent: AgentKind) -> &'static str {
        match self {
            ContextCategory::SystemPrompt => "system prompt",
            ContextCategory::ProjectDoc => match agent {
                AgentKind::Codex => "AGENTS.md",
                AgentKind::ClaudeCode => "CLAUDE.md",
                AgentKind::Gemini => "GEMINI.md",
            },
            ContextCategory::Rules => "rules",
            ContextCategory::Skills => "skills",
            ContextCategory::Mcp => "mcp",
            ContextCategory::Apps => "apps",
            ContextCategory::Plugins => "plugins",
            ContextCategory::UserPrompt => "user prompt",
            ContextCategory::ToolCall => "tool call",
            ContextCategory::AssistantMessage => "assistant message",
            ContextCategory::Unknown => "unknown",
        }
    }

    pub fn ordered() -> [ContextCategory; 11] {
        [
            ContextCategory::SystemPrompt,
            ContextCategory::ProjectDoc,
            ContextCategory::Rules,
            ContextCategory::Skills,
            ContextCategory::Mcp,
            ContextCategory::Apps,
            ContextCategory::Plugins,
            ContextCategory::UserPrompt,
            ContextCategory::ToolCall,
            ContextCategory::AssistantMessage,
            ContextCategory::Unknown,
        ]
    }
}
