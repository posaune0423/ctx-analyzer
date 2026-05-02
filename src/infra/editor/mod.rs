//! Editor launcher implementation (UI.md §19). Reserved for the TUI
//! integration. Resolution order will be:
//!
//! 1. `$CTX_ANALYZER_EDITOR`
//! 2. `$VISUAL`
//! 3. `$EDITOR`
//! 4. `cursor` / `code` / `zed` / `nvim` / `vim`
