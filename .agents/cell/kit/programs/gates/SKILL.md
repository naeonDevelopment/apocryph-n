---
name: "gates"
description: "Run this repository's release checks exactly as CI does and report each result; only exit 0 is a pass."
---

# gates — the checks CI runs, run the same way

1. Read `.agents/cell/cell.json` → `gates`. Those commands are the repository's gates; run each from the
   Git root, in order, and keep going after a failure so the report shows every result.
2. Always also run `python3 .agents/cell/kit/cell.py conform`: the cell's own conformance (contract,
   programs, policy canaries, sandbox settings, kit provenance, pulse).
3. Before committing, run `python3 .agents/cell/kit/cell.py attribution --message <file>` on the commit
   message, or `--range origin/<default>..HEAD` on the branch. No AI attribution of any kind.

Exit codes: 0 clean, 1 findings, 2 could not reach a verdict, 3 an external service failed. **Only 0 is a
pass.** A gate you did not run is "not run", never "passed". A missing tool is exit 2: name it.

Report:

| Gate | Result | Evidence |
|---|---|---|
| each gate | pass / FAIL / not run (why) | the command and the last lines of its output |

For each failure give the file and line and the smallest fix. Never edit a test's expectation to turn a red
gate green; decide first whether the test, the harness or the product is wrong.
