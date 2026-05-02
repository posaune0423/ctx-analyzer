//! ctx-analyzer library entry point.
//!
//! The crate is binary-first; this `lib` exists so integration tests and
//! future TUI/IDE consumers can call into the application layer without
//! going through the CLI process boundary.

pub mod adapters;
pub mod application;
pub mod cli;
pub mod constants;
pub mod domain;
pub mod infra;
pub mod ports;
pub mod utils;
