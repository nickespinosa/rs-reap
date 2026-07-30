# Shared agent configuration

Single source of truth for multi-tool agent setup. Tool directories
(`.claude/`, `.grok/`, `.pi/`) are thin adapters that inherit
from here plus root `AGENTS.md`.

```text
.agents/
  rules/          Path-scoped markdown rules
  skills/         Agent Skills packages (SKILL.md)
  prompts/        Shared subagent bodies (+ name/description frontmatter)
```

## Inheritance

| Tool        | Instructions               | Rules                      | Skills                       | Agents                                      |
| ----------- | -------------------------- | -------------------------- | ---------------------------- | ------------------------------------------- |
| All         | `AGENTS.md`                | —                          | —                            | —                                           |
| Claude Code | `CLAUDE.md` → `@AGENTS.md` | `.claude/rules` → `rules/` | `.claude/skills` → `skills/` | `.claude/agents/*` → `@prompts/*`           |
| OpenCode    | `opencode.json`            | via `instructions` glob    | `.agents/skills`              | `opencode.json` `agent.*.prompt` `{file:}` |
| Grok Build  | discovers `AGENTS.md`      | `.grok/rules` → `rules/`   | `.grok/skills` → `skills/`   | `.grok/agents` → `.claude/agents`           |
| Codex       | discovers `AGENTS.md`      | —                          | `.agents/skills`             | —                                           |
| Pi          | discovers `AGENTS.md`      | —                          | —                            | `.pi/` settings only                        |

### Edit rules

1. **Project facts / commands / architecture** → `AGENTS.md` only.
2. **Path-scoped rules or skills** → `.agents/rules/` or `.agents/skills/` only.
3. **Subagent behavior** → `.agents/prompts/*.md` only.
4. **Tool wiring** (permissions, colors, tool allowlists) → that tool’s adapter
   (`opencode.json`, `.claude/agents/*` frontmatter, `.claude/settings.json`, …).

Never copy prompt bodies or project facts into tool trees.
