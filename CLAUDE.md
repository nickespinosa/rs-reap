@AGENTS.md

## Claude Code

- Rules/skills: `.claude/{rules,skills}/` → `.agents/{rules,skills}/`
- Subagents: `.claude/agents/*` (tool frontmatter + `@.agents/prompts/*`)
- Settings: `.claude/settings.json` (permissions only — not project facts)
- Prefer plan mode for non-trivial API or `unsafe` changes.
