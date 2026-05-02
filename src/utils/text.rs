//! Char-bounded text helpers. We count Unicode scalar values everywhere
//! so multi-byte CJK / emoji input behaves identically to ASCII.

/// Trim `text` to at most `max_chars` characters; if more were elided,
/// append a single `…` ellipsis.
///
/// - `truncate_chars("abcdefghij", 5) == "abcde…"`
/// - `truncate_chars("abc", 5)        == "abc"`
/// - `truncate_chars("",    5)        == ""`
/// - `truncate_chars("xyz", 0)        == "…"`
///
/// Boundary semantics: `text.chars().count() <= max_chars` ⇒ verbatim
/// (no ellipsis appended even when equal).
pub fn truncate_chars(text: &str, max_chars: usize) -> String {
    let total = text.chars().count();
    if total <= max_chars {
        return text.to_string();
    }
    let mut out = String::with_capacity(text.len().min(max_chars * 4 + 4));
    for (idx, ch) in text.chars().enumerate() {
        if idx >= max_chars {
            break;
        }
        out.push(ch);
    }
    out.push('…');
    out
}
