use std::path::PathBuf;

use serde::Serialize;

/// Sub-classification of a context segment within its `ContextCategory`.
///
/// Agent adapters map their own raw event/tag taxonomy onto these. Add new
/// values when a new agent introduces a category not covered here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextSourceKind {
    // Instructions & Rules
    BaseInstructions,
    UserInstructions,
    ProjectInstructions,
    Rules,
    // Capabilities
    PermissionsInstructions,
    AppsInstructions,
    SkillsInstructions,
    PluginsInstructions,
    McpInstructions,
    // Runtime
    UserPrompt,
    AssistantMessage,
    FunctionCall,
    FunctionCallOutput,
    Reasoning,
    // Unknown
    EncryptedReasoning,
}

impl ContextSourceKind {
    pub fn label(self) -> &'static str {
        match self {
            ContextSourceKind::BaseInstructions => "system prompt",
            ContextSourceKind::UserInstructions => "user instructions",
            ContextSourceKind::ProjectInstructions => "project instructions",
            ContextSourceKind::Rules => "rules",
            ContextSourceKind::PermissionsInstructions => "permissions",
            ContextSourceKind::AppsInstructions => "apps",
            ContextSourceKind::SkillsInstructions => "skills",
            ContextSourceKind::PluginsInstructions => "plugins",
            ContextSourceKind::McpInstructions => "mcp",
            ContextSourceKind::UserPrompt => "user prompt",
            ContextSourceKind::AssistantMessage => "assistant message",
            ContextSourceKind::FunctionCall => "tool call",
            ContextSourceKind::FunctionCallOutput => "tool output",
            ContextSourceKind::Reasoning => "reasoning",
            ContextSourceKind::EncryptedReasoning => "encrypted reasoning",
        }
    }
}

/// Provenance pointer back to the on-disk artefact that produced a segment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SourceRef {
    pub file: PathBuf,
    /// 1-based line number for line-oriented sources (e.g. JSONL rollouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,
}
