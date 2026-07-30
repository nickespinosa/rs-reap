---
name: verify
description: Run the rs-reap verify suite (fmt, clippy, test). Use after code changes or before commit/PR.
disable-model-invocation: false
---

# Verify rs-reap

Run from the repository root:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Optional:

```bash
cargo doc --no-deps
cargo build
```

Report pass/fail per command. Fix failures before finishing the task.
