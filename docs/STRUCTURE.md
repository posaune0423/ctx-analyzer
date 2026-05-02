# ctx-analyzer STRUCTURE

## Repository Tree

```txt
ctx-analyzer/
  README.md
  Cargo.toml
  Cargo.lock
  flake.nix
  flake.lock
  justfile
  lefthook.yml
  deny.toml
  clippy.toml
  .taplo.toml
  rust-toolchain.toml
  docs/
    PRD.md
    TECH.md
    ARCHITECTURE.md
    STRUCTURE.md
    UI.md
    samples/                # docs 用の実セッション断片（マスク済）
    development/
    specs/
      codex.md
      session-graph.md
      token-estimation.md
      export-json.md
      diagnostics.md
  references/
    codex/                  # OpenAI Codex CLI 本体（Rust）
    claude-devtools/        # Claude Code 用 Web UI ツール（TS/Electron）
  src/
    main.rs
    lib.rs
    domain/                 # agent-agnostic core model
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
    ports/                  # 外部依存への trait
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
      claude_code/          # post-MVP placeholder
      cursor/               # post-MVP placeholder
      gemini/               # post-MVP placeholder
    infra/                  # concrete IO
      fs/
      json/
      toml/
      sqlite/
      editor/
      token/                # CharsPer4Estimator
    constants/              # 値だけ・他 layer に依存しない
      schema/
      preview/
      severity/
      codex/
    utils/                  # 純粋関数だけ・他 layer に依存しない
      text/
      format/
    tui/                    # post-MVP
      widgets/
    cli/
      commands/
        inspect/
        export/
        doctor/
        compare/
  tests/
    unit/
      domain/
      application/
      adapters/
        codex/
      infra/
      constants/
      utils/
    integration/
      codex/
      cli/
      export/
      session_graph/
    fixtures/
      codex/
        rollout-with-subagent/
        malformed/
      claude-code/          # post-MVP
      cursor/               # post-MVP
  .github/
    workflows/
      ci.yml
```

## Root Files

- `README.md`: project の概要、install、基本 usage、現在の制限を書く。
- `Cargo.toml`: Rust package / dependency / binary 設定を書く。Cargo は
  `tests/<file>.rs` しか自動 discover しないため、nested test path は
  `[[test]]` entry で明示的に declare する。
- `Cargo.lock`: dependency lock file。
- `flake.nix`: Nix development environment を定義する。
- `flake.lock`: Nix flake dependency lock file。
- `justfile`: format、lint、test、dev などの task command を定義する。
- `lefthook.yml`: pre-commit / pre-push hook を定義する。
- `deny.toml`: cargo-deny による license / advisory / duplicate dependency / banned crate policy を定義する。
- `clippy.toml`: clippy 設定（msrv 等）。
- `.taplo.toml`: taplo（TOML formatter）設定。
- `rust-toolchain.toml`: Rust toolchain version を固定する。

## Docs

- `docs/PRD.md`: product の目的、課題、MVP scope、success criteria を書く。
- `docs/TECH.md`: 使用技術、architecture policy、tooling 方針を書く。
- `docs/ARCHITECTURE.md`: layer 構成・依存方向・module 配置のルールを書く。
- `docs/STRUCTURE.md`: repository / folder / code 配置方針を書く。
- `docs/UI.md`: TUI layout、navigation、keybinding、preview などの UI 仕様を書く。
- `docs/samples/`: ドキュメント用の**実データ**を置く。Codex など各 agent の**実際の session 用ファイル**（ログ、transcript、設定断片など）を、再現手順の説明とあわせて置いてよい。公開リポジトリに載せる場合は**トークン・パス・個人情報を除去**すること。
- `docs/development/`: 開発者向けのセットアップ、デバッグ、コントリビューション手順などを置く。
- `docs/specs/`: PRD / TECH / UI に入れると重い詳細仕様を書く。

## References

- `references/`: 上流プロダクトの**実装がそのまま**入る参照用ツリー（サブモジュール、vendor clone など）。ctx-analyzer の `src/` とは切り離し、**ビルド成果物・crate にバンドルしない**。リポジトリに置く場合は**各プロジェクトのライセンス**に従う。
- `references/codex/`: OpenAI Codex（coding agent）の **OSS 本体**。session / turn などの**永続化形式・読み取り経路はここを直接読む**。
  - 重点ファイル:
    - `references/codex/codex-rs/rollout/src/recorder.rs` — rollout file の path 構築 / event 書き込み (`RolloutItem` 列挙)
    - `references/codex/codex-rs/rollout/src/session_index.rs` — `~/.codex/session_index.jsonl` の per-line shape (`{id, thread_name, updated_at}`) と "scan-from-EOF, last-entry-wins" semantics
    - `references/codex/codex-rs/core/state_db.rs` — `~/.codex/state.db` の schema（rusqlite, thread metadata + backfill state）
