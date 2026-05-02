# ctx-analyzer

Local-first context analyzer for coding agents. Visualises what a coding-agent
session loaded into its context window — instructions, capabilities, runtime
events, delegated subagents — bucketed into agent-agnostic categories with
per-segment token estimates and provenance. Helps teams that wire up plugins,
connectors, app integrations, and MCP servers catch **duplicate or overlapping
loads** before context bloat pushes sessions against model or product limits.

The MVP ships with the **Codex** adapter; future adapters (Claude Code, Cursor,
Gemini CLI, OpenCode) plug in behind the same `AgentAdapter` port.

See:

- [`docs/PRD.md`](docs/PRD.md) — product, scope, success criteria
- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) — Clean Architecture layout
- [`docs/STRUCTURE.md`](docs/STRUCTURE.md) — repository placement rules
- [`docs/TECH.md`](docs/TECH.md) — tech stack, tooling, dev environment
- [`docs/UI.md`](docs/UI.md) — TUI design (deferred for MVP)
- [`docs/specs/`](docs/specs/) — adapter / schema / token / diagnostics specs

## Install (development)

Requires Rust 1.92 (pinned via `rust-toolchain.toml`). With Nix Flakes:

```sh
nix develop
just check
```

Without Nix:

```sh
rustup toolchain install 1.92
cargo build --release
```

## Usage

```sh
# Discover Codex rollout files under ~/.codex/sessions/
ctx-analyzer doctor

# Inspect a specific rollout: header + summary + accordion breakdown
ctx-analyzer inspect --file tests/fixtures/codex/rollout-with-subagent/ctx.jsonl

# Export the same session as agent-agnostic JSON (schema_version 0.1)
ctx-analyzer export --file tests/fixtures/codex/rollout-with-subagent/ctx.jsonl --out session.json
```

When `--file` is omitted, `ctx-analyzer` picks the most-recently-modified
rollout under `~/.codex/sessions/` (override with `CODEX_HOME`). When
`--session <id>` is provided, it filters by the session UUID embedded in the
rollout filename.

## Architecture (one-line sketch)

```
CLI / TUI -> Application (usecases, view-models)
              -> Domain (Session, Turn, ContextSegment, …)
              -> Ports (AgentAdapter, TokenEstimator, Exporter, …)
           -> Adapters (codex/{discovery,raw,parsers,classifiers,mappers,graph})
           -> Infra    (fs, json, toml, sqlite, editor, token)
```

`domain` is agent-agnostic; Codex-specific filenames and schema live exclusively
under `src/adapters/codex/` per the placement rules in `docs/STRUCTURE.md`.

## Development

```sh
just fmt         # rustfmt + prettier
just lint        # clippy -D warnings
just test        # cargo test --all-targets
just deny        # cargo deny check
just check       # all of the above (CI gate)

just demo-doctor   # run the binary against the bundled fixture
just demo-inspect
just demo-export
```

Pre-commit / pre-push hooks are managed via [lefthook](lefthook.yml). To enable:

```sh
lefthook install
```

## Status

MVP per `docs/PRD.md` §13.1: Codex adapter, session/turn reconstruction,
context breakdown across {System, Configuration, Runtime, Delegated, Unknown}
categories, token estimation (`ceil(chars/4)` per segment + observed totals
from final `token_count` event), agent-agnostic JSON export, source-file
provenance, reconstruction confidence. TUI, SQLite state-db ingestion,
cross-file child-session linking, comparison, diagnostics, and cost estimation
are deferred (see `docs/PRD.md` §8.2).

## License

MIT OR Apache-2.0 (see `Cargo.toml`).
