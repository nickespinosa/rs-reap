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

## Compatibility

The crate uses Rust 2024 edition and requires Rust 1.85 or newer. The project
was developed against current stable Rust 1.97.1 (July 2026).

## License

Mozilla Public License 2.0. See [LICENSE](LICENSE).

