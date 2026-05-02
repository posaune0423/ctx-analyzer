# Overlay — Warning Modal

Used to surface non-fatal issues (theme load error, partial-parse warning
summary, …). Wraps the warning text inside a centered box.

## Layout (modal centered, ≈ 60×10)

```
┌─ Warning ──────────────────────────────────────────────────┐
│ Theme could not be loaded.                                 │
│ Using built-in default theme.                              │
│                                                            │
│ themes/custom.toml: invalid color "neon-pink"              │
│                                                            │
│                                                            │
│                                                            │
│                                                            │
└────────────────────────────────────────────────────────────┘
 esc close
```

## Sources of warnings

| Source                                       | Trigger                                                                                                        |
| -------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| `Theme::parse` error                         | invalid TOML / unknown color in user theme                                                                     |
| `Session.warnings` (per-line `ParseWarning`) | shown via footer summary; modal only used when the user opens the warnings list (`r` then `?` flow — post-MVP) |

## Keymap

| Key         | Action      |
| ----------- | ----------- |
| `Esc` / `q` | close modal |
