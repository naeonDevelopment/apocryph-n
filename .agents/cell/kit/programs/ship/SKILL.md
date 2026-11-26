---
name: "ship"
description: "Take finished work to a pull request: gates, attribution check, scoped commit, feature-branch push. Never merges."
---

# ship — from done to a reviewable pull request

1. **Account for every path.** `git status --short` and `git diff --stat`. Stage only paths in the brief's
   write scope; a path you cannot explain is a stray, whatever it contains.
2. **Run `gates`.** Anything other than all-pass stops here, with the report.
3. **Generated files.** If the repository has a compiler (`cell.json` → `compiler.check`), its check must pass;
   regenerate rather than hand-edit.
4. **Commit** on a feature branch, never the default branch: small, imperative, scoped to the task, following
   the repository's message convention. No AI attribution.
5. **Push the feature branch** and open a pull request whose body holds the brief, the gates table, what was
   driven against the real artifact (not only unit-tested), and what remains unmeasured. Reference the issue
   with the repository's convention.
6. **Stop.** Merging needs an independent `review` verdict on the exact head and the owner's merge route
   (`issues.py merge`, a Black Box mandate, or the forge rule). The cell's policy refuses pushes to the
   default branch and force pushes.
