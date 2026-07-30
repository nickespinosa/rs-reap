@AGENTS.md

## Claude Code

- Path rules: `.claude/rules/` (symlink → `.agents/rules/`).
- Skills: `.claude/skills/` (symlink → `.agents/skills/`).
- Subagents: `.claude/agents/` (thin frontmatter + shared prompts).
- Settings: `.claude/settings.json` (permissions/hooks only — not project facts).
- Prefer plan mode for non-trivial API or `unsafe` changes.
- After edits: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`.
