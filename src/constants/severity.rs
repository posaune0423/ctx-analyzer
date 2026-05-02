//! Severity thresholds for token counts (UI.md §10 / `docs/specs/token-estimation.md` §5).
//!
//! `tokens < SEVERITY_MEDIUM`        → Low     (no symbol)
//! `SEVERITY_MEDIUM..SEVERITY_HIGH`  → Medium  (no symbol)
//! `SEVERITY_HIGH..SEVERITY_CRITICAL`→ High    (`!`)
//! `tokens >= SEVERITY_CRITICAL`     → Critical(`!!`)

pub const SEVERITY_MEDIUM: u64 = 1_000;
pub const SEVERITY_HIGH: u64 = 5_000;
pub const SEVERITY_CRITICAL: u64 = 15_000;

pub const SYMBOL_HIGH: &str = "!";
pub const SYMBOL_CRITICAL: &str = "!!";
pub const SYMBOL_UNKNOWN: &str = "?";
