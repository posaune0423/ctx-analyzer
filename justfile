default: check

# Format all sources (Rust + Markdown)
fmt:
    cargo fmt --all
    -prettier --write "docs/**/*.md" "README.md"

# Format check (CI-friendly: no mutations)
fmt-check:
    cargo fmt --all -- --check
    -prettier --check "docs/**/*.md" "README.md"

# Run clippy with warnings-as-errors
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Run the full test suite
test:
    cargo test --all-targets --all-features

# Run with cargo-nextest if available, fall back to cargo test
test-fast:
    @if command -v cargo-nextest >/dev/null 2>&1; then \
        cargo nextest run --all-features; \
    else \
        cargo test --all-targets --all-features; \
    fi

# Run cargo-deny over license / advisory / duplicate / banned crates
deny:
    cargo deny check

# Build release binary
build:
    cargo build --release

# Full local quality gate: fmt-check + lint + test + deny
check: fmt-check lint test deny

# Run the binary against the bundled sample
demo-inspect:
    cargo run --quiet -- inspect --file tests/fixtures/codex/rollout-with-subagent/ctx.jsonl

demo-export:
    cargo run --quiet -- export --file tests/fixtures/codex/rollout-with-subagent/ctx.jsonl

demo-doctor:
    cargo run --quiet -- doctor --file tests/fixtures/codex/rollout-with-subagent/ctx.jsonl
