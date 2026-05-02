# JSON Export Spec

## 1. Goal

PRD §13.1 #18: 「agent-agnostic JSON として export できる」。
schema は agent 中立で、Codex 固有名（rollout, AGENTS.md, etc.）を JSON key に
含めない。

## 2. Schema (`schema_version: "0.1"`)

```jsonc
{
  "schema_version": "0.1",
  "agent": "codex",
  "session": {
    "id": "019d72f3-…",
    "started_at": "2026-04-09T15:54:43.396Z",
    "cwd": "/Users/.../unigacha-contracts",
    "source": {
      "path": "tests/fixtures/codex/rollout-with-subagent/ctx.jsonl",
      "subagent": "review"
    },
    "model_provider": "openai",
    "cli_version": "0.118.0",
    "is_delegated_child": true
  },
  "totals": {
    "total_tokens": 1513937,
    "input_tokens": 1503435,
    "cached_input_tokens": 1417984,
    "output_tokens": 10502,
    "reasoning_output_tokens": 8207,
    "confidence": "Observed"
  },
  "categories": {
    "system":        { "total_tokens": N, "segments": [Segment, …] },
    "configuration": { "total_tokens": N, "segments": […] },
    "runtime":       { "total_tokens": N, "segments": […] },
    "delegated":     { "total_tokens": N, "segments": […] },
    "unknown":       { "total_tokens": N, "segments": […] }
  },
  "turns": [
    {
      "id": "019d72f3-…",
      "index": 1,
      "model": "gpt-…",
      "user_instructions": "…",
      "segment_ids": [0, 1, …],
      "last_token_usage": { "tokens": …, "confidence": "Observed", … }
    }
  ],
  "warnings": [
    { "line": 42, "message": "json parse error: …" }
  ]
}
```

### 2.1 `Segment` shape

```jsonc
{
  "id": 7,
  "turn_id": "019d72f3-…",
  "category": "configuration",
  "source_kind": "permissions_instructions",
  "label": "Permissions Instructions",
  "preview": "Filesystem sandboxing defines …",     // ≤200 chars + "…"
  "full_len_chars": 307,
  "source_ref": {
    "file": "tests/fixtures/codex/rollout-with-subagent/ctx.jsonl",
    "line": 3
  },
  "tokens": { "tokens": 77, "confidence": "Estimated" },
  "confidence": "Estimated"
}
```

## 3. Stability rules

- top-level keys (`schema_version, agent, session, totals, categories, turns, warnings`) は **削除しない / rename しない**。`schema_version` を bump する場合のみ。
- category keys は固定: `system, configuration, runtime, delegated, unknown` (PRD §12)。
- `source_kind` は新値追加可能（forward compat: consumer は未知値を skip してよい）。

## 4. Tests

- `tests/integration/export/json_schema.rs`:
  - bundled fixture を export し、top-level keys を全て assert
  - `schema_version == "0.1"`, `agent == "codex"`
  - `totals.total_tokens == 1_513_937`
  - 5 つの category key 存在
- `tests/integration/cli/export.rs`:
  - `ctx-analyzer export --file <fixture>` の stdout が JSON parse 可能
  - 同 invariants
