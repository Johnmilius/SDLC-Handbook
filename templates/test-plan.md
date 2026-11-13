# Template: test plan

Use at stage 4 of the [workflow](../handbook/workflow.md#wf-4), before the code changes. This is the output format of the [`test-plan` skill](../skills/test-plan/SKILL.md). Rules: [plan-tests](../handbook/plan-tests.md). Size it to the tier ([TP-10](../handbook/plan-tests.md#tp-10)).

```markdown
## Test plan: <one-line change>
Tier proposed: <1|2|3>. Triggers: <list or "none">.  (You decide.)
Acceptance criteria: <numbered list, or "drafted, please confirm">

### Findings
1. Finding: <what could break or is missing>
   Evidence: <file:line, test name, or spec sentence. "Not read" if not read>
   Proposed check: <what to assert> (<unit | integration | end-to-end | real environment>) [exists: <test> | to write]
   Why it matters for this change: <one sentence>
   Handbook: <rule ID>

### Order of work
1. <failing reproduction or characterization tests first>

### Not verified by this plan
- <what no check here covers, and what needs a real environment>

### Questions for you (blocking)
1. <or "none">
```

## Filled-in example (synthetic, Tier 1)

```markdown
## Test plan: fix page_count returning one page too few
Tier proposed: 1. Triggers: none.  (You decide.)
Acceptance criteria: AC-1 Given 11 items and a page size of 10, page_count returns 2.

### Findings
1. Finding: no test covers a total that is not a multiple of the page size.
   Evidence: tests/paging.rs has cases for 10/10 and 20/10 only; src/paging.rs:4 uses integer division.
   Proposed check: page_count(11, 10) == 2 (unit) [to write]
   Why it matters for this change: it is the reported bug, and nothing fails today.
   Handbook: TP-4
2. Finding: the neighbors of the boundary are untested.
   Evidence: same test file.
   Proposed check: (0, 10) == 0, (10, 10) == 1, (1, 10) == 1 (unit) [to write]
   Why it matters for this change: the fix must not change exact multiples or zero.
   Handbook: TP-3

### Order of work
1. Write finding 1's test and watch it fail.
2. Fix the function. 3. Add finding 2's tests.

### Not verified by this plan
- page_size == 0 divides by zero today. It is outside this fix.

### Questions for you (blocking)
1. Should page_size == 0 be handled in this change, or tracked separately?
```