- `references/claude-devtools/`: Claude Code 向けの **Web UI** ツール。`~/.claude/` など既存ログ・transcript から**コンテキストやセッションを再構成する**処理が ctx-analyzer と近い関心を持つ。
  - 重点ファイル:
    - `src/main/services/discovery/ProjectScanner.ts` — `~/.claude/projects/<encoded-cwd>/` の発見と decode
    - `src/main/services/discovery/SubagentResolver.ts` — parent ↔ subagent の linking、`PARALLEL_WINDOW_MS = 100ms` で並列判定
    - `src/main/services/parsing/MessageClassifier.ts` — 5 分類 (`user / system / hardNoise / compact / ai`)
    - `src/main/utils/tokenizer.ts` — `Math.ceil(text.length / 4)`（わたしたちの `CharsPer4Estimator` と完全一致）

### 参照の使い分け（メモ）

- **Codex**: エージェント実装が OSS のため、**どのように session / turn データを保存しているか**は `references/codex/` を追うのが最短。`docs/specs/codex.md` の path / schema 記述はすべてここから fact-check する。
- **Claude Code**: **claude-devtools** がローカルに既にあるログから詳細を復元する。Claude Code adapter やコンテキスト分解で行き詰まったら、同ツリーの読み取り・スキャン・renderer まわりを参照する。post-MVP の `usecases/reconstruct_session_graph/` 設計はこの実装の `SubagentResolver` がベース。

## Specs

- `docs/specs/codex.md`: Codex adapter の詳細仕様を書く。
- `docs/specs/session-graph.md`: parent / child session や delegated context の扱いを書く。
- `docs/specs/token-estimation.md`: token estimation の詳細仕様を書く。
- `docs/specs/export-json.md`: JSON export schema の詳細仕様を書く。
- `docs/specs/diagnostics.md`: warning / diagnostics / optimization suggestion の仕様を書く。

## Source Code

- `src/main.rs`: application entrypoint を置く。
- `src/lib.rs`: integration test / 将来の TUI が application layer を呼ぶための公開 lib root。
- `src/domain/`: Workspace、Agent、Session、SessionGraph、Turn、ContextSource、ContextSegment などの core model を置く。
- `src/application/`: workspace analysis、session discovery、context breakdown、export などの usecase を置く。
- `src/ports/`: AgentAdapter、TokenEstimator、FilePreviewReader、Exporter など外部依存への interface を置く。
- `src/adapters/`: Codex / Claude Code / Cursor / Gemini など agent-specific な実装を置く。
- `src/infra/`: filesystem、SQLite、JSON、TOML、editor launcher など concrete IO 実装を置く。
- `src/constants/`: layer 横断で使う **値**（schema version、severity 閾値、Codex の filesystem marker など）を置く。
- `src/utils/`: layer 横断で使う **副作用なし関数**（`truncate_chars`、`fmt_thousands` など）を置く。
- `src/tui/`: ratatui による terminal UI 実装を置く（post-MVP）。
- `src/cli/`: CLI command parsing と command execution を置く。

## Adapters

- `src/adapters/codex/`: Codex の config、instruction sources、session artifacts、local DB などを normalized model に変換する実装を置く。
  - `discovery/` の path 文字列はすべて `src/constants/codex/` から import すること（ハードコード禁止）。
- `src/adapters/claude_code/`: Claude Code 対応用の adapter 実装を置く（post-MVP）。
- `src/adapters/cursor/`: Cursor 対応用の adapter 実装を置く（post-MVP）。
- `src/adapters/gemini/`: Gemini CLI 対応用の adapter 実装を置く（post-MVP）。

### 上流ツールの対話ログ（transcript）の保存場所

adapter 実装やローカル検証のとき、**実際の対話履歴**は次のようなパスに置かれる（OS・Cursor / Codex の版により細部は変わりうる）。

- **Codex（OpenAI Codex CLI の rollout ログ）**
  - 環境変数: `CODEX_HOME`（未設定時 `~/.codex`）
  - rollout: `<codex_home>/sessions/YYYY/MM/DD/rollout-<ISO 風 ts>-<thread_uuid>.jsonl`（**厳密な日付ツリー**。flat layout は無い）
  - 例: `~/.codex/sessions/2026/04/10/rollout-2026-04-10T00-54-43-019d72f3-f339-71c3-810e-f714359dcdff.jsonl`
  - thread name index: `<codex_home>/session_index.jsonl`（append-only、**last entry wins**）
  - SQLite cache: `<codex_home>/state.db`（read-only で開く。filesystem fallback の高速化用）
  - archived sessions: `<codex_home>/sessions-archived/`（flat layout）
