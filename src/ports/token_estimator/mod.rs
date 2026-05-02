use crate::domain::TokenEstimate;

/// Pluggable token-count estimator. The default `CharsPer4Estimator`
/// (in `src/infra/token/`) gives a coarse `ceil(chars/4)` heuristic; a
/// `tiktoken`-backed implementation can be swapped in without touching
/// the application or adapter layers.
pub trait TokenEstimator {
    fn estimate(&self, text: &str) -> TokenEstimate;
}
