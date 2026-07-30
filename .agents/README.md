# Shared agent configuration

Single source of truth for multi-tool agent setup. Tool directories
(`.claude/`, `.opencode/`, `.grok/`, `.pi/`) are thin adapters that inherit
from here plus root `AGENTS.md`.

```text
.agents/
  rules/          Path-scoped markdown rules
  skills/         Agent Skills packages (SKILL.md)
  prompts/        Shared subagent bodies (no tool frontmatter)
```

## Inheritance

| Tool        | Instructions                   | Rules                      | Skills                         | Agents                                |
| ----------- | ------------------------------ | -------------------------- | ------------------------------ | ------------------------------------- |
| All         | `AGENTS.md`                    | —                          | —                              | —                                     |
| Claude Code | `CLAUDE.md` → `@AGENTS.md`     | `.claude/rules` → `rules/` | `.claude/skills` → `skills/`   | `.claude/agents/*` wraps `prompts/`   |
| OpenCode    | `opencode.json` `instructions` | via glob                   | `.opencode/skills` → `skills/` | `.opencode/agents/*` wraps `prompts/` |
| Grok Build  | discovers `AGENTS.md`          | `.grok/rules` → `rules/`   | `.grok/skills` → `skills/`     | `.grok/agents/*` wraps `prompts/`     |
| Codex       | discovers `AGENTS.md`          | —                          | `.agents/skills`               | —                                     |
| Pi          | discovers `AGENTS.md`          | —                          | —                              | `.pi/` settings only                  |

When adding a rule or skill, put it under `.agents/` only. Adapters pick it up via symlink.
