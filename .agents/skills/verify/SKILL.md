---
name: verify
description: Run the rs-reap verify suite (fmt, clippy, test). Use after code changes or before commit/PR.
disable-model-invocation: false
---

# Verify rs-reap

Run from the repository root:

```bash
make verify
```

Full CI parity (adds docs):

```bash
make ci
```

Equivalents without Make:

```bash
cargo fmt-check
cargo lint
cargo test
cargo docs   # optional
```

Style knobs: `rustfmt.toml` (100 cols), `clippy.toml`, `.editorconfig`.

Report pass/fail per step. Fix failures before finishing.
