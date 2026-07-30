---
name: code-quality
description: Review and enforce this repository's documented code-quality standards, including naming, documentation, layout, implementation patterns, tests, formatting, linting, and CI gates. Use when reviewing a repository, preparing a change for merge, or asked to make a project meet its standards.
disable-model-invocation: false
---

# Code quality

Review the repository as an integrated project. Follow its `AGENTS.md`, scoped
rules, contribution guide, and existing automation before applying changes.
Treat those files as authoritative; use neighboring or upstream repositories as
references only when the project explicitly points to them.

## Workflow

1. Establish scope with `git status --short --branch`, the current diff, and
   the repository's instruction files. Preserve unrelated user changes.
2. Identify the project language, supported platforms, public API, directory
   layout, naming patterns, documentation expectations, and canonical commands.
   Prefer existing Make targets, scripts, cargo aliases, or CI commands over
   inventing new checks.
   Run `scripts/check-dependencies.sh` from this skill when dependency or
   toolchain readiness is part of the review; it is read-only and reports
   missing local prerequisites.
3. Review implementation and tests for correctness, portability, error
   handling, security, resource cleanup, and consistency with established
   patterns. Check that public API, filenames, modules, and symbols follow the
   repository's naming conventions.
4. Check documentation for accurate setup, usage, API behavior, supported
   platforms, development commands, and license information. Remove stale
   references to deleted tools or directories.
5. Run the strongest available quality gate. For `rs-reap`, use:

   ```bash
   make verify
   make ci
   ```

   `make verify` covers formatting, Clippy, and tests; `make ci` also builds
   documentation. If the toolchain or dependencies are unavailable, report
   that explicitly instead of treating an unrun check as passing.
6. Apply only focused fixes required by the request and add or update tests and
   docs for behavior changes. Do not broaden the public API or perform
   drive-by refactors.
7. Re-run relevant checks, inspect `git diff --check`, and finish with a clean
   or clearly documented worktree state.

## Review checklist

- Naming, module boundaries, directory structure, and file placement match
  the repository's established patterns.
- Public APIs and documented behavior are intentional and consistent.
- Tests cover changed behavior, edge cases, platform branches, and cleanup;
  tests do not leak processes, files, threads, or credentials.
- Formatting and lint configuration are used rather than bypassed.
- Unsafe code, shell commands, dependency changes, and platform-specific code
  have a concrete safety rationale.
- CI runs the same meaningful checks developers are instructed to run.
- README, contribution guidance, API docs, changelog expectations, and license
  metadata are accurate and internally consistent.
- Generated artifacts, local caches, secrets, and editor/tool state are not
  committed.

## Dependency readiness

Run the bundled checker from the repository root:

```bash
.agents/skills/code-quality/scripts/check-dependencies.sh
```

It discovers common manifests (`Cargo.toml`, `go.mod`, `package.json`, and
`pyproject.toml`), verifies the corresponding package-manager commands, and
for this Rust project checks `cargo`, `rustc`, `rustfmt`, `clippy`, and `make`.
It never edits manifests or lockfiles and does not download dependencies.

## Report

Classify the result as `ship`, `ship with nits`, or `block`. List findings by
severity with file paths and concrete fixes. Include each check's pass, fail,
or blocked status and explain environmental blockers separately from code
failures.
