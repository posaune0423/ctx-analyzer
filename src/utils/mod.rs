//! Generic helpers shared across layers (text truncation, number
//! formatting). Like `constants/`, this module has no inbound deps on
//! other layers — it must be importable from anywhere.

pub mod format;
pub mod text;
