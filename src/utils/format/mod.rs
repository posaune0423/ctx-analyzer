//! Number-formatting helpers used by the `inspect` text renderer.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Utc};

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

/// Human-readable relative age for TUI list columns (`2 min ago`, `just now`).
pub fn relative_time_ago(t: Option<SystemTime>) -> String {
    let Some(t) = t else {
        return "—".to_string();
    };
    let now = SystemTime::now();
    let Ok(dur) = now.duration_since(t) else {
        return "—".to_string();
    };
    format_duration_ago(dur)
}

/// Relative age from a UTC timestamp (turn `started_at` in the domain).
pub fn relative_time_utc(dt: Option<DateTime<Utc>>) -> String {
    let Some(dt) = dt else {
        return "—".to_string();
    };
    let st = UNIX_EPOCH + Duration::from_secs(dt.timestamp().max(0) as u64);
    relative_time_ago(Some(st))
}

fn format_duration_ago(d: Duration) -> String {
    let s = d.as_secs();
    if s < 5 {
        return "just now".to_string();
    }
    if s < 60 {
        return format!("{s} sec ago");
    }
    if s < 3_600 {
        return format!("{} min ago", s / 60);
    }
    if s < 86_400 {
        return format!("{} h ago", s / 3_600);
    }
    format!("{} d ago", s / 86_400)
}
