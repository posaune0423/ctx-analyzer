# Stage 2 — Turn List

After a session is loaded, the user picks one turn to inspect in detail.
Each row shows the user prompt that opened that turn so the user can navigate
by intent rather than by index.

## Layout (120×24)

```
┌─ ctx-analyzer · 019d72f3…dcdff · ~/Work/velvett-io/unigacha-contracts · 5 turns ───────────────────────────────────┐
│   #   Tokens    When           User Prompt                                                                          │
│>  1   ~12,400   2 min ago      Review the code changes against the base branch 'main'. The merge base co…           │
│   2   ~18,200   1 min ago      Address P1 finding about RNG seed reuse in batch draws                               │
│   3   ~24,100   30 sec ago     Add fuzz tests for inventory underflow boundary                                      │
│   4   ~28,800   10 sec ago     Re-run forge snapshot and verify gas budget delta                                    │
│   5   ~31,500   just now       Document the security review findings in PR description                              │
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
 ↑↓ select · enter inspect turn · s sessions · q back
```

## Column semantics

| Column        | Source                                                               | Width hint |
| ------------- | -------------------------------------------------------------------- | ---------- |
| `#`           | `Turn.index`                                                         | 4          |
| `Tokens`      | sum of `Segment.tokens.tokens` over `Turn.segment_ids`, prefixed `~` | 9          |
| `When`        | relative `Turn.started_at` (`—` when unknown)                        | 14         |
| `User Prompt` | first `UserPrompt` segment of the turn, char-truncated to 60         | flex       |

A `⚠` glyph appears at the end of the row when the turn produced a delegated
subagent (`Segment.source_kind == SubagentMarker`).

## Empty / unusual states

- **Session has no turns** (rare; corrupt rollout): show `No turns recorded
for this session.` centered.
- **Turn has no UserPrompt** (e.g., delegated review without a prompt
  segment): the prompt cell shows the dim placeholder `(no user prompt
recorded)`.

## Keymap (active in Stage 2)

| Key         | Action                                                                  |
| ----------- | ----------------------------------------------------------------------- |
| `↑` / `k`   | move cursor up                                                          |
| `↓` / `j`   | move cursor down                                                        |
| `Enter`     | open the **Context Breakdown** for the selected turn                    |
| `s`         | jump back to **Session List**                                           |
| `q` / `Esc` | pop stage (Session List, or quit if launched with `--file`/`--session`) |
| `?`         | toggle help overlay                                                     |
