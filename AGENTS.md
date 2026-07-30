# rs-reap

Rust library that reaps abandoned child processes for PID 1-style apps (containers).

Canonical instructions for coding agents. Tool-specific adapters inherit this file — do not duplicate project facts elsewhere.

## Commands

| Command        | Purpose                                      |
| -------------- | -------------------------------------------- |
| `make verify`  | fmt-check → clippy → test (default gate)     |
| `make ci`      | verify + docs (matches GitHub Actions)       |
| `make fmt`     | Apply rustfmt                                |
| `make lint`    | Clippy with `-D warnings`                    |
| `make test`    | Unit tests                                   |
| `cargo fmt-check` / `cargo lint` / `cargo docs` | Cargo aliases (see `.cargo/config.toml`) |

Style: `rustfmt.toml` (**max_width 100**, low width thresholds for airy breaks),
`.editorconfig` (100-col soft wrap), `clippy.toml` + `[lints]` in `Cargo.toml`.

Toolchain: `rust-toolchain.toml` pins Rust **1.97.1** with `rustfmt` + `clippy`.
Edition **2024**, MSRV **1.85**.

## Layout

```text
src/lib.rs              Public API + Unix reaper + platform stubs
Cargo.toml              Crate metadata, deps, lints
rustfmt.toml            Opinionated format (100 cols, vertical/airy)
clippy.toml             Complexity thresholds
.editorconfig           Cross-editor indent + max_line_length 100
Makefile                fmt / lint / test / verify / ci
.cargo/config.toml      cargo aliases (fmt-check, lint, docs)
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
- Format with `rustfmt.toml` only — do not hand-wrap against it. **100 columns**.
- Prefer vertical, airy layouts (rustfmt width thresholds ≈ 40) over dense one-liners.
- Clippy `all` + `pedantic` (+ a few explicit lints) warn in `Cargo.toml`; fix or justify.
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
- Before finishing: `make verify` (or `make ci`) and fix failures.
- Prefer feature branches; do not force-push shared default branch.
- PR description: what changed, why, how verified.

## Agent config inheritance

| Layer                                      | Role                                                                 |
| ------------------------------------------ | -------------------------------------------------------------------- |
| `AGENTS.md`                                | Canonical project facts (all tools)                                  |
| `.agents/rules/`                           | Path-scoped rules (shared)                                           |
| `.agents/skills/`                          | Agent Skills packages (shared)                                       |
| `.agents/prompts/`                         | Shared subagent bodies (+ name/description)                          |
| `CLAUDE.md`                                | `@AGENTS.md` + Claude-only notes                                     |
| `opencode.json`                            | `instructions` + `agent.*.prompt` `{file:./.agents/prompts/*}`       |
| `.claude/rules`, `skills`                  | Symlinks → `.agents/{rules,skills}`                                  |
| `.claude/agents/*`                         | Tool frontmatter + `@.agents/prompts/*`                              |
| `.grok/{rules,skills}`                     | Symlinks → `.agents/{rules,skills}`                                  |
| `.grok/agents`                             | Symlink → `.claude/agents`                                           |
| `.opencode/skills`                         | Symlink → `.agents/skills`                                           |
| `.pi/`                                     | Settings only                                                        |

Edit shared content under `.agents/` or `AGENTS.md` only. Adapters are symlinks, `@` imports, or `{file:}` refs — never duplicated bodies.
