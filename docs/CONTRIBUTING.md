# Contributing to ctx-analyzer

Thank you for your interest in contributing!

## Development Setup

Requires Rust 1.92 (pinned via `rust-toolchain.toml`).

```bash
# Using Nix
nix develop

# Build and check
just check
```

## Pull Request Process

1. Ensure `just check` passes locally.
2. Follow the architectural guidelines in `docs/ARCHITECTURE.md` and `docs/STRUCTURE.md`.
3. If adding a new agent adapter, follow the specs in `docs/specs/`.
4. Run `just fmt` before committing.

## Commands

| Command      | Action                                     |
| :----------- | :----------------------------------------- |
| `just fmt`   | Format code (rustfmt + prettier)           |
| `just lint`  | Run clippy                                 |
| `just test`  | Run tests                                  |
| `just check` | Full CI-gate check (fmt, lint, test, deny) |

Released under the [MIT OR Apache-2.0 License](../LICENSE).
