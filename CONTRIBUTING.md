# Contributing

## Setup

```bash
git clone https://github.com/nickespinosa/rs-reap.git
cd rs-reap
cargo test
```

Rust **1.97.1** is pinned via `rust-toolchain.toml` (MSRV **1.85**).

## Verify before opening a PR

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Guidelines

- Keep the public API minimal (`is_supported`, `reap_children`) unless the change is requested.
- Prefer small, focused commits (conventional style is welcome).
- Match existing code style in `src/lib.rs`.
- Agents and humans: project instructions live in [`AGENTS.md`](AGENTS.md).

## License

By contributing, you agree that your contributions are licensed under the
Mozilla Public License 2.0 (see [LICENSE](LICENSE)).
