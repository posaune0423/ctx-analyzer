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
  -> Constants / Utils  (cross-cutting, sink-only)
```

依存方向は内側に向ける。

- `domain` は他 layer に依存しない。
- `application` は `domain` と `ports` に依存する。
- `ports` は interface を定義する。
- `adapters` は agent-specific な実装を持つ。
- `infra` は filesystem / SQLite / JSON / TOML / editor などの concrete IO を持つ。
- `cli` / `tui` は user-facing な presentation を持つ。
- `constants` / `utils` はどの layer からも参照可能。逆向きの依存は持たない（純粋値・純粋関数のみ）。

---

## 3. Source Tree

```txt
src/
  main.rs
  lib.rs
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
    token/
  constants/
    schema/         # SCHEMA_VERSION, AGENT_KEY_*
    preview/        # PREVIEW_LIMIT_CHARS, LABEL_TRUNCATE_CHARS
    severity/       # SEVERITY_MEDIUM/HIGH/CRITICAL + symbols
    codex/          # CODEX_HOME_ENV, CODEX_ROLLOUT_GLOB,
                    # DEVELOPER_BLOCK_TAGS, PROJECT_DOC_PREFIX, …
  utils/
    text/           # truncate_chars (char-bounded ellipsis)
    format/         # fmt_thousands, opt_thousands
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
- `constants/codex` には依存してよいが、その逆（constants が domain に依存）は不可。

---

## 5. Application

`src/application/` は usecase と view model を置く。

- `usecases/analyze_workspace/`: workspace 全体の解析を行う。
- `usecases/detect_agents/`: 利用可能な coding agent を検出する。
- `usecases/discover_sessions/`: session 一覧を取得する。
- `usecases/reconstruct_session_graph/`: session 間の parent / child relationship を復元する。
- `usecases/build_context_breakdown/`: context source / segment を分類・集計する。
- `usecases/estimate_tokens/`: token estimate を付与する（default は infra/token の `CharsPer4Estimator`）。
- `usecases/export_json/`: JSON export を生成する。
- `usecases/compare/`: agent / session / turn の比較を行う。
- `view_models/`: CLI / TUI が表示しやすい形の model を置く。`view_models/context_breakdown/text_render.rs` のように "view-model + その renderer" を同じ場所に置いてよい（CLI / TUI 共有）。

Rule:

- agent-specific parser を直接呼ばない。
- filesystem / SQLite などの concrete IO を直接呼ばない。
- adapter / infra には `ports` 経由でアクセスする。

---

## 6. Ports

`src/ports/` は外部依存への interface を置く。

- `agent_adapter/`: agent-specific adapter の interface。
- `session_source_reader/`: session artifact / DB を読む interface（post-MVP）。
- `context_source_reader/`: instruction / config / capability source を読む interface（post-MVP）。
- `token_estimator/`: token estimation の interface。
- `file_preview_reader/`: source preview の interface（post-MVP）。
- `editor_launcher/`: editor open の interface（post-MVP）。
- `exporter/`: export の interface。

Rule:

- trait は小さく保つ。
- 巨大な God trait を作らない。
- test double を作りやすくする。

---

## 7. Adapters

`src/adapters/` は agent-specific な実装を置く。

- `codex/`: Codex adapter。
- `claude_code/`: Claude Code adapter（post-MVP placeholder）。
- `cursor/`: Cursor adapter（post-MVP placeholder）。
- `gemini/`: Gemini CLI adapter（post-MVP placeholder）。

### Codex Adapter

- `discovery/`: Codex home、session artifact、config、DB path などを発見する。
  filesystem 文字列はすべて `crate::constants::codex` から import し、ハードコードしない。
- `raw/`: Codex artifact に近い raw schema を置く。
  `references/codex/codex-rs/rollout/src/recorder.rs` の `RolloutItem` 列挙に
  対応する: `SessionMeta | ResponseItem | TurnContext | EventMsg | Compacted`。
  未知 variant は `#[serde(other)] Other` で前方互換にする。
- `parsers/`: rollout JSONL、session metadata、response item などを parse する。
  失敗行は `ParseWarning` に積んで partial result を返す。
