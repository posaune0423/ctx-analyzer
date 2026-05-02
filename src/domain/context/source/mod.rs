use std::path::PathBuf;

use serde::Serialize;

/// Sub-classification of a context segment within its `ContextCategory`.
///
/// Agent adapters map their own raw event/tag taxonomy onto these. Add new
/// values when a new agent introduces a category not covered here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextSourceKind {
    // Configuration
    BaseInstructions,
    UserInstructions,
    PermissionsInstructions,
    AppsInstructions,
    SkillsInstructions,
    PluginsInstructions,
    ProjectInstructions,
    // Runtime
    UserPrompt,
    AssistantMessage,
    FunctionCall,
    FunctionCallOutput,
    Reasoning,
    // Delegated
    SubagentMarker,
    // Unknown
    EncryptedReasoning,
}

impl ContextSourceKind {
    pub fn label(self) -> &'static str {
        match self {
            ContextSourceKind::BaseInstructions => "Base Instructions",
            ContextSourceKind::UserInstructions => "User Instructions",
            ContextSourceKind::PermissionsInstructions => "Permissions Instructions",
            ContextSourceKind::AppsInstructions => "Apps Instructions",
            ContextSourceKind::SkillsInstructions => "Skills Instructions",
            ContextSourceKind::PluginsInstructions => "Plugins Instructions",
            ContextSourceKind::ProjectInstructions => "Project Instructions",
            ContextSourceKind::UserPrompt => "User Prompts",
            ContextSourceKind::AssistantMessage => "Assistant Messages",
            ContextSourceKind::FunctionCall => "Function Calls",
            ContextSourceKind::FunctionCallOutput => "Function Call Outputs",
            ContextSourceKind::Reasoning => "Reasoning",
            ContextSourceKind::SubagentMarker => "Subagent Marker",
            ContextSourceKind::EncryptedReasoning => "Encrypted Reasoning",
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
