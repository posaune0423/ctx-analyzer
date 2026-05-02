//! Char-bounded text helpers. We count Unicode scalar values everywhere
//! so multi-byte CJK / emoji input behaves identically to ASCII.

use unicode_width::UnicodeWidthChar;

/// Trim `text` to at most `max_width` display width; if more were elided,
/// append a single `…` ellipsis.
///
/// Boundary semantics: if total display width <= `max_width` ⇒ verbatim
/// (no ellipsis appended even when equal).
pub fn truncate_chars(text: &str, max_width: usize) -> String {
    let mut total = 0;
    for ch in text.chars() {
        total += ch.width().unwrap_or(0);
    }
    if total <= max_width {
        return text.to_string();
    }

    let mut current_width = 0;
    let mut out = String::with_capacity(text.len().min(max_width * 4 + 4));
    for ch in text.chars() {
        let w = ch.width().unwrap_or(0);
        if current_width + w > max_width {
            break;
        }
        out.push(ch);
        current_width += w;
    }
    out.push('…');
    out
}

/// Pad `text` with spaces to reach `width` display cells.
/// If `text` is already wider than `width`, it is returned as-is.
pub fn pad_to_width(text: &str, width: usize) -> String {
    let mut current = 0;
    for ch in text.chars() {
        current += ch.width().unwrap_or(0);
    }
    if current >= width {
        return text.to_string();
    }
    let extra = width - current;
    let mut out = text.to_string();
    for _ in 0..extra {
        out.push(' ');
    }
    out
}
