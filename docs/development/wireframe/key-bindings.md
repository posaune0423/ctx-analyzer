# Key Bindings (cross-stage)

Single source of truth for the `(Stage, Key) → KeyAction` mapping. The
implementation lives in `src/cli/tui/keymap.rs`; whenever this table changes,
both `keymap.rs` and the corresponding `tests/unit/cli/tui/keymap.rs`
expectations must be updated together.

## Modes

| Mode          | Surface                                             |
| ------------- | --------------------------------------------------- |
| `SessionList` | Stage 1 full-screen (was `SessionSelector` overlay) |
| `TurnList`    | Stage 2 full-screen (was `TurnSelector` overlay)    |
| `Breakdown`   | Stage 3 full-screen (was `Main` accordion view)     |
| `Preview`     | overlay on top of `Breakdown`                       |
| `Help`        | overlay on top of any stage                         |

## Global

| Key      | Action                             |
| -------- | ---------------------------------- |
| `Ctrl+C` | quit                               |
| `?`      | toggle Help overlay (in any stage) |

## SessionList

| Key         | Action                                 |
| ----------- | -------------------------------------- |
| `↑` / `k`   | `MoveUp`                               |
| `↓` / `j`   | `MoveDown`                             |
| `Enter`     | `Confirm` → load + advance to TurnList |
| `q` / `Esc` | `Quit` (bottom of stack)               |

## TurnList

| Key         | Action                                                                          |
| ----------- | ------------------------------------------------------------------------------- |
| `↑` / `k`   | `MoveUp`                                                                        |
| `↓` / `j`   | `MoveDown`                                                                      |
| `Enter`     | `Confirm` → enter Breakdown for selected turn                                   |
| `s`         | `OpenSessionSelector` (jump to SessionList)                                     |
| `q` / `Esc` | `PopStage` (back to SessionList; quits if launched with `--file` / `--session`) |

## Breakdown

| Key         | Action                                                              |
| ----------- | ------------------------------------------------------------------- |
| `↑` / `k`   | `MoveUp`                                                            |
| `↓` / `j`   | `MoveDown`                                                          |
| `←` / `h`   | `MoveLeft` (collapse)                                               |
| `→` / `l`   | `MoveRight` (expand) — on a **leaf row** behaves as `OpenPreview`   |
| `Enter`     | `Confirm` (toggle expand on Section/Group, OpenPreview on leaf Row) |
| `Space`     | `OpenPreview` (leaf row only)                                       |
| `p`         | `OpenPreview`                                                       |
| `o`         | `OpenInEditor`                                                      |
| `m`         | `ToggleViewFilter` (cumulative ↔ delta)                             |
| `d`         | `ToggleDetail`                                                      |
| `r`         | `Refresh`                                                           |
| `t`         | `OpenTurnSelector` (jump to TurnList)                               |
| `s`         | `OpenSessionSelector` (jump to SessionList)                         |
| `g`         | `SessionGraphPlaceholder` (footer message)                          |
| `/`         | `StartSearch` (footer placeholder)                                  |
| `q` / `Esc` | `PopStage` (back to TurnList)                                       |

## Preview overlay

| Key         | Action              |
| ----------- | ------------------- |
| `↑` / `k`   | `MoveUp` (scroll)   |
| `↓` / `j`   | `MoveDown` (scroll) |
| `o`         | `OpenInEditor`      |
| `q` / `Esc` | `CloseOverlay`      |

## Help overlay

| Key               | Action         |
| ----------------- | -------------- |
| `?` / `q` / `Esc` | `CloseOverlay` |
