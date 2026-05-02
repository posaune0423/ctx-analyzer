//! Number-formatting helpers used by `inspect` to render token counts.

use ctx_analyzer::utils::format::{fmt_thousands, opt_thousands};

#[test]
fn small_numbers_keep_their_form() {
    assert_eq!(fmt_thousands(0), "0");
    assert_eq!(fmt_thousands(7), "7");
    assert_eq!(fmt_thousands(999), "999");
}

#[test]
fn four_digits_get_one_comma() {
    assert_eq!(fmt_thousands(1_000), "1,000");
    assert_eq!(fmt_thousands(9_999), "9,999");
}

#[test]
fn large_numbers_get_grouped_in_threes() {
    assert_eq!(fmt_thousands(1_513_937), "1,513,937");
    assert_eq!(fmt_thousands(1_000_000_000), "1,000,000,000");
}

#[test]
fn opt_thousands_renders_dash_for_none() {
    assert_eq!(opt_thousands(None), "-");
}

#[test]
fn opt_thousands_delegates_for_some() {
    assert_eq!(opt_thousands(Some(1_234)), "1,234");
}
