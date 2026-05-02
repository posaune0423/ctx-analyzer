# Overlay — Preview Modal

The Preview Modal shows the **full payload text** of one `ContextSegment`,
read on-demand from the rollout JSONL line referenced by
`segment.source_ref.line`.

Behavior change versus the previous implementation: rollout files are no
longer skipped. Instead the modal:

1. Reads exactly one line from the JSONL at `source_ref.line`
   (`infra::json::read_jsonl_line`).
2. Parses it into the typed `Envelope`.
3. Extracts the appropriate text payload based on `segment.source_kind`
   (e.g. `event_msg.user_message.message`, `response_item.message.content[].text`,
   `function_call.arguments`, `function_call_output.output`,
   `session_meta.base_instructions.text`,
   `turn_context.user_instructions`, …).
4. Falls back to `segment.preview` if the file/line cannot be read or the
   payload shape doesn't match.

## Layout (modal centered, ≈ 100×24 over a 120×30 frame)

```
┌─ Preview · User Prompt (turn 3) · ~220 tokens ─────────────────────────────────────────────────────┐
│ Source: ~/.codex/sessions/2026/05/02/rollout-…f7.jsonl:142  [click to open]                         │
│ Label : User Prompt                                                                                 │
│ ────────────────────────────────────────────────────────────────────────────────────────────────────│
│ Add fuzz tests for inventory underflow boundary. Use vm.assume to constrain the                     │
│ amount to <= currentBalance and verify that decrement reverts when amount > balance.                │
│                                                                                                     │
│ Coverage:                                                                                           │
│ - happy path (amount within balance) does NOT revert                                                │
│ - boundary (amount == balance) decrements to zero                                                   │
│ - over-draw (amount > balance) reverts with InsufficientBalance()                                   │
│                                                                                                     │
│ Update .gas-snapshot afterwards so the new tests are tracked.                                       │
│                                                                                                     │
│ (full text — no truncation; scroll with j/k)                                                        │
│                                                                                                     │
│                                                                                                     │
│                                                                                                     │
│                                                                                                     │
│                                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────────────────────┘
 j/k scroll · o open in editor · click path to open · esc close
```

## Header rows

| Field    | Source                                                                                           |
| -------- | ------------------------------------------------------------------------------------------------ |
| Title    | `Preview · {segment.label} · ~{tokens} tokens` (`(turn N)` suffix when `segment.turn_id` is set) |
| `Source` | `segment.source_ref.file`:`segment.source_ref.line`.                                             |
| `Label`  | `segment.label`                                                                                  |

## Body extraction matrix

| `source_kind`                                  | Extracted payload                                                                             |
| ---------------------------------------------- | --------------------------------------------------------------------------------------------- |
| `BaseInstructions`                             | `session_meta.base_instructions.text` _or_ developer message text                             |
| `UserInstructions`                             | `turn_context.user_instructions`                                                              |
| `ProjectInstructions`                          | `response_item.message.content[].text` (joined)                                               |
| `Permissions/Apps/Skills/Plugins Instructions` | corresponding tagged block of the developer message (re-extracted via classifier on the line) |
| `UserPrompt` (event_msg)                       | `event_msg.user_message.message`                                                              |
| `UserPrompt` (response_item)                   | `response_item.message.content[].text`                                                        |
| `AssistantMessage` (event_msg)                 | `event_msg.agent_message.message`                                                             |
| `AssistantMessage` (response_item)             | `response_item.message.content[].text`                                                        |
| `FunctionCall`                                 | `response_item.function_call.arguments` (JSON string verbatim)                                |
| `FunctionCallOutput`                           | `response_item.function_call_output.output`                                                   |
| `EncryptedReasoning`                           | empty + dim note `(encrypted reasoning · not decryptable locally)`                            |
| `SubagentMarker`                               | `segment.preview` (already a synthetic note)                                                  |
| anything else / fallback                       | `segment.preview`                                                                             |

If the line / parse fails, render `segment.preview` and a single dim line
`(could not re-read JSONL line; showing cached preview)`.

## Truncation

Body is clamped to **65,536 chars** to keep the buffer rendering fast. When
clamped, append a final dim line `(truncated — body exceeded 65,536 chars)`.

## Keymap (active in Preview overlay)

| Key                      | Action                     |
| ------------------------ | -------------------------- |
| `j` / `↓`                | scroll down                |
| `k` / `↑`                | scroll up                  |
| `o`                      | open source file in editor |
| Mouse left-click on path | open source file in editor |
| `Esc` / `q`              | close overlay              |
|                          |
