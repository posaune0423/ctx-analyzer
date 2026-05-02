# Stage 1 — Session List

The first screen the user sees when launching `ctx-analyzer inspect` without
`--file` / `--session`. Mirrors the column layout of Codex's `resume_picker`
(`references/codex/codex-rs/tui/src/resume_picker.rs`).

## Layout (120×24)

```
┌─ Sessions · ctx-analyzer · Codex ──────────────────────────────────────────────────────────────────────────────────┐
│  Created          Updated         Conversation                                                                      │
│> 2 min ago        30 sec ago      Inspect 3-stage navigation                                                        │
│  1 h ago          35 min ago      Investigate lazy pagination cap                                                   │
│  2 h ago          2 h ago         Explain the codebase                                                              │
│  3 d ago          3 d ago         —                                                                                 │
│  …                                                                                                                  │
│                                                                                                                     │
│                                                                                                                     │
│                                                                                                                     │
│                                                                                                                     │
│                                                                                                                     │
│                                                                                                                     │
│                                                                                                                     │
│                                                                                                                     │
│                                                                                                                     │
│                                                                                                                     │
│                                                                                                                     │
│                                                                                                                     │
│                                                                                                                     │
│                                                                                                                     │
│                                                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
 ↑↓ select · →/l/enter open · q quit
```

## Column semantics

| Column         | Source                                                               | Width hint |
| -------------- | -------------------------------------------------------------------- | ---------- |
| `Created`      | rollout file `created_at` (fs ctime, fallback: filename date prefix) | 16         |
| `Updated`      | rollout file `modified_at`                                           | 16         |
| `Conversation` | `~/.codex/session_index.jsonl` `thread_name` (`—` when missing)      | flex       |

Times use a relative humanizer (`just now`, `30 sec ago`, `2 min ago`,
`1 h ago`, `3 d ago`).

## Empty state

When no rollouts are discovered:

```
┌─ Sessions · ctx-analyzer · Codex ──────────────────────────────────────────────────────────────────────────────────┐
│                                                                                                                     │
│                                  No Codex sessions found under $CODEX_HOME                                          │
│                                                                                                                     │
│                                  Looked in: /Users/<user>/.codex/sessions                                           │
│                                                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
 q quit
```

## Keymap (active in Stage 1)

| Key         | Action                                 |
| ----------- | -------------------------------------- |
| `↑` / `k`   | move cursor up                         |
| `↓` / `j`   | move cursor down                       |
| `→` / `l`   | load session, advance to **Turn List** |
| `Enter`     | load session, advance to **Turn List** |
| `q` / `Esc` | quit                                   |
| `?`         | toggle help overlay                    |
| `Ctrl+C`    | quit                                   |
