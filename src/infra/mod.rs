//! Concrete IO implementations. No agent-specific semantics live here —
//! adapter logic stays in `src/adapters/<agent>/` (ARCHITECTURE.md §8).

pub mod editor;
pub mod fs;
pub mod json;
pub mod sqlite;
pub mod token;
pub mod toml;

pub use token::CharsPer4Estimator;
