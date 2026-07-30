# rs-reap

`rs-reap` is a small Rust library for reaping abandoned child processes in
PID 1-style applications, such as processes running inside containers.

On supported Unix platforms, `reap_children` listens for `SIGCHLD` and drains
all waitable children with nonblocking `wait4` calls. It can report reaped PIDs,
surface unexpected wait errors, and coordinate with application subprocess
waits through an optional `Arc<RwLock<()>>`. Windows and Solaris provide a
safe no-op implementation.

```rust
use std::sync::{mpsc, Arc, RwLock};
use rs_reap::reap_children;

let (pids_tx, pids_rx) = mpsc::channel();
let (errors_tx, errors_rx) = mpsc::channel();
let (stop_tx, stop_rx) = mpsc::channel();
let reap_lock = Arc::new(RwLock::new(()));

std::thread::spawn(move || {
    reap_children(Some(pids_tx), Some(errors_tx), stop_rx, Some(reap_lock));
});

// Read PIDs from `pids_rx`; stop the reaper with `stop_tx.send(())`.
let _ = (pids_rx, errors_rx, stop_tx);
```

## Develop

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Toolchain is pinned in `rust-toolchain.toml` (Rust 1.97.1). Edition 2024,
MSRV 1.85.

## Agent configuration

Project instructions for coding agents live in [`AGENTS.md`](AGENTS.md).
Tool adapters inherit from that file and shared content under [`.agents/`](.agents/):

| Path | Role |
|------|------|
| `AGENTS.md` | Canonical instructions (Codex, OpenCode, Grok, Pi, …) |
| `CLAUDE.md` | Claude Code import of `AGENTS.md` + thin notes |
| `opencode.json` | OpenCode project config |
| `.agents/` | Shared rules, skills, subagent prompts |
| `.claude/` / `.opencode/` / `.grok/` / `.pi/` | Tool-specific adapters (symlinks + frontmatter) |

See [`.agents/README.md`](.agents/README.md) for the inheritance map.

## License

Mozilla Public License 2.0. See [LICENSE](LICENSE).

