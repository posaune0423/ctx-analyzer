//! Agent-agnostic core model. No `domain/` module may depend on
//! `adapters/`, `infra/`, `cli/`, `tui/`, or any other layer
//! (ARCHITECTURE.md §4).

pub mod agent;
pub mod confidence;
pub mod context;
pub mod session;
pub mod session_graph;
pub mod token;
pub mod turn;
pub mod workspace;

pub use agent::AgentKind;
pub use confidence::Confidence;
pub use context::{ContextCategory, ContextSegment, ContextSourceKind, SourceRef};
pub use session::{ParseWarning, Session, SessionSource};
pub use session_graph::SessionGraph;
pub use token::TokenEstimate;
pub use turn::Turn;
pub use workspace::Workspace;
