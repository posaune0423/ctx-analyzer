# Stage 2 — Interaction List

After a session is loaded, the user picks one interaction to inspect in detail.
Each row shows the user prompt that opened that interaction so the user can navigate
by intent rather than by index.

## Layout (120×24)

```
┌─ ctx-analyzer · 019d72f3…dcdff · ~/Work/velvett-io/unigacha-contracts · 5 ──────────────────────────────────────────┐
│   When           Conversation                                                            Tokens                     │
│>  2 min ago      Review the code changes against the base branch 'main'. The merge base co…      ~12,400            │
│   1 min ago      Address P1 finding about RNG seed reuse in batch draws                          ~18,200            │
│   30 sec ago     Add fuzz tests for inventory underflow boundary                                 ~24,100            │
│   10 sec ago     Re-run forge snapshot and verify gas budget delta                               ~28,800            │
│   just now       Document the security review findings in PR description                         ~31,500            │
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
 ↑↓ select · enter inspect · s sessions · q back
```

## Column semantics

| Column         | Source                                                               | Width hint |
| -------------- | -------------------------------------------------------------------- | ---------- |
| `When`         | relative `Turn.started_at` (`—` when unknown)                        | 14         |
| `Conversation` | first `UserPrompt` segment of the turn, char-truncated to 72         | flex       |
| `Tokens`       | sum of `Segment.tokens.tokens` over `Turn.segment_ids`, prefixed `~` | 10         |

A `⚠` glyph appears at the end of the `Conversation` cell when the interaction produced a delegated
subagent (`Segment.source_kind == SubagentMarker`).

## Empty / unusual states

- **Session has no interactions** (rare; corrupt rollout): show `No interactions recorded for this session.` centered.
- **Interaction has no UserPrompt** (e.g., delegated review without a prompt
  segment): the prompt cell shows the dim placeholder `(no user prompt
recorded)`.

## Keymap (active in Stage 2)

| Key         | Action                                                                  |
| ----------- | ----------------------------------------------------------------------- |
| `↑` / `k`   | move cursor up                                                          |
| `↓` / `j`   | move cursor down                                                        |
| `Enter`     | open the **Context Breakdown** for the selected interaction             |
| `s`         | jump back to **Session List**                                           |
| `q` / `Esc` | pop stage (Session List, or quit if launched with `--file`/`--session`) |
| `?`         | toggle help overlay                                                     |
