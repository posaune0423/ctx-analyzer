//! Enumerate which coding-agent adapters are available on this host.
//!
//! MVP: Codex only. Once new adapters land they advertise themselves
//! here so `inspect`/`export` can switch on adapter name.

use crate::domain::AgentKind;

pub fn available() -> Vec<AgentKind> {
    vec![AgentKind::Codex]
}
