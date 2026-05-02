//! Preview / label truncation limits used across adapters and the CLI
//! renderer.

/// Maximum number of Unicode scalar values kept in a `ContextSegment.preview`
/// string. Anything beyond is replaced with a single `…` ellipsis.
pub const PREVIEW_LIMIT_CHARS: usize = 200;

/// Maximum width (in chars) for row labels in the `inspect` accordion
/// before they get ellipsised.
pub const LABEL_TRUNCATE_CHARS: usize = 40;
