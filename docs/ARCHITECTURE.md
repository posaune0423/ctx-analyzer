# ctx-analyzer ARCHITECTURE

## 1. Overview

`ctx-analyzer` は Clean Architecture + DDD を基本方針にする。

最初は Codex adapter から実装するが、core は agent-agnostic に保つ。

各 coding agent 固有の session 保存方式、context 構成、instruction source、capability source、subagent / child session の扱いは adapter に閉じ込める。

---

## 2. Module Relationship

```txt
CLI / TUI
  -> Application
    -> Domain
    -> Ports
  -> Adapters
  -> Infrastructure
```

依存方向は内側に向ける。

- `domain` は他 layer に依存しない。
- `application` は `domain` と `ports` に依存する。
- `ports` は interface を定義する。
- `adapters` は agent-specific な実装を持つ。
- `infra` は filesystem / SQLite / JSON / TOML / editor などの concrete IO を持つ。
- `cli` / `tui` は user-facing な presentation を持つ。

---

## 3. Source Tree

```txt
src/
  main.rs
  domain/
    workspace/
    agent/
    session/
    session_graph/
    turn/
    context/
      source/
      segment/
      category/
    token/
    confidence/
  application/
    usecases/
      analyze_workspace/
      detect_agents/
      discover_sessions/
      reconstruct_session_graph/
      build_context_breakdown/
      estimate_tokens/
      export_json/
      compare/
    view_models/
      workspace/
      session/
      context_breakdown/
  ports/
    agent_adapter/
    session_source_reader/
    context_source_reader/
    token_estimator/
    file_preview_reader/
    editor_launcher/
    exporter/
  adapters/
    codex/
      discovery/
      raw/
      parsers/
      classifiers/
      mappers/
      graph/
    claude_code/
    cursor/
    gemini/
  infra/
    fs/
    json/
    toml/
    sqlite/
    editor/
  tui/
    app/
    state/
    layout/
    widgets/
  cli/
    commands/
      inspect/
      export/
      doctor/
      compare/
```

---

## 4. Domain

`src/domain/` は agent-agnostic な core model を置く。

- `workspace/`: 解析対象 workspace を表す。
- `agent/`: Codex / Claude Code / Cursor などの agent 種別を表す。
- `session/`: coding agent の session を表す。
- `session_graph/`: parent / child session、delegated session、orphan session の関係を表す。
- `turn/`: user interaction 単位を表す。
- `context/source/`: instruction / configuration / capability / runtime / delegation source を表す。
- `context/segment/`: token 計測・preview・source tracking の単位を表す。
- `context/category/`: system / configuration / runtime / delegated / unknown などの分類を表す。
- `token/`: token estimate を表す。
- `confidence/`: reconstruction confidence を表す。

Rule:

- Codex 固有名を入れない。
- `AGENTS.md`、`CLAUDE.md`、`GEMINI.md`、Cursor rules などの固有名を入れない。
- rollout schema や DB schema を入れない。

---

## 5. Application

`src/application/` は usecase と view model を置く。

- `usecases/analyze_workspace/`: workspace 全体の解析を行う。
- `usecases/detect_agents/`: 利用可能な coding agent を検出する。
- `usecases/discover_sessions/`: session 一覧を取得する。
- `usecases/reconstruct_session_graph/`: session 間の parent / child relationship を復元する。
- `usecases/build_context_breakdown/`: context source / segment を分類・集計する。
- `usecases/estimate_tokens/`: token estimate を付与する。
- `usecases/export_json/`: JSON export を生成する。
- `usecases/compare/`: agent / session / turn の比較を行う。
- `view_models/`: CLI / TUI が表示しやすい形の model を置く。

Rule:

- agent-specific parser を直接呼ばない。
- filesystem / SQLite などの concrete IO を直接呼ばない。
- adapter / infra には `ports` 経由でアクセスする。

---

## 6. Ports

`src/ports/` は外部依存への interface を置く。

