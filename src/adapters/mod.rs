//! Per-agent adapters. Each subdirectory implements the `AgentAdapter`
//! port defined in `src/ports/agent_adapter/`.

pub mod claude_code;
pub mod codex;
pub mod cursor;
pub mod gemini;
