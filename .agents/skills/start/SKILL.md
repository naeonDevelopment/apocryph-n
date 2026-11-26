---
name: start
description: Begin any task in this repository. Measures the tree, reads the cell's pulse and peers, checks the sandbox, and writes the task brief before any edit. Use at session start, after compaction, and before widening scope.
---

# start — measure, then brief

1. **Measure, never remember.** Run `pwd`, `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git rev-parse HEAD`, `git status --short`, and `git rev-list --left-right --count HEAD...origin/<default>`
   (default branch: `.agents/cell/cell.json` → `default_branch`). If HEAD is behind, say so before relying
   on anything in the tree. Never switch a dirty checkout; use a worktree.
2. **Read the cell.** `python3 .agents/cell/kit/cell.py pulse` shows recent changes and register updates.
   If `cell.json` names a `state_check`, run it; a failure means "state unmeasured", not "state fine".
3. **Peers.** If the repository has a coordination board or an issue claim loop (see AGENTS.md), check it
   and claim the paths you will touch before editing.
4. **Sandbox.** In Claude Code, if `/sandbox` reports it off, say so once; the cell's write and read
   denials only hold inside it.
5. **Flow limit.** If `cell.json` → `flow.max_open_agent_prs` is set and that many agent pull requests are
   already open and unreviewed, do not start new implementation: review or finish one instead.
6. **Write the brief** before the first edit, in the issue, the pull request or your first message:

   | Field | Content |
   |---|---|
   | Outcome | what will be true when done |
   | Non-goals | what this task will not touch |
   | Write scope | the paths you will change |
   | Acceptance | the exact checks that must pass (`gates` names them) |
   | Verification | how the behaviour will be shown, not just built |
   | Stop condition | when you stop and report instead of continuing (default: two failed CI rounds, or 15 minutes without progress) |

Reversible work inside the brief proceeds without asking. Irreversible effects (deploy, migration on a live
database, publication, a message to a person, a merge) wait for the human who owns that decision.
