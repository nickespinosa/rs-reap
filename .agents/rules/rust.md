---
paths:
  - "**/*.rs"
  - "Cargo.toml"
  - "rust-toolchain.toml"
---

# Rust

- Edition 2024, MSRV 1.85, toolchain pin in `rust-toolchain.toml`.
- Prefer safe Rust; every `unsafe` block needs `// SAFETY:`.
- Keep public API minimal: `is_supported`, `reap_children`.
- Match style in `src/lib.rs`; no drive-by refactors or unsolicited comments.
- After changes: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
