# Repository Guidelines

## Project Structure & Module Organization

`ctx-analyzer` is a Rust 2021 local-first context analyzer for coding agents. The binary entrypoint is `src/main.rs`; library code starts at `src/lib.rs`.

- `src/domain/`: agent-agnostic models for sessions, turns, context sources, tokens, and graphs.
- `src/application/`: use cases and view models. Keep workflow logic here, not in CLI or TUI layers.
- `src/ports/`: traits for adapters, exporters, token estimation, previews, and editors.
- `src/adapters/`: agent-specific implementations. Codex logic belongs under `src/adapters/codex/`.
- `src/infra/`: filesystem, JSON, TOML, SQLite, editor, and token utilities.
- `src/cli/commands/`: commands such as `inspect`, `export`, `doctor`, and `compare`.
- `tests/unit/`, `tests/integration/`, `tests/fixtures/`: explicit test targets and sanitized artifacts.
- `docs/`: PRD, architecture, structure, UI, and specs.
- `references/`: upstream research material only; do not treat it as product source.

## Build, Test, and Development Commands

Use `just` as the task runner:

- `just fmt`: format Rust and Markdown.
- `just fmt-check`: verify formatting without edits.
- `just lint`: run clippy with warnings denied.
- `just test`: run all test targets.
- `just test-fast`: use `cargo nextest` when available, otherwise `cargo test`.
- `just build`: build the release binary.
- `just check`: run format checks, lint, tests, and `cargo deny check`.
- `just demo-inspect`, `just demo-export`, `just demo-doctor`: run CLI flows against the Codex fixture.

## Coding Style & Naming Conventions

Follow Rust 2021 idioms. Use `snake_case` for modules, functions, and files; `PascalCase` for types and traits; `SCREAMING_SNAKE_CASE` for constants. Keep parser, classifier, mapper, and graph reconstruction separate. Do not leak Codex schemas into `domain` or `application`; map them through `src/adapters/codex/`.

## Testing Guidelines

Tests are nested by layer and wired explicitly in `Cargo.toml` using `[[test]]`. Add new nested test files there, or Cargo will not discover them. Use `assert_cmd` and `predicates` for CLI tests. Fixtures must be sanitized: no real home directories, tokens, private paths, or timestamp-dependent expectations.

## Commit & Pull Request Guidelines

This directory is not currently a Git repository, so no local history is available to infer a convention. Use short imperative subjects such as `Add Codex rollout parser`.

PRs should include the summary, affected commands or modules, verification output (`just check` when possible), fixture/schema changes, and screenshots only for UI/TUI layout changes.

## Security & Configuration Tips

The tool inspects local agent artifacts read-only. Avoid persisting prompts, copied source code, private reports, or raw session data unless the user explicitly exports them.
