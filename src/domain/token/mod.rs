use serde::Serialize;

use crate::domain::confidence::Confidence;

/// Token count estimate for a single context segment or aggregate.
///
/// `tokens` is always populated. The breakdown fields (`input` / `cached_input`
/// / `output` / `reasoning_output`) are only set when the agent's own
/// telemetry provided them (i.e. `confidence == Observed`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct TokenEstimate {
    pub tokens: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_input: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_output: Option<u64>,
    pub confidence: Confidence,
}

impl TokenEstimate {
    pub fn estimated(tokens: u64) -> Self {
        Self {
            tokens,
            input: None,
            cached_input: None,
            output: None,
            reasoning_output: None,
            confidence: Confidence::Estimated,
        }
    }

    pub fn unknown() -> Self {
        Self {
            tokens: 0,
            input: None,
            cached_input: None,
            output: None,
            reasoning_output: None,
            confidence: Confidence::Unknown,
        }
    }
}
