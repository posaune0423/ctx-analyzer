//! Built-in default theme baked in at compile time so `ctx-analyzer`
//! always has something to fall back to (UI.md §12 step 4 / §24 invalid
//! theme path).

pub const DEFAULT_THEME_TOML: &str = include_str!("../../../../themes/default.toml");
