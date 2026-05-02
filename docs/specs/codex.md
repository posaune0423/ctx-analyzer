# Codex Adapter Spec

`ctx-analyzer` の最初の adapter。`src/adapters/codex/` に閉じ込め、Codex 固有の
file path / schema / marker を domain に漏らさない。

## 1. Discovery

`src/adapters/codex/discovery/`:

- `codex_home()` — 環境変数 `CODEX_HOME` を優先。なければ `dirs::home_dir().join(".codex")`。
- `list_rollouts()` — `<codex_home>/sessions/**/rollout-*.jsonl` を glob する。

実例 path:

- `~/.codex/sessions/2026/04/09/rollout-2026-04-09T15-54-43-019d72f3-f339-71c3-810e-f714359dcdff.jsonl`

## 2. Raw schema

`src/adapters/codex/raw/`:

JSONL の 1 行は `{ timestamp, type, payload }` の Envelope。

`type` の値:

| `type`          | payload 構造 (主要)                                                                                             |
| --------------- | --------------------------------------------------------------------------------------------------------------- |
| `session_meta`  | `{id, timestamp, cwd, originator, cli_version, source: {subagent?}, model_provider, base_instructions: {text}}` |
| `event_msg`     | inner `payload.type ∈ {task_started, task_complete, token_count, user_message, agent_message}`                  |
| `response_item` | inner `payload.type ∈ {message, reasoning, function_call, function_call_output}`                                |
| `turn_context`  | `{turn_id, model, user_instructions, ...}`                                                                      |

すべての enum は未知 variant を `#[serde(other)]` で吸収（前方互換）。
すべての非 critical field は `Option<T>` + `#[serde(default)]`。

## 3. Parser

`src/adapters/codex/parsers/`:

- 行ごとに `serde_json::from_str::<Envelope>` を走らせる。
- 失敗行は `ParseWarning { line, message }` に積んで継続（PRD §13.2 partial result）。
- 空行は warning なしで skip。

## 4. Classifier

`src/adapters/codex/classifiers/`:

- `split_developer_blocks(text)`:
  最初の `developer` role の message text を、以下の tag で分割:
  - `<permissions instructions>...</permissions instructions>` → `PermissionsInstructions`
  - `<apps_instructions>...</apps_instructions>` → `AppsInstructions`
  - `<skills_instructions>...</skills_instructions>` → `SkillsInstructions`
  - `<plugins_instructions>...</plugins_instructions>` → `PluginsInstructions`
  - 残った text は `BaseInstructions` 1 segment
  - close tag が無い open は emit せず、leftover に流れる。
- `looks_like_project_doc(text)`:
  `text.trim_start().starts_with("# AGENTS.md instructions for ")` を検査。
  最初の `user` role message がこれに合致したら `ProjectInstructions` (Configuration)、
  以降は `UserPrompt` (Runtime)。

## 5. Mapper

`src/adapters/codex/mappers/`:

events を順番に走査して `Session / Turn / ContextSegment` を構築:

| 入力                                             | category      | source_kind           |
| ------------------------------------------------ | ------------- | --------------------- |
| `session_meta.base_instructions.text`            | System        | `BaseInstructions`    |
| `turn_context.user_instructions`                 | Configuration | `UserInstructions`    |
| 最初の `developer` message の tagged block 4 種  | Configuration | 各 `*Instructions`    |
| 最初の `developer` message の leftover           | Configuration | `BaseInstructions`    |
| 最初の `user` message が project-doc prefix      | Configuration | `ProjectInstructions` |
| 以降の `user` message / `event_msg.user_message` | Runtime       | `UserPrompt`          |
| `assistant` message / `agent_message`            | Runtime       | `AssistantMessage`    |
| `response_item.function_call`                    | Runtime       | `FunctionCall`        |
| `response_item.function_call_output`             | Runtime       | `FunctionCallOutput`  |
| `response_item.reasoning` (encrypted_content 有) | Unknown       | `EncryptedReasoning`  |
| `session_meta.source.subagent.is_some()`         | Delegated     | `SubagentMarker`      |

