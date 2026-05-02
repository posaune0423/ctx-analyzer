# Overlay — Help Modal

Static keymap reference. Reachable from any stage with `?`. Closed with
`?`, `Esc`, or `q`.

## Layout (modal centered, ≈ 60×22)

```
┌─ Help ─────────────────────────────────────────────────────────┐
│ Key            Action                                          │
│ q              quit (or pop one stage)                         │
│ ?              toggle this help                                │
│ ↑↓ / jk        move                                            │
│ ←→ / hl        collapse / expand · l on leaf row → preview     │
│ enter          confirm (open / preview)                        │
│ space          preview selected segment                        │
│ p              preview selected segment                        │
│ o              open source in editor                           │
│ click path     open source in editor                           │
│ s              jump to Session List                            │
│ t              jump to Turn List                               │
│ m              toggle view mode (cumulative / delta)           │
│ d              toggle row detail                               │
│ r              refresh breakdown                               │
│ g              session graph (coming soon)                     │
│ /              search (coming soon)                            │
└────────────────────────────────────────────────────────────────┘
 esc close
```

The exact list also lives in `key-bindings.md`; both must stay in sync with
the `HELP` table in `src/cli/tui/overlays/help_modal.rs`.
