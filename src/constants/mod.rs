//! Application-wide constants. Single source of truth for thresholds,
//! schema versions, and adapter-specific filesystem markers.
//!
//! Layering rule: `constants/` may be referenced by any layer (domain,
//! application, adapters, infra, cli). It must not import from those
//! layers (no inbound dependencies); it only owns plain values.

pub mod codex;
pub mod preview;
pub mod schema;
pub mod severity;