- `classifiers/`: raw event / input text を context category に分類する。
  developer-message の inline タグ（`<permissions instructions>` 等）は
  `constants::codex::DEVELOPER_BLOCK_TAGS` の table を 1 つ参照する。
- `mappers/`: raw / classified data を domain model に変換する。
  preview の char 上限は `constants::preview::PREVIEW_LIMIT_CHARS`、truncation は
  `utils::text::truncate_chars` を呼ぶ。
- `graph/`: child session / delegated session / forked session の関係を復元する。
  MVP は `source.subagent` の有無のみ。cross-file linking は post-MVP
  （`docs/specs/session-graph.md` 参照）。

References から得た知見（`docs/specs/codex.md` で詳述）:

- `~/.codex/sessions/YYYY/MM/DD/rollout-<ts>-<uuid>.jsonl`（厳密な日付ツリー）
- `~/.codex/session_index.jsonl` を **末尾から逆順に走査して "last entry wins"**
  で `ThreadId → human label` を解決する（`session_index.rs:28-49`）
- `~/.codex/state.db` は SQLite cache（filesystem scan の高速化）。
  fallback ではなく optimization。`infra/sqlite/` を将来 read-only で開く。
- `RolloutItem::Compacted { replacement_history: [...] }` は `event_msg` の
  variant ではなく **top-level** の variant。compaction 解析の起点になる
  （`docs/specs/diagnostics.md` の future scope）。

Rule:

- agent-specific file name / marker / DB schema は adapter 内に閉じる。
  どうしても外に出す場合は `constants/codex/` に集約する。
- raw schema と domain model を分ける。
- parser、classifier、mapper、graph builder を混ぜない。

### Claude Code Adapter（post-MVP・参考）

`references/claude-devtools/` から得たパターン:

- `~/.claude/projects/<encoded-cwd>/<session-uuid>.jsonl`。
  `encoded-cwd` は `path.replace(/[/\\]/g, '-')` + leading dash。dash を含む
  パスは lossy なので、JSONL 内の `cwd` を信頼する。
- `services/discovery/SubagentResolver.ts` の処理は post-MVP の
  `usecases/reconstruct_session_graph/` に移植できる:
  1. `<sessionId>/subagents/agent-<id>.jsonl` を列挙
  2. parent の Task tool_use を `sourceToolUseID` で結ぶ
  3. start/end 時刻が 100 ms 以内に重なる subagent は parallel と判定
- 5 分類: `user / system / hardNoise / compact / ai`。
  わたしたちの 5 分類（System / Configuration / Runtime / Delegated / Unknown）
  と直交ではないので、Claude Code adapter の `mappers/` で `hardNoise`
  （`<system-reminder>`, `<local-command-caveat>` 等）を `Unknown` に流す。

---

## 8. Infrastructure

`src/infra/` は concrete IO を置く。

- `fs/`: file read、directory scan、metadata access。
- `json/`: JSON / JSONL read helper。
- `toml/`: TOML read helper。
- `sqlite/`: SQLite read-only access（`SQLITE_OPEN_READONLY` 必須）。
- `editor/`: source file を editor で開く処理。
- `token/`: `TokenEstimator` port の concrete impl。MVP では `CharsPer4Estimator`。

Rule:

- infra は agent-specific な意味づけをしない。
- source mutation はしない。
- SQLite は read-only mode で開く。

---

## 9. Constants

`src/constants/` は **値のみ** を置く（型・関数を入れない例外: `DeveloperBlockTag` のような構造化された const 表のみ）。

- `schema/`: JSON export schema 関連（`SCHEMA_VERSION`, `AGENT_KEY_CODEX` …）
- `preview/`: text 表示の char 上限（`PREVIEW_LIMIT_CHARS`, `LABEL_TRUNCATE_CHARS`）
- `severity/`: token severity の閾値と symbol（`SEVERITY_MEDIUM/HIGH/CRITICAL`, `SYMBOL_HIGH/CRITICAL/UNKNOWN`）
- `codex/`: Codex 固有の filesystem / inline marker（`CODEX_HOME_ENV`, `CODEX_ROLLOUT_GLOB`, `CODEX_SESSION_INDEX_FILE`, `CODEX_STATE_DB_FILE`, `PROJECT_DOC_PREFIX`, `DEVELOPER_BLOCK_TAGS` …）

