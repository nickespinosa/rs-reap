# rs-reap

Rust library that reaps abandoned child processes for PID 1-style apps (containers).

Canonical instructions for coding agents. Tool-specific adapters inherit this file — do not duplicate project facts elsewhere.

## Commands

| Command                                     | Purpose                               |
| ------------------------------------------- | ------------------------------------- |
| `cargo test`                                | Run unit tests                        |
| `cargo clippy --all-targets -- -D warnings` | Lint (pedantic enabled in Cargo.toml) |
| `cargo fmt --check`                         | Format check                          |
| `cargo doc --no-deps`                       | Build API docs                        |
| `cargo build`                               | Debug build                           |

Verify order after code changes: `cargo fmt --check` → `cargo clippy --all-targets -- -D warnings` → `cargo test`.

Toolchain: `rust-toolchain.toml` pins Rust **1.97.1** with `rustfmt` + `clippy`. Edition **2024**, MSRV **1.85**.

## Layout

```text
src/lib.rs              Public API + Unix reaper + platform stubs
Cargo.toml              Crate metadata, deps, lints
rust-toolchain.toml     Pinned toolchain
AGENTS.md               Canonical agent instructions (this file)
.agents/                Shared rules, skills, agent prompts (source of truth)
.claude/                Claude Code adapter (inherits .agents + AGENTS.md)
.opencode/              OpenCode adapter
.grok/                  Grok Build adapter
.pi/                    Pi adapter
.github/                CI, templates, Dependabot
```

## Architecture

- **API surface:** `is_supported()`, `Pid`, and `reap_children(pids, errors, shutdown, reap_lock)`.
- **Unix (not Solaris):** `signal-hook` on `SIGCHLD`, nonblocking `wait4(-1, WNOHANG)`, optional `Arc<RwLock<()>>` write-lock while draining.
- **Windows / Solaris / non-Unix:** safe no-op (matches upstream `go-reap` behavior).
- **Coordination:** callers that `wait` on their own children should hold a **read** lock on `reap_lock` so the reaper does not steal the exit status.
- **Channels:** PID/error sends are intentionally blocking.

## Code style

- Prefer safe Rust; any `unsafe` needs a `// SAFETY:` justification on the block.
- `unsafe_op_in_unsafe_fn` is denied.
- Clippy `all` + `pedantic` are warnings in `Cargo.toml`; treat new pedantic hits as fix-or-justify.
- No drive-by refactors; no comments unless asked.
- Match existing naming and module structure in `src/lib.rs`.
- Do not expand the public API without an explicit request.

## Testing

- Tests live in `src/lib.rs` under `#[cfg(test)]`.
- Unix integration test spawns `sh -c "exit 0"` and asserts the reaper reports that PID.
- Unsupported platforms test the no-op path.
- Do not leave zombie processes; always shut down the reaper thread in tests.
- Add or update tests for behavior changes.

## Safety / platform notes

- Only Unix reaping path uses `libc::wait4`; keep `cfg` gates accurate.
- Never weaken `WNOHANG` or block forever inside the drain loop.
- Do not log or commit secrets. This crate has none today — keep it that way.
- MPL-2.0: preserve license headers/SPDX if added; do not relicense.

## Git / PR

- Conventional, concise commits focused on one change.
- Before finishing: run the verify order above and fix failures.
- Prefer feature branches; do not force-push shared default branch.
- PR description: what changed, why, how verified.

## Agent config inheritance

| Layer                                      | Role                                              |
| ------------------------------------------ | ------------------------------------------------- |
| `AGENTS.md`                                | Canonical project facts (all tools)               |
| `.agents/rules/`                           | Path-scoped rules (shared)                        |
| `.agents/skills/`                          | Agent Skills standard packages (shared)           |
| `.agents/prompts/`                         | Shared subagent bodies                            |
| `CLAUDE.md`                                | Imports `AGENTS.md`; Claude-only notes            |
| `opencode.json`                            | Points `instructions` at `AGENTS.md`              |
| `.claude/`, `.opencode/`, `.grok/`, `.pi/` | Thin tool adapters (symlinks or frontmatter only) |

Edit shared content under `.agents/` or `AGENTS.md`. Do not copy-paste the same rule into multiple tool trees.
