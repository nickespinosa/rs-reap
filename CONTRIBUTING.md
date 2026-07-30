# Contributing

This repository is public so you can read it, learn from it, and fork it freely.
It does not accept pull requests or feature requests.

## Forks

If you want different behavior or API surface, fork the repo and make it yours.

## Bugs

Reproducible bugs are welcome via [GitHub issues](https://github.com/nickespinosa/rs-reap/issues/new/choose)
using the bug report template. Please include platform details and a minimal
reproduction when possible.

## Local development

```bash
git clone https://github.com/nickespinosa/rs-reap.git
cd rs-reap
make verify   # fmt-check + clippy + test
make ci       # verify + docs (matches GitHub Actions)
```

Rust **1.97.1** is pinned via `rust-toolchain.toml` (MSRV **1.85**).
Formatting is governed by `rustfmt.toml` (100-column max width) and
`.editorconfig`. Lint knobs live in `Cargo.toml` + `clippy.toml`.

Agents and humans: project instructions live in [`AGENTS.md`](AGENTS.md).

## License

This project is licensed under the Mozilla Public License 2.0
(see [LICENSE](LICENSE)).