Rule:

- 他 layer に依存しない（`use crate::xxx` は domain の純粋型に限る）。
- 同じ値が 2 箇所以上に書かれていたら、まずここに引き上げる。
- adapter 固有の constants は `constants/<agent>/` に置く。

## 10. Utils

`src/utils/` は **副作用なしの汎用関数** を置く。

- `text/`: char 単位の text 操作（`truncate_chars`）
- `format/`: 数値 / 日付の整形（`fmt_thousands`, `opt_thousands`）

Rule:

- 純粋関数のみ。IO・グローバル状態に触らない。
- domain / adapter / infra への依存を持たない。
- 関数 1 つに対して unit test を書く。

---

## 11. Presentation

### TUI

`src/tui/` は ratatui による terminal UI を置く（post-MVP）。

- `app/`: TUI application lifecycle。
- `state/`: UI state。
- `layout/`: terminal layout。
- `widgets/`: header、summary、accordion、selector、preview など。

Rule:

- TUI は parser を直接呼ばない。
- TUI は application の view model を描画する。
  `view_models/context_breakdown/text_render.rs` に書いたような renderer は
  CLI / TUI で共有する（`Write` trait 経由）。
- business logic を持たない。

### CLI

`src/cli/` は CLI command を置く。

- `commands/inspect/`: session / context inspection。
- `commands/export/`: JSON export。
- `commands/doctor/`: environment check。
- `commands/compare/`: comparison command（post-MVP placeholder）。

Rule:

- CLI は command dispatch に集中する。
- parser / adapter の内部実装を直接呼ばない。
- application usecase を呼ぶ。

---

## 12. Test Tree

```txt
tests/
  unit/
    domain/
      session_graph.rs
      context_category.rs
    application/
      build_context_breakdown.rs
    adapters/
      codex/
        parsers.rs
        classifiers.rs
    infra/
      token_heuristic.rs
    constants/
      schema.rs
      severity.rs
      codex.rs
    utils/
      text.rs
      format.rs
  integration/
    codex/
      rollout_with_subagent.rs
    cli/
      doctor.rs
      inspect.rs
      export.rs
    export/
      json_schema.rs
    session_graph/
      delegated_session.rs
  fixtures/
    codex/
      rollout-with-subagent/
        ctx.jsonl
      malformed/        # for parser warnings (post-MVP)
    claude-code/        # post-MVP
    cursor/             # post-MVP
```

Cargo は `tests/<file>.rs` だけを自動 discover するため、上記 nested layout
は `Cargo.toml` の `[[test]]` entry で 1 件ずつ明示的に declare する。

## 13. Test Policy

- `tests/unit/`: 小さい module の仕様を検証する。
- `tests/integration/`: 複数 layer を通した behavior を検証する。
- `tests/fixtures/`: artificial な sample artifact を置く。

Rules:

- real user data を使わない。
- real Codex home に依存しない。fixture は `tests/fixtures/codex/<scenario>/`
  に置き、 absolute path / token / private path を含めない。
- snapshot test を使う場合は path や timestamp を安定化する。
- TDD: production code を書く前に必ず 1 つ failing test を見せる。
  `RED → verify-RED → GREEN → verify-GREEN → REFACTOR` を逸脱しない。

---

## 14. Module Granularity Rules

- 1 module は 1 responsibility にする。
- parser、classifier、mapper、graph builder を混ぜない。
- raw schema と domain model を分ける。
- Codex 固有名は `src/adapters/codex/` か `src/constants/codex/` に閉じる
  （domain には絶対に出さない）。
- application usecase は UI 都合を持たない。
- TUI widget は business logic を持たない。
- test しづらい module は分割する。
- 同じ literal / magic number を 2 箇所以上に書いたら `constants/` に上げる。
- 同じ utility を 2 箇所以上に書いたら `utils/` に上げる。