- `agent_adapter/`: agent-specific adapter の interface。
- `session_source_reader/`: session artifact / DB を読む interface。
- `context_source_reader/`: instruction / config / capability source を読む interface。
- `token_estimator/`: token estimation の interface。
- `file_preview_reader/`: source preview の interface。
- `editor_launcher/`: editor open の interface。
- `exporter/`: export の interface。

Rule:

- trait は小さく保つ。
- 巨大な God trait を作らない。
- test double を作りやすくする。

---

## 7. Adapters

`src/adapters/` は agent-specific な実装を置く。

- `codex/`: Codex adapter。
- `claude_code/`: Claude Code adapter。
- `cursor/`: Cursor adapter。
- `gemini/`: Gemini CLI adapter。

### Codex Adapter

- `discovery/`: Codex home、session artifact、config、DB path などを発見する。
- `raw/`: Codex artifact に近い raw schema を置く。
- `parsers/`: rollout JSONL、session metadata、response item などを parse する。
- `classifiers/`: raw event / input text を context category に分類する。
- `mappers/`: raw / classified data を domain model に変換する。
- `graph/`: child session / delegated session / forked session の関係を復元する。

Rule:

- agent-specific file name / marker / DB schema は adapter 内に閉じる。
- raw schema と domain model を分ける。
- parser、classifier、mapper、graph builder を混ぜない。

---

## 8. Infrastructure

`src/infra/` は concrete IO を置く。

- `fs/`: file read、directory scan、metadata access。
- `json/`: JSON / JSONL read helper。
- `toml/`: TOML read helper。
- `sqlite/`: SQLite read-only access。
- `editor/`: source file を editor で開く処理。

Rule:

- infra は agent-specific な意味づけをしない。
- source mutation はしない。
- SQLite は read-only mode で開く。

---

## 9. Presentation

### TUI

`src/tui/` は ratatui による terminal UI を置く。

- `app/`: TUI application lifecycle。
- `state/`: UI state。
- `layout/`: terminal layout。
- `widgets/`: header、summary、accordion、selector、preview など。

Rule:

- TUI は parser を直接呼ばない。
- TUI は application の view model を描画する。
- business logic を持たない。

### CLI

`src/cli/` は CLI command を置く。

- `commands/inspect/`: session / context inspection。
- `commands/export/`: JSON export。
- `commands/doctor/`: environment check。
- `commands/compare/`: comparison command。

Rule:

- CLI は command dispatch に集中する。
- parser / adapter の内部実装を直接呼ばない。
- application usecase を呼ぶ。

---

## 10. Test Tree

```txt
tests/
  unit/
    domain/
    application/
    adapters/
      codex/
    infra/
  integration/
    codex/
      config_context/
      rollout_basic/
      rollout_with_instructions/
      rollout_with_skills/
      rollout_with_tools/
      rollout_with_subagent/
      rollout_with_compact/
      state_db/
    cli/
      inspect/
      export/
      doctor/
      compare/
    export/
      json_schema/
    session_graph/
      parent_child_link/
      forked_session/
      delegated_session/
      orphan_session/
    tui/
      view_model/
      smoke/
  fixtures/
    codex/
      config-only/
      rollout-basic/
      rollout-with-instructions/
      rollout-with-skills/
      rollout-with-tools/
      rollout-with-subagent/
      rollout-with-compact/
      state-db-basic/
      corrupted/
      missing-fields/
    claude-code/
      placeholder/
    cursor/
      placeholder/
```

---

## 11. Test Policy

- `tests/unit/`: 小さい module の仕様を検証する。
- `tests/integration/`: 複数 layer を通した behavior を検証する。
- `tests/fixtures/`: artificial な sample artifact を置く。

Rules:

- real user data を使わない。
- real Codex home に依存しない。
- fixture には secret / token / private path を入れない。
- snapshot test を使う場合は path や timestamp を安定化する。

---

## 12. Module Granularity Rules

- 1 module は 1 responsibility にする。
- parser、classifier、mapper、graph builder を混ぜない。
- raw schema と domain model を分ける。
- Codex 固有名は `src/adapters/codex/` に閉じる。
- application usecase は UI 都合を持たない。
- TUI widget は business logic を持たない。
- test しづらい module は分割する。Ï