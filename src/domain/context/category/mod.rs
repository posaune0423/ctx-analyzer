use serde::Serialize;

/// Top-level context bucket per PRD §12.
///
/// Agent-agnostic: the same five values are used for every adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextCategory {
    System,
    Configuration,
    Runtime,
    Delegated,
    Unknown,
}

impl ContextCategory {
    pub fn label(self) -> &'static str {
        match self {
            ContextCategory::System => "System Context",
            ContextCategory::Configuration => "Configuration Context",
            ContextCategory::Runtime => "Runtime Context",
            ContextCategory::Delegated => "Delegated Context",
            ContextCategory::Unknown => "Unknown Context",
        }
    }

    pub fn ordered() -> [ContextCategory; 5] {
        [
            ContextCategory::System,
            ContextCategory::Configuration,
            ContextCategory::Runtime,
            ContextCategory::Delegated,
            ContextCategory::Unknown,
        ]
    }
}
