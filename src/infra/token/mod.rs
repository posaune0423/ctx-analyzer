//! `TokenEstimator` implementations.
//!
//! `infra/` is appropriate (rather than `adapters/`) because token
//! estimation is not agent-specific — every adapter feeds text through
//! the same estimator.

use crate::domain::TokenEstimate;
use crate::ports::TokenEstimator;

/// Coarse heuristic: `ceil(text.chars().count() / 4)`.
///
/// Per `docs/specs/token-estimation.md` §3. Counts Unicode scalar values,
/// not bytes, so multi-byte text isn't double-counted. Confidence is
/// always `Estimated`.
#[derive(Debug, Default, Clone, Copy)]
pub struct CharsPer4Estimator;

impl TokenEstimator for CharsPer4Estimator {
    fn estimate(&self, text: &str) -> TokenEstimate {
        let chars = text.chars().count() as u64;
        let tokens = chars.div_ceil(4);
        TokenEstimate::estimated(tokens)
    }
}
