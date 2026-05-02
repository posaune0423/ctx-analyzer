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
  rust-toolchain.toml
  docs/
    PRD.md
    TECH.md
    STRUCTURE.md
    UI.md
    sample/
    development/
    specs/
      codex.md
      session-graph.md
      token-estimation.md
      export-json.md
      diagnostics.md
  references/
    codex/
    claude-devtools/
  src/
    main.rs
    domain/
    application/
    ports/
    adapters/
      codex/
      claude_code/
      cursor/
      gemini/
    infra/
      fs/
      sqlite/
      json/
      toml/
      editor/
    tui/
      widgets/
    cli/
      commands/
  tests/
    unit/
      domain/
      application/
      adapters/
      infra/
    integration/
      codex/
      cli/
      export/
      session_graph/
    fixtures/
      codex/
      claude-code/
      cursor/
  .github/
    workflows/
      ci.yml
```

## Root Files

- `README.md`: project の概要、install、基本 usage、現在の制限を書く。
- `Cargo.toml`: Rust package / dependency / binary 設定を書く。
- `Cargo.lock`: dependency lock file。
- `flake.nix`: Nix development environment を定義する。
- `flake.lock`: Nix flake dependency lock file。
- `justfile`: format、lint、test、dev などの task command を定義する。
- `lefthook.yml`: pre-commit / pre-push hook を定義する。
- `deny.toml`: cargo-deny による license / advisory / duplicate dependency / banned crate policy を定義する。
- `rust-toolchain.toml`: Rust toolchain version を固定する。

## Docs

- `docs/PRD.md`: product の目的、課題、MVP scope、success criteria を書く。
- `docs/TECH.md`: 使用技術、architecture policy、tooling 方針を書く。
- `docs/STRUCTURE.md`: repository / folder / code 配置方針を書く。
- `docs/UI.md`: TUI layout、navigation、keybinding、preview などの UI 仕様を書く。
- `docs/sample/`: ドキュメント用の**実データ**を置く。Codex など各 agent の**実際の session 用ファイル**（ログ、transcript、設定断片など）を、再現手順の説明とあわせて置いてよい。公開リポジトリに載せる場合は**トークン・パス・個人情報を除去**すること。
- `docs/development/`: 開発者向けのセットアップ、デバッグ、コントリビューション手順などを置く。
- `docs/specs/`: PRD / TECH / UI に入れると重い詳細仕様を書く。

## References

- `references/`: 上流プロダクトの**実装がそのまま**入る参照用ツリー（サブモジュール、vendor clone など）。ctx-analyzer の `src/` とは切り離し、**ビルド成果物・crate にバンドルしない**。リポジトリに置く場合は**各プロジェクトのライセンス**に従う。一般に「本プロダクトの差分」とは別扱いにし、必要なときだけ実装を読みにいく。
- `references/codex/`: OpenAI Codex（coding agent）の **OSS 本体**。session / turn などの**永続化形式・読み取り経路はここを直接読む**（adapter や仕様を書く段階で都度参照する想定）。
- `references/claude-devtools/`: Claude Code 向けの **Web UI** ツール。`~/.claude/` など既存ログ・transcript から**コンテキストやセッションを再構成する**処理が ctx-analyzer と近い関心を持つ。**context 取得・パース・表示まわりの実装参照用**（TUI ではなくブラウザ UI だが、データソースと解釈は流用のヒントになる）。

### 参照の使い分け（メモ）

- **Codex**: エージェント実装が OSS のため、**どのように session / turn データを保存しているか**は `references/codex/` を追うのが最短。仕様だけで足りないときにソースを読む。
- **Claude Code**: **claude-devtools** がローカルに既にあるログから詳細を復元する。Claude Code adapter やコンテキスト分解で行き詰まったら、同ツリーの読み取り・スキャン・renderer まわりを参照する。

## Specs

- `docs/specs/codex.md`: Codex adapter の詳細仕様を書く。
- `docs/specs/session-graph.md`: parent / child session や delegated context の扱いを書く。
- `docs/specs/token-estimation.md`: token estimation の詳細仕様を書く。
- `docs/specs/export-json.md`: JSON export schema の詳細仕様を書く。
- `docs/specs/diagnostics.md`: warning / diagnostics / optimization suggestion の仕様を書く。

## Source Code

- `src/main.rs`: application entrypoint を置く。
- `src/domain/`: Workspace、Agent、Session、SessionGraph、Turn、ContextSource、ContextSegment などの core model を置く。
- `src/application/`: workspace analysis、session discovery、context breakdown、export などの usecase を置く。
- `src/ports/`: AgentAdapter、TokenEstimator、FilePreviewReader、Exporter など外部依存への interface を置く。
- `src/adapters/`: Codex / Claude Code / Cursor / Gemini など agent-specific な実装を置く。
- `src/infra/`: filesystem、SQLite、JSON、TOML、editor launcher など concrete IO 実装を置く。
- `src/tui/`: ratatui による terminal UI 実装を置く。
- `src/cli/`: CLI command parsing と command execution を置く。

## Adapters

- `src/adapters/codex/`: Codex の config、instruction sources、session artifacts、local DB などを normalized model に変換する実装を置く。
- `src/adapters/claude_code/`: Claude Code 対応用の adapter 実装を置く。
- `src/adapters/cursor/`: Cursor 対応用の adapter 実装を置く。
- `src/adapters/gemini/`: Gemini CLI 対応用の adapter 実装を置く。

### 上流ツールの対話ログ（transcript）の保存場所

adapter 実装やローカル検証のとき、**実際の対話履歴**は次のようなパスに置かれる（OS・Cursor / Codex の版により細部は変わりうる）。

- **Cursor**
  - ベース: `~/.cursor/projects/<workspace-derived-id>/agent-transcripts/`
  - ファイル: `<session-uuid>/<session-uuid>.jsonl`（セッション ID 名のディレクトリ配下に同名の JSONL）
  - `<workspace-derived-id>` はワークスペースのパスなどから決まる識別子（例: パス区切りを `-` へ寄せたような文字列）
  - 例: `~/.cursor/projects/Users-example-Work-myrepo/agent-transcripts/bbef8396-1f07-403c-9cdc-1a9c4b28892e/bbef8396-1f07-403c-9cdc-1a9c4b28892e.jsonl`
- **Codex（OpenAI Codex CLI の rollout ログ）**
  - ベース: `~/.codex/sessions/`
  - 階層: `YYYY/MM/DD/`（日付ディレクトリ）
  - ファイル: `rollout-<ISO-風タイムスタンプ>-<uuid>.jsonl`
  - 例: `~/.codex/sessions/2026/04/10/rollout-2026-04-10T00-54-43-019d72f3-f339-71c3-810e-f714359dcdff.jsonl`

公開リポジトリや `docs/samples/` にコピーする場合は、**トークン・個人パス・識別子**をマスクすること（`docs/sample/` の方針と同様）。

## Infra

- `src/infra/fs/`: file read、directory scan、file metadata access を置く。
- `src/infra/sqlite/`: SQLite read-only access を置く。
- `src/infra/json/`: JSON / JSONL parsing を置く。
- `src/infra/toml/`: TOML parsing を置く。
- `src/infra/editor/`: source file を editor で開く処理を置く。

## TUI

- `src/tui/`: TUI application state、event loop、layout を置く。
- `src/tui/widgets/`: header、summary、accordion、selector、preview modal などの TUI component を置く。

## CLI

- `src/cli/`: CLI bootstrap と command routing を置く。
- `src/cli/commands/`: inspect、export、doctor、compare などの subcommand を置く。

## Tests

- `tests/unit/`: domain、application、adapter helper、infra utility など小さい単位の test を置く。
- `tests/unit/domain/`: core model や value object の test を置く。
- `tests/unit/application/`: usecase と context breakdown logic の test を置く。
- `tests/unit/adapters/`: adapter 内の parser / mapper の unit test を置く。
- `tests/unit/infra/`: filesystem、JSON、TOML、SQLite reader など IO wrapper の test を置く。
- `tests/integration/`: 複数 layer を通した end-to-end 寄りの test を置く。
- `tests/integration/codex/`: fixture を使った Codex adapter 全体の integration test を置く。
- `tests/integration/cli/`: CLI command の integration test を置く。
- `tests/integration/export/`: JSON export の integration test を置く。
- `tests/integration/session_graph/`: parent / child session reconstruction の integration test を置く。
- `tests/fixtures/`: adapter / parser / integration test 用の sample artifacts を置く。

## CI

- `.github/workflows/ci.yml`: format、lint、test、build、cargo-deny の CI workflow を置く。

## Placement Rules

- project documentation は `docs/` に置く。
- ドキュメント用の**実セッション断片・再現用ファイル**は `docs/sample/` に置く（秘密情報はマスクする）。
- 開発・デバッグ・コントリビューション手順は `docs/development/` に置く。
- detailed specs は `docs/specs/` に置く。
- 上流の**フル実装**は `references/` に置く（本パッケージのソースではない。ライセンス遵守・バンドル対象外）。
- production code は `src/` 以下に置く。
- test code は `tests/` 以下に置く。
- test fixtures は `tests/fixtures/` 以下に置く。
- agent に依存しない domain / usecase / ports は `src/domain/`、`src/application/`、`src/ports/` に置く。
- agent 固有の file path / DB schema / parser は `src/adapters/` に置く。
- filesystem / SQLite / parser / editor command などの concrete IO は `src/infra/` に置く。
- terminal rendering と key handling は `src/tui/` に置く。
- CLI command と process bootstrap は `src/cli/` に置く。
