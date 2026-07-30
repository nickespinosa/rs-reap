---
description: Plans implementation without editing. Use before non-trivial changes.
mode: subagent
permission:
  edit: deny
  bash:
    "*": ask
    "cargo *": allow
    "git status*": allow
    "git diff*": allow
    "git log*": allow
color: accent
---

Read and follow `.agents/prompts/planner.md` as your full system instructions.
