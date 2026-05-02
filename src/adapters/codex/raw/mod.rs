//! Loose serde mirror of the Codex JSONL rollout schema.
//!
//! Unknown variants fall through to `Other` so future additions don't break parsing.

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Envelope {
    #[serde(default)]
    pub timestamp: Option<DateTime<Utc>>,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RolloutRecord {
    Envelope(Envelope),
    SessionMeta(SessionMeta),
}

impl RolloutRecord {
    pub fn into_envelope(self) -> Envelope {
        match self {
            Self::Envelope(env) => env,
            Self::SessionMeta(meta) => Envelope {
                timestamp: meta.timestamp,
                payload: Payload::SessionMeta(meta),
            },
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Payload {
    #[serde(rename = "session_meta")]
    SessionMeta(SessionMeta),
    #[serde(rename = "event_msg")]
    EventMsg(EventMsg),
    #[serde(rename = "response_item")]
    ResponseItem(ResponseItem),
    #[serde(rename = "compacted")]
    Compacted(Compacted),
    #[serde(rename = "turn_context")]
    TurnContext(TurnContext),
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
pub struct Compacted {
    #[serde(default)]
    pub replacement_history: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct SessionMeta {
    pub id: String,
    #[serde(default)]
    pub timestamp: Option<DateTime<Utc>>,
    #[serde(default)]
    pub cwd: Option<String>,
    #[serde(default)]
    pub originator: Option<String>,
    #[serde(default)]
    pub cli_version: Option<String>,
    #[serde(default)]
    pub source: Option<MetaSource>,
    #[serde(default)]
    pub model_provider: Option<String>,
    #[serde(default)]
    pub base_instructions: Option<BaseInstructions>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MetaSource {
    Structured {
        #[serde(default)]
        subagent: Option<String>,
    },
    Named(String),
}

impl MetaSource {
    pub fn delegated_subagent(&self) -> Option<&str> {
        match self {
            Self::Structured {
                subagent: Some(name),
                ..
            } => Some(name.as_str()),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BaseInstructions {
    #[serde(default)]
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum EventMsg {
    #[serde(rename = "task_started")]
    TaskStarted(TaskStarted),
    #[serde(rename = "task_complete")]
    TaskComplete(TaskComplete),
    #[serde(rename = "token_count")]
    TokenCount(TokenCount),
    #[serde(rename = "user_message")]
    UserMessage(UserMessage),
    #[serde(rename = "agent_message")]
    AgentMessage(AgentMessage),
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
pub struct TaskStarted {
    #[serde(default)]
    pub turn_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TaskComplete {
    #[serde(default)]
    pub turn_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TokenCount {
    #[serde(default)]
    pub info: Option<TokenInfo>,
}

#[derive(Debug, Deserialize)]
pub struct TokenInfo {
    #[serde(default)]
    pub total_token_usage: Option<TokenUsage>,
    #[serde(default)]
    pub last_token_usage: Option<TokenUsage>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct TokenUsage {
    #[serde(default)]
    pub input_tokens: Option<u64>,
    #[serde(default)]
    pub cached_input_tokens: Option<u64>,
    #[serde(default)]
    pub output_tokens: Option<u64>,
    #[serde(default)]
    pub reasoning_output_tokens: Option<u64>,
    #[serde(default)]
    pub total_tokens: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct UserMessage {
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AgentMessage {
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ResponseItem {
    #[serde(rename = "message")]
    Message(Message),
    #[serde(rename = "reasoning")]
    Reasoning(Reasoning),
    #[serde(rename = "function_call")]
    FunctionCall(FunctionCall),
    #[serde(rename = "function_call_output")]
    FunctionCallOutput(FunctionCallOutput),
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub role: String,
    #[serde(default)]
    pub content: Vec<ContentPart>,
}

#[derive(Debug, Deserialize)]
pub struct ContentPart {
    #[serde(rename = "type", default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Reasoning {
    #[serde(default)]
    pub encrypted_content: Option<String>,
    #[serde(default)]
    pub content: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct FunctionCall {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub arguments: Option<String>,
    #[serde(default)]
    pub call_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FunctionCallOutput {
    #[serde(default)]
    pub call_id: Option<String>,
    #[serde(default)]
    pub output: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TurnContext {
    #[serde(default)]
    pub turn_id: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub user_instructions: Option<String>,
}
