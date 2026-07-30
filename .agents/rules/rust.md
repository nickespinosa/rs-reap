---
paths:
  - "**/*.rs"
  - "Cargo.toml"
  - "rust-toolchain.toml"
  - "rustfmt.toml"
  - "clippy.toml"
  - "Makefile"
  - ".cargo/**"
  - ".editorconfig"
---

# Rust

- Edition 2024, MSRV 1.85, toolchain pin in `rust-toolchain.toml`.
- Prefer safe Rust; every `unsafe` block needs `// SAFETY:`.
- Keep public API minimal: `is_supported`, `reap_children`.
- Format via `rustfmt.toml` only (max_width **100**, airy width thresholds).
- Match style in `src/lib.rs`; no drive-by refactors or unsolicited comments.
- After changes: `make verify`.
