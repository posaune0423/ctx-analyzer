//! Default token estimator selection. The CLI / TUI obtains an estimator
//! through this usecase so a future swap (e.g. tiktoken) only changes
//! one place.

use crate::infra::CharsPer4Estimator;
use crate::ports::TokenEstimator;

pub fn default_estimator() -> impl TokenEstimator {
    CharsPer4Estimator
}
