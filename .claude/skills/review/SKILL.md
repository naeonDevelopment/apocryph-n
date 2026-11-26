---
name: review
description: Independent review of a diff in a fresh context. Use before opening or merging a pull request, for any change that touches a protected path, a policy or an evaluator, and whenever the author is an agent.
---

# review — a second pair of eyes that did not write the code

1. **Fresh context.** The reviewer is not the session that wrote the change. Give it only: the diff
   (`git diff origin/<default>...HEAD`), the brief, the acceptance checks and AGENTS.md. Never the author's
   reasoning. Self-review is not review.
2. **Read what the diff touches**, not just the diff: callers, tests, and the policy in `.agents/cell/cell.json`.
3. **Look for, in this order:**
   - a check that would still pass if the change did nothing (tests that assert nothing, mocks of the thing under test);
   - edits to evaluators, tests or expectations made to turn something green;
   - absent values coerced to zero or empty; errors swallowed;
   - effects outside the brief's write scope; protected or generated paths changed by hand;
   - secrets, customer data, other companies' material, AI attribution.
4. **Each finding** names the path and line, the observable failure, the evidence, and the check that would
   catch it. Opinions about style are not findings.
5. **Risky changes get a second reviewer from another model family** (policy, authorization, money,
   migrations, anything irreversible): reviewers from different families find different defects.
6. **Verdict:** `NO ISSUES at <full head sha>` or the list of blocking findings. A verdict is bound to the
   exact head it read; a new push needs a new verdict.

The reviewer is read-only. It does not push fixes; it reports them.
