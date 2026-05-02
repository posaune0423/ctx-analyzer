use serde::Serialize;

/// Reconstruction confidence per PRD §7.2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Confidence {
    /// Provided directly by the agent's own telemetry / event stream.
    Observed,
    /// Derived from a heuristic (e.g. `ceil(chars/4)` token estimation).
    Estimated,
    /// Not visible (e.g. encrypted reasoning blob); count is unreliable.
    Unknown,
}
