//! Preview / label truncation limits used across adapters and the CLI
//! renderer.

/// Maximum number of Unicode scalar values kept in a `ContextSegment.preview`
/// string. Anything beyond is replaced with a single `…` ellipsis.
pub const PREVIEW_LIMIT_CHARS: usize = 200;

/// Maximum width (in chars) for row labels in the `inspect` accordion
/// before they get ellipsised.
pub const LABEL_TRUNCATE_CHARS: usize = 40;

/// Hard cap on preview-modal body size (Unicode scalar values) after
/// extracting text from a rollout JSONL line (`docs/development/wireframe/preview-modal.md`).
pub const PREVIEW_BODY_MAX_CHARS: usize = 65_536;

/// Approximate number of body lines shown in the Preview modal (used for
/// scroll clamping in `app.rs` and `preview_modal.rs`).
pub const PREVIEW_MODAL_VISIBLE_BODY_LINES: usize = 18;
