//! External-dependency interfaces. Application code calls into adapters /
//! infra via these traits only; no direct imports of `adapters/`,
//! `infra/`, or third-party crates beyond `domain` (ARCHITECTURE.md §6).

pub mod agent_adapter;
pub mod context_source_reader;
pub mod editor_launcher;
pub mod exporter;
pub mod file_preview_reader;
pub mod session_source_reader;
pub mod token_estimator;

pub use agent_adapter::AgentAdapter;
pub use exporter::Exporter;
pub use token_estimator::TokenEstimator;
