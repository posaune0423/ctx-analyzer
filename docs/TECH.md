了解です。TECH.md はこのくらいの薄さでよいです。設計詳細は ARCHITECTURE.md / SPEC.md / STRUCTURE.md に逃がします。

# ctx-analyzer TECH

## 1. Overview

`ctx-analyzer` は、各 coding agent が保存している config / instruction sources / session artifacts / local database を read-only に解析する local-first tool です。
最初は Codex adapter から実装しますが、tool 全体は agent-agnostic に設計します。
この tool 自体は基本的に stateless に動作し、独自に session data や source code を保存しません。

---

## 2. Tech Stack

- Language: Rust
- TUI: ratatui
- Terminal backend: crossterm
- CLI: clap
- Serialization: serde / serde_json
- Config parsing: toml
- SQLite read support: rusqlite, if needed
- Dev environment: Nix Flakes
- Task runner: just
- Git hooks: lefthook

---

## 3. Architecture Policy

Clean Architecture + DDD を基本方針にします。
ただし、この document では詳細な domain model や module 構成には踏み込みません。
方針だけ定義します。

- core は agent-agnostic に保つ
- agent-specific な処理は adapter に閉じ込める
- UI / CLI は domain logic を直接持たない
- filesystem / sqlite / external command などの IO は infrastructure 側に閉じ込める
- 各 layer の詳細は別 document で定義する

---

## 4. Agent Adapter Policy

各 coding agent は context management と session storage が異なるため、agent-specific な処理は adapter として実装します。
Initial adapter:

- Codex
  Future adapters:
- Claude Code
- Cursor
- Gemini CLI
- OpenCode
  adapter は agent 固有の artifact を読み取り、共通の normalized model に変換します。
  agent-specific な file name や storage schema を core に漏らさない方針にします。

---

## 5. Stateless / Read-only Policy

`ctx-analyzer` は基本的に stateless です。
保持してよいもの:

- user preferences
- editor command
- known agent data source paths
- last selected agent
- lightweight cache metadata, if needed later
  保持しないもの:
- copied source code
- copied session artifacts
- copied prompts
- copied tool results
- private reports unless explicitly exported by the user
  agent artifact は read-only に扱います。

---

## 6. Development Environment

Nix Flakes で Rust toolchain と必要 package を管理します。
Nix で管理するもの:

- Rust toolchain
- rustfmt
- clippy
- rust-analyzer
- cargo-nextest
- prettier
- lefthook
- just
- sqlite package, if needed

---

## 7. Formatting / Linting / Testing

Formatting:

- Rust: rustfmt
- Markdown: prettier
  Linting:
- Rust: clippy
- spelling: VS Code Code Spell Checker
  Testing:
- cargo test
- cargo nextest, if needed
  Recommended commands:
- just fmt
- just lint
- just test
- just check

---

## 8. Git Hooks

Git hooks は lefthook で管理します。
Purpose:

- commit 前に format / lint を実行する
- push 前に test / check を実行する
- local と CI の品質基準を揃える
  想定 hook:
- pre-commit: format check, lint
- pre-push: test, full check
  具体的な hook 定義は `lefthook.yml` に記述します。
