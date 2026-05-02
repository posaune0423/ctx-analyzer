//! Number-formatting helpers used by the `inspect` text renderer.

/// Insert thousands separators (`,`) into a non-negative integer.
/// `fmt_thousands(1_513_937) == "1,513,937"`.
pub fn fmt_thousands(n: u64) -> String {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + len / 3);
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 && (len - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(*b as char);
    }
    out
}

/// `Some` → `fmt_thousands(n)`; `None` → `"-"`.
pub fn opt_thousands(o: Option<u64>) -> String {
    o.map(fmt_thousands).unwrap_or_else(|| "-".to_string())
}
