---
paths:
  - "src/**/*.rs"
---

# Process reaping safety

- Drain with `wait4(-1, …, WNOHANG, …)` only — never block in the drain loop.
- Preserve `cfg` gates: Unix-not-Solaris vs Windows/Solaris/non-Unix no-op.
- `reap_lock` write-lock only while draining; document read-lock for callers that wait their own children.
- Channel sends stay blocking (upstream `go-reap` semantics).
- Tests must shut down the reaper thread; no leftover zombies.