- **Claude Code**
  - 環境変数: `CLAUDE_HOME`（未設定時 `~/.claude`）
  - transcripts: `~/.claude/projects/<encoded-cwd>/<session-uuid>.jsonl`
  - encoded-cwd: `path.replace(/[/\\]/g, '-')` + leading dash。dash を含むパスは
    lossy なので、JSONL 内の `cwd` を信頼する。
  - subagents: `~/.claude/projects/<encoded-cwd>/<session-uuid>/subagents/agent-<id>.jsonl`
  - parent ↔ subagent linking は `sourceToolUseID` field で結ぶ。
- **Cursor**
  - ベース: `~/.cursor/projects/<workspace-derived-id>/agent-transcripts/`
  - ファイル: `<session-uuid>/<session-uuid>.jsonl`
  - 例: `~/.cursor/projects/Users-example-Work-myrepo/agent-transcripts/bbef8396-1f07-403c-9cdc-1a9c4b28892e/bbef8396-1f07-403c-9cdc-1a9c4b28892e.jsonl`

公開リポジトリや `docs/samples/` にコピーする場合は、**トークン・個人パス・識別子**をマスクすること（`docs/samples/` の方針と同様）。

## Infra

- `src/infra/fs/`: file read、directory scan、file metadata access を置く。
- `src/infra/sqlite/`: SQLite read-only access を置く（Codex `state.db` 等）。
- `src/infra/json/`: JSON / JSONL parsing を置く。
- `src/infra/toml/`: TOML parsing を置く。
- `src/infra/editor/`: source file を editor で開く処理を置く。
- `src/infra/token/`: `TokenEstimator` port の concrete impl（MVP は `CharsPer4Estimator`）。

## Constants

- `src/constants/schema/`: JSON export schema の version / agent key
- `src/constants/preview/`: text preview / label の char 上限
- `src/constants/severity/`: token severity 閾値と symbol
- `src/constants/codex/`: Codex 固有の filesystem marker、inline タグ table

## Utils

- `src/utils/text/`: char 単位の text 操作（`truncate_chars` など）
- `src/utils/format/`: 数値 / 日付の整形（`fmt_thousands`, `opt_thousands` など）

## TUI

- `src/tui/`: TUI application state、event loop、layout を置く（post-MVP）。
- `src/tui/widgets/`: header、summary、accordion、selector、preview modal などの TUI component を置く。

## CLI

- `src/cli/`: CLI bootstrap と command routing を置く。
- `src/cli/commands/`: inspect、export、doctor、compare などの subcommand を置く。

## Tests

- `tests/unit/`: 小さい module の仕様を検証する。
  - `tests/unit/domain/`、`tests/unit/application/`、`tests/unit/adapters/codex/`、`tests/unit/infra/`、`tests/unit/constants/`、`tests/unit/utils/`
- `tests/integration/`: 複数 layer を通した end-to-end 寄りの test を置く。
  - `tests/integration/codex/`、`tests/integration/cli/`、`tests/integration/export/`、`tests/integration/session_graph/`
- `tests/fixtures/`: adapter / parser / integration test 用の sample artifacts を置く。
  - `tests/fixtures/codex/<scenario>/<file>.jsonl` という階層。
  - 既存: `rollout-with-subagent/`。今後追加予定:
    `rollout-basic/`, `rollout-with-instructions/`, `rollout-with-skills/`,
    `rollout-with-tools/`, `rollout-with-compact/`, `state-db-basic/`,
    `corrupted/`, `missing-fields/`。

Cargo は `tests/<file>.rs` だけを自動 discover するため、nested layout の各
test file は `Cargo.toml` の `[[test]]` entry で 1 件ずつ declare する。

## CI

- `.github/workflows/ci.yml`: format、lint、test、build、cargo-deny の CI workflow を置く。

## Placement Rules

- project documentation は `docs/` に置く。
- ドキュメント用の**実セッション断片・再現用ファイル**は `docs/samples/` に置く（秘密情報はマスクする）。
- 開発・デバッグ・コントリビューション手順は `docs/development/` に置く。
- detailed specs は `docs/specs/` に置く。
- 上流の**フル実装**は `references/` に置く（本パッケージのソースではない。ライセンス遵守・バンドル対象外）。
- production code は `src/` 以下に置く。
- test code は `tests/` 以下に置く。
- test fixtures は `tests/fixtures/` 以下に置く。fixture file は `tests/fixtures/<agent>/<scenario>/` 階層に整理する。
- agent に依存しない domain / usecase / ports は `src/domain/`、`src/application/`、`src/ports/` に置く。
- agent 固有の file path / DB schema / parser は `src/adapters/` に置く。
- agent 固有の **値 (path 文字列、tag literal など)** は `src/constants/<agent>/` に集約する。adapter 内でハードコードしない。
- filesystem / SQLite / parser / editor command などの concrete IO は `src/infra/` に置く。
- 副作用なしの汎用関数は `src/utils/` に置く。同じ helper を 2 箇所以上に書いたら必ず引き上げる。
- terminal rendering と key handling は `src/tui/` に置く。
- CLI command と process bootstrap は `src/cli/` に置く。
