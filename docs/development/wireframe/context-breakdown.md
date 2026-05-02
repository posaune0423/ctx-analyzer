# Stage 3 — Context Breakdown (per interaction)

The detail screen for one interaction. Two view modes are toggled with `m`:

- `cumulative` _(default)_ — every segment that is in scope by the time of
  interaction N. This is what the model actually receives in its provider request
  (session-level Configuration / System loaded once, plus the running
  Runtime history of interactions 1..N).
- `delta` — only the segments that **first appeared at interaction N** (per-interaction
  `user_instructions`, this interaction's prompt / assistant message / tool calls).
  Session-level rows are still rendered but dimmed and tagged `[inherited]`
  so the user can see what is "new" at a glance.

## Layout — cumulative mode (120×30)

When a preview is active (via `enter`, `p`, or `space`), the screen splits into a 50/50 horizontal layout.

```
┌─ ctx-analyzer · 3/5 · "Add fuzz tests for inventory underflow…" · view: cumulative ────────────────────────────────┐
│ Total ~24,100 !  System ~3,200  Config ~5,800  Runtime ~15,100                                                      │
├───────────────────────────────────────┬─────────────────────────────────────────────────────────────────────────────┤
│ Context Breakdown                     │ Preview · User Prompt · ~220 tokens                                         │
│ ▾ System Context              ~3,200  │ Source: rollout-…f7.jsonl:142                                               │
│   ▸ Base Instructions         ~3,200  │ Label : User Prompt                                                         │
│ ▾ Configuration Context       ~5,800  │ ────────────────────────────────────────────────────────────────────────────│
│   ▸ Project Instructions      ~2,400  │ Add fuzz tests for inventory underflow boundary. Use vm.assume to constrain │
│   ▸ User Instructions         ~1,200  │ the amount to <= currentBalance and verify that decrement reverts when      │
│ ▾ Runtime Context             ~15,100 │ amount > balance.                                                           │
│   ▾ User Prompts              ~  480  │                                                                             │
│       · 1  Review the code…     ~120  │ Coverage:                                                                   │
│       · 2  Address P1 find…     ~140  │ - happy path (amount within balance) does NOT revert                        │
│       · 3  Add fuzz tests…      ~220  │ - boundary (amount == balance) decrements to zero                           │
│   ▸ Assistant Messages        ~12,000 │ - over-draw (amount > balance) reverts with InsufficientBalance()            │
│   ▸ Function Calls            ~ 2,200 │                                                                             │
│   ▸ Function Call Outputs     ~   420 │ Update .gas-snapshot afterwards so the new tests are tracked.               │
│ ▾ Delegated Context           —       │                                                                             │
│ ▾ Unknown Context             —       │ (full text — scroll with j/k)                                               │
└───────────────────────────────────────┴─────────────────────────────────────────────────────────────────────────────┘
 ↑↓ move · ←→ collapse/expand · enter/p/space preview · o open · m mode · t interactions · s sess · ? help
```

## Layout — delta mode (no preview)

When no preview is active, the breakdown takes the full width.

```
┌─ ctx-analyzer · 3/5 · "Add fuzz tests for inventory underflow…" · view: delta ─────────────────────────────────────┐
│ Total ~5,800  System ~0  Config ~400  Runtime ~5,400                                                                │
├─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ Context Breakdown                                                                                                   │
│ ▾ Configuration Context       ~  400                                                                                │
│   ▸ Project Instructions      ~2,400 [inherited]                                                                    │
│   ▸ User Instructions         ~  400 [current]                                                                      │
│ ▾ Runtime Context             ~ 5,400                                                                               │
│   ▾ User Prompts              ~  220                                                                                │
│       · 3  Add fuzz tests for inventory…                ~220 ★ current                                              │
...
```

## Row badges

| Badge             | Meaning                                                     |
| ----------------- | ----------------------------------------------------------- |
| `[session-level]` | `Segment.turn_id.is_none()` — emitted once at session start |
| `[1,2,3]`         | repeated across interactions 1, 2, and 3                    |
| `★ current`       | first appears at the active interaction                     |
| `[inherited]`     | shown only in **delta** mode for session-level rows         |
| `[AGENTS.md]`     | extra hint on `ProjectInstructions`                         |

## Detail mode (`d`)

When `d` is toggled, the currently selected row gets two extra dim lines
underneath:

```
│       · 3  Add fuzz tests for inventory…                ~220 ★ current                                              │
│         path: ~/.codex/sessions/2026/05/02/rollout-…f7.jsonl:142  [click to open]                                   │
│         category: Runtime Context · source: User Prompts · confidence: Estimated                                    │
```

The `path:` span is registered as a click region and triggers `o`
(`OpenInEditor`).

## Keymap (active in Stage 3)

| Key         | Action                                                               |
| ----------- | -------------------------------------------------------------------- |
| `↑` / `k`   | move cursor up                                                       |
| `↓` / `j`   | move cursor down                                                     |
| `←` / `h`   | collapse current node                                                |
| `→` / `l`   | expand current node; on a **leaf row**, opens preview (vim drill-in) |
| `Enter`     | toggle expand or open preview (leaf row)                             |
| `Space`     | open preview (leaf row only)                                         |
| `p`         | open preview                                                         |
| `o`         | open source in editor (`$EDITOR` / `$VISUAL` / fallback chain)       |
| `m`         | toggle view mode (cumulative ↔ delta)                                |
| `d`         | toggle inline detail (path / meta) under the selected row            |
| `t`         | jump to **Interaction List**                                         |
| `s`         | jump to **Session List**                                             |
| `r`         | rebuild the breakdown from the loaded session                        |
| `g`         | session graph view (placeholder, footer message only)                |
| `?`         | toggle help overlay                                                  |
| `q` / `Esc` | pop stage (Interaction List)                                         |
