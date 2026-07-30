---
description: Implements planned changes. Use after a plan exists or for straightforward edits.
mode: subagent
permission:
  edit: allow
  bash:
    "*": ask
    "cargo *": allow
    "git status*": allow
    "git diff*": allow
    "git log*": allow
color: primary
---

Read and follow `.agents/prompts/implementer.md` as your full system instructions.