各 segment は `source_ref { file: rollout_path, line: <1-based JSONL line> }` を持つ。

Turn bracketing: `event_msg.task_started{turn_id}` で open / `task_complete` で close。
turn_id を持たない segment は最後に open した turn に紐付く。

## 6. Token estimation

- 各 segment は port `TokenEstimator::estimate(text)` (default: `CharsPer4Estimator`) で estimate。Confidence: `Estimated`。
- Reasoning encrypted blob は `tokens=0`, Confidence: `Unknown`。
- Session totals は `info` が non-null な `event_msg.token_count` のうち**最後**の `total_token_usage` を採用。Confidence: `Observed`。

## 7. Graph

`src/adapters/codex/graph/`:

MVP では single-file scope:

- `SessionGraph.root_id = session_id`
- `is_delegated_child = session.source.subagent.is_some()`
- `subagent_label = source.subagent`
- `children = []`

cross-file な parent → child rollout linking は post-MVP（`docs/specs/session-graph.md`）。

## 8. Limitations / known gaps

- `# AGENTS.md instructions for ` prefix を欠く project doc は `UserPrompt` に分類される。
- 観測 sample に依存して enum 化した tag セット以外（将来 Codex が追加する `<*_instructions>`）は leftover に流れる。
- `~/.codex/state.db` の SQLite ingestion は post-MVP（`docs/specs/diagnostics.md`）。

## 9. Post-MVP findings from `references/codex/`

### 9.1 `session_index.jsonl`

- Path: `<codex_home>/session_index.jsonl`
- Per-line shape: `{"id": "<thread-uuid>", "thread_name": "<string>", "updated_at": "<RFC3339>"}`
- Append-only; resolution scans **from EOF backwards** and returns the first
  match per id (i.e. **last entry wins**).
- Source: `references/codex/codex-rs/rollout/src/session_index.rs:28-49`.
- Action item: when our adapter learns to surface human session labels,
  consume this file via `adapters/codex/discovery::session_index_path()`.

### 9.2 `state.db` (SQLite cache)

- Path: `<codex_home>/state.db`
- Tables: thread metadata + `BackfillStatus { NotStarted, Running, Complete }`.
- Used by Codex as a **performance optimisation** for `list_threads()`,
  `get_thread()`, `upsert_thread()`. **Not** a fallback for the JSONL files.
- Source: `references/codex/codex-rs/core/state_db.rs`.
- Action item: open with `SQLITE_OPEN_READONLY` from `infra/sqlite/`.

### 9.3 `RolloutItem::Compacted`

- Top-level rollout variant (NOT a sub-variant of `event_msg`).
- Payload: `{ "replacement_history": [<RolloutItem>, …] }` — the items that
  existed before the compaction summary replaced them.
- Source: `references/codex/codex-rs/rollout/src/recorder.rs:897-898`.
- Action item: extend `adapters/codex/raw::Payload` with a `Compacted` variant
  (currently absorbed by `Other`); emit a `Configuration / CompactSummary`
  segment and surface it in the breakdown.

### 9.4 Additional `event_msg` variants observed in upstream

Beyond the five we currently parse (`task_started, task_complete, token_count,
user_message, agent_message`), upstream emits at least:

- `thread_name_updated`
- `context_compacted`
- `exec_command_end`
- `patch_apply_end`

All currently fall through to `EventMsg::Other` (forward-compat). Post-MVP we
will lift `exec_command_end` / `patch_apply_end` into `Runtime` segments to
give a more accurate runtime breakdown.

## 10. Reference

- 上流実装: `references/codex/codex-rs/`
- rollout path 構築: `references/codex/codex-rs/rollout/src/recorder.rs:1363-1393`
- session_index: `references/codex/codex-rs/rollout/src/session_index.rs:17-49`
- state.db: `references/codex/codex-rs/core/state_db.rs`
