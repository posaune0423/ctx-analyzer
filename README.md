<div align="center">

# ctx-analyzer

**Local-first context analyzer for coding agents.**

<!-- <img src="docs/demo.gif" width="700" alt="ctx-analyzer rendering context breakdown" /> -->

[![CI](https://github.com/posaune0423/ctx-analyzer/actions/workflows/ci.yml/badge.svg)](https://github.com/posaune0423/ctx-analyzer/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.92%2B-orange.svg)](https://www.rust-lang.org/)

</div>

ctx-analyzer visualises what a coding-agent session loaded into its context window — instructions, capabilities, runtime events, delegated subagents — bucketed into agent-agnostic categories with per-segment token estimates and provenance. Helps teams that wire up plugins, connectors, app integrations, and MCP servers catch **duplicate or overlapping loads** before context bloat pushes sessions against model or product limits.

## FEATURES

- **Agent-agnostic categorization**: Breakdowns across System, Configuration, Runtime, Delegated, and Unknown categories
- **Token estimation**: Accurate `ceil(chars/4)` per segment + observed totals from final token events
- **JSON export**: Export agent-agnostic JSON (schema_version 0.1) for further analysis
- **Local-first privacy**: Inspects local agent artifacts read-only without sending data to external APIs
- **Adapter architecture**: MVP ships with the **Codex** adapter; future adapters (Claude Code, Cursor, Gemini CLI, OpenCode) plug in behind the same port

## QUICK START

Requires Rust 1.92 (pinned via `rust-toolchain.toml`).

With Nix Flakes:

```bash
nix develop
just check
```

Without Nix:

```bash
rustup toolchain install 1.92
cargo build --release
```

## USAGE

```bash
# Discover Codex rollout files under ~/.codex/sessions/
ctx-analyzer doctor

# Inspect a specific rollout: header + summary + accordion breakdown
ctx-analyzer inspect --file tests/fixtures/codex/rollout-with-subagent/ctx.jsonl

# Export the same session as agent-agnostic JSON
ctx-analyzer export --file tests/fixtures/codex/rollout-with-subagent/ctx.jsonl --out session.json
```

When `--file` is omitted, `ctx-analyzer` picks the most-recently-modified rollout under `~/.codex/sessions/` (override with `CODEX_HOME`). When `--session <id>` is provided, it filters by the session UUID embedded in the rollout filename.

## DEVELOPMENT

| Command       | Action                                                                     |
| :------------ | :------------------------------------------------------------------------- |
| `just fmt`    | rustfmt + prettier                                                         |
| `just lint`   | clippy -D warnings                                                         |
| `just test`   | cargo test --all-targets                                                   |
| `just deny`   | cargo deny check                                                           |
| `just check`  | all of the above (CI gate)                                                 |
| `just demo-*` | run the binary against the bundled fixture (`doctor`, `inspect`, `export`) |

Pre-commit / pre-push hooks are managed via [lefthook](lefthook.yml). To enable: `lefthook install`

## DOCUMENTATION

- `llm.txt` — Agent-oriented install & CLI summary
- Architecture & core concepts
- Adapter specs & token estimation

## CONTRIBUTING

See `docs/CONTRIBUTING.md`. Run `just check` before sending a PR.

## LICENSE

Released under the [MIT OR Apache-2.0 License](LICENSE).
