//! Char-bounded text utilities used by the segment preview / label
//! formatters.

use ctx_analyzer::utils::text::truncate_chars;

#[test]
fn shorter_than_limit_is_returned_verbatim() {
    assert_eq!(truncate_chars("hello", 10), "hello");
}

#[test]
fn at_limit_no_ellipsis() {
    assert_eq!(truncate_chars("abcde", 5), "abcde");
}

#[test]
fn longer_than_limit_gets_ellipsis_appended() {
    let out = truncate_chars("abcdefghij", 5);
    assert!(out.ends_with('…'), "got: {out:?}");
    // Must contain exactly `limit` content chars before the ellipsis.
    assert_eq!(out.chars().count(), 6);
    assert!(out.starts_with("abcde"));
}

#[test]
fn multibyte_counted_by_display_width_not_bytes() {
    // 5 hiragana = 15 UTF-8 bytes but 10 display width; limit 4 -> "あい…"
    let out = truncate_chars("あいうえお", 4);
    assert_eq!(out.chars().count(), 3);
    assert!(out.starts_with("あい"));
    assert!(out.ends_with('…'));
}

#[test]
fn empty_input_stays_empty_no_ellipsis() {
    assert_eq!(truncate_chars("", 10), "");
}

#[test]
fn zero_limit_yields_just_the_ellipsis_for_nonempty() {
    let out = truncate_chars("abc", 0);
    assert_eq!(out, "…");
}
