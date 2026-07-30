---
name: review
description: Review the current diff for rs-reap correctness, platform safety, and AGENTS.md compliance.
disable-model-invocation: false
---

# Review rs-reap changes

## Diff

!`git diff HEAD`

## Status

!`git status -sb`

## Checklist

1. Public API unchanged unless requested (`is_supported`, `reap_children`).
2. `unsafe` blocks have `// SAFETY:` comments.
3. Drain loop stays nonblocking (`WNOHANG`); no infinite block on shutdown path beyond intended polling.
4. Platform `cfg` gates correct for Unix-not-Solaris vs stubs.
5. Tests shut down reaper threads; no zombies.
6. `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` green (or note why not run).

## Output

- Ship / ship-with-nits / block
- Findings with severity and concrete fixes
