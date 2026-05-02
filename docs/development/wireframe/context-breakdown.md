# Stage 3 — Context Breakdown (per turn)

The detail screen for one turn. Two view modes are toggled with `m`:

- `cumulative` _(default)_ — every segment that is in scope by the time of
  turn N. This is what the model actually receives in its provider request
  (session-level Configuration / System loaded once, plus the running
  Runtime history of turns 1..N).
- `delta` — only the segments that **first appeared at turn N** (per-turn
  `user_instructions`, this turn's prompt / assistant message / tool calls).
  Session-level rows are still rendered but dimmed and tagged `[inherited]`
  so the user can see what is "new this turn" at a glance.

## Layout — cumulative mode (120×30)

```
┌─ ctx-analyzer · Turn 3/5 · "Add fuzz tests for inventory underflow…" · view: cumulative ───────────────────────────┐
│ Total ~24,100 !  System ~3,200  Config ~5,800  Runtime ~15,100                                                      │
├─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ Context Breakdown                                                                                                   │
│ ▾ System Context              ~3,200    [session-level · loaded once]                                               │
│   ▸ Base Instructions         ~3,200 (1)                                                                            │
│ ▾ Configuration Context       ~5,800                                                                                │
│   ▸ Project Instructions      ~2,400 (1) [session-level · AGENTS.md]                                                │
│   ▸ User Instructions         ~1,200 (3) [per-turn · turns 1,2,3]                                                   │
│   ▸ Permissions Instructions  ~  400 (1) [session-level]                                                            │
│   ▸ Apps Instructions         ~  900 (1) [session-level]                                                            │
│   ▸ Skills Instructions       ~  600 (1) [session-level]                                                            │
│   ▸ Plugins Instructions      ~  300 (1) [session-level]                                                            │
│ ▾ Runtime Context             ~15,100                                                                               │
│   ▾ User Prompts              ~  480 (3)                                                                            │
│       · turn 1  Review the code changes…                ~120                                                        │
│       · turn 2  Address P1 finding about RNG…           ~140                                                        │
│       · turn 3  Add fuzz tests for inventory…           ~220 ★ this turn                                            │
│   ▸ Assistant Messages        ~12,000 (3)                                                                           │
│   ▸ Function Calls            ~ 2,200 (4)                                                                           │
│   ▸ Function Call Outputs     ~   420 (4)                                                                           │
│ ▾ Delegated Context           —                                                                                     │
│ ▾ Unknown Context             —                                                                                     │
│                                                                                                                     │
│                                                                                                                     │
│                                                                                                                     │
│                                                                                                                     │
│                                                                                                                     │
│                                                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
 ↑↓ move · ←→ collapse/expand · enter/p/space preview · l drill-in · o open · m mode · t turns · s sess · ? help
```

## Layout — delta mode

In delta mode the same skeleton is rendered, but rows that were emitted with
`turn_id == None` (session-level) are dimmed and the badge changes to
`[inherited]`. Per-turn rows whose `turn_id` differs from the active turn are
hidden; rows of the active turn keep their `★ this turn` marker.

```
│ ▾ Configuration Context       ~  400                                                                                │
│   ▸ Project Instructions      ~2,400 (1) [inherited · AGENTS.md]   ← dim                                            │
│   ▸ User Instructions         ~  400 (1) [per-turn] ★ this turn                                                     │
│ ▾ Runtime Context             ~ 5,800                                                                               │
│   ▾ User Prompts              ~  220 (1)                                                                            │
│       · turn 3  Add fuzz tests for inventory…           ~220 ★ this turn                                            │
│   ▸ Assistant Messages        ~ 4,000 (1)                                                                           │
│   ▸ Function Calls            ~ 1,200 (2)                                                                           │
│   ▸ Function Call Outputs     ~   380 (2)                                                                           │
```

## Row badges

| Badge                  | Meaning                                                      |
| ---------------------- | ------------------------------------------------------------ |
| `[session-level]`      | `Segment.turn_id.is_none()` — emitted once at session start  |
| `[per-turn · turns …]` | repeated per `turn_context` event (e.g. `user_instructions`) |
| `★ this turn`          | first appears at the active turn (`turn_id == active_turn`)  |
| `[inherited]` (dimmed) | shown only in **delta** mode for session-level rows          |
| `[AGENTS.md]`          | extra hint on `ProjectInstructions`                          |

## Detail mode (`d`)

When `d` is toggled, the currently selected row gets two extra dim lines
underneath:

```
│       · turn 3  Add fuzz tests for inventory…           ~220 ★ this turn                                            │
│         path: ~/.codex/sessions/2026/05/02/rollout-…f7.jsonl:142  [click to open]                                   │
│         category: Runtime Context · source: User Prompts · confidence: Estimated                                    │
```

The `path:` span is registered as a click region and triggers `o`
(`OpenInEditor`).

## Keymap (active in Stage 3)

| Key                      | Action                                                               |
| ------------------------ | -------------------------------------------------------------------- |
| `↑` / `k`                | move cursor up                                                       |
| `↓` / `j`                | move cursor down                                                     |
| `←` / `h`                | collapse current node                                                |
| `→` / `l`                | expand current node; on a **leaf row**, opens preview (vim drill-in) |
| `Enter`                  | toggle expand or open preview (leaf row)                             |
| `Space`                  | open preview (leaf row only)                                         |
| `p`                      | open preview                                                         |
| `o`                      | open source in editor (`$EDITOR` / `$VISUAL` / fallback chain)       |
| `m`                      | toggle view mode (cumulative ↔ delta)                                |
| `d`                      | toggle inline detail (path / meta) under the selected row            |
| `t`                      | jump to **Turn List**                                                |
| `s`                      | jump to **Session List**                                             |
| `r`                      | rebuild the breakdown from the loaded session                        |
| `g`                      | session graph view (placeholder, footer message only)                |
| `?`                      | toggle help overlay                                                  |
| `q` / `Esc`              | pop stage (Turn List)                                                |
| Mouse left-click on path | open in editor (same as `o`)                                         |
