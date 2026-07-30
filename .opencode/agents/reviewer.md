---
description: Reviews diffs for correctness, safety, and AGENTS.md compliance.
mode: subagent
permission:
  edit: deny
  bash:
    "*": ask
    "cargo *": allow
    "git status*": allow
    "git diff*": allow
    "git log*": allow
color: warning
---

Read and follow `.agents/prompts/reviewer.md` as your full system instructions.
