use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::domain::token::TokenEstimate;

/// One user-interaction unit within a session (PRD §10.6).
#[derive(Debug, Clone, Serialize)]
pub struct Turn {
    pub id: String,
    pub index: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_instructions: Option<String>,
    pub segment_ids: Vec<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_token_usage: Option<TokenEstimate>,
}
