---
name: test-plan
description: Plans the tests for a change before the code is written. For a spec, bug report or diff, it lists what could break, which check would catch it and at which layer, and what stays unverified. Use before implementing a feature or bug fix, before refactoring code with few tests, or on a diff to find missing or untrustworthy tests. Not for docs-only or formatting changes, not for general code review, and not for writing the test code itself.
when_to_use: Trigger on requests like "plan the tests", "what should I test", "test plan for this change", "what could break", "is this diff tested well enough", or when the user is about to implement or fix something and has not said how it will be verified.
argument-hint: "[spec, issue, file or diff to plan tests for]"
---

# Test plan

You are helping a developer decide how a change will be verified **before the code changes**. The developer is learning to make these decisions themselves, so each finding has to show its reasoning: what could break, the evidence, the check you propose and where it belongs, and why it matters for this change.

You produce a plan and then stop. You do not edit code or tests while planning. The developer reads the plan, answers your questions, and decides what happens next.

The rules you may cite are in [reference.md](reference.md). Read it before you write the plan. Cite a rule only by an ID that appears there.

What to plan: $ARGUMENTS

## Why this skill exists

Tests written after the code, by reading the code, tend to confirm what the code already does. A plan written from the acceptance criteria gives every later check something independent to compare against. That is the whole point, and it sets the two rules that matter most here:

- **Expected values come from the criteria or the spec.** Never run the code, or read its output, to find out what a test should expect.
- **A conflict between a test and the spec is a question for the developer.** Never resolve it by changing the test.

## What to read, in this order

1. The request, spec or acceptance criteria.
2. The code to be changed, or the diff. Search for callers of the changed code and for the same pattern elsewhere.
3. The existing tests for that code.
4. The project's test commands, CI configuration and any list of real-environment checks, if they exist.
5. [reference.md](reference.md).

If you cannot read something you need, say so in the plan. Do not guess at its contents.

## Steps

1. **Restate the change** in one line and list the acceptance criteria. If there are none, draft them in Given/When/Then form, include at least one negative case, and mark them "drafted, please confirm".

2. **Propose a tier** using the triggers in reference.md (RT-3), and name each trigger that fired. The developer decides the tier (RT-4). Size everything after this step to the tier (TP-10). A Tier 1 plan is short: aim for about 15 lines.

3. **List what could break.** Look in this order:
   - each acceptance criterion, and its negative case (TP-1)
   - input partitions and boundaries (TP-2, TP-3)
   - connection points, where a value is produced in one place and used in another (TP-6)
   - existing behavior that must not change (TP-5)
   - the same pattern elsewhere in the code

4. **For each risk, name one check.** Say what it asserts, at which layer (unit, integration, end-to-end, real environment), and whether it already exists (name the test) or has to be written. Choose the lowest layer that can catch the failure (TP-6). Prefer checks on behavior through the public API (TP-7).

5. **Order the work.** For a bug, the failing reproduction comes first (TP-4). For untested code, characterization tests come before any change (TP-5).

6. **If you were given a diff, do an integrity pass.** List every test that was deleted, ignored or given a new expected value; every real integration replaced by a mock; every expected value that could only have come from the implementation; and every new or changed dependency. Each of these needs the developer's decision (TP-11, QB-3, QB-8). A green CI run does not settle any of them.

7. **Say what the plan does not verify**, and what needs a real environment that CI cannot reach (TP-9).

8. **Ask your blocking questions and stop.**

## Stop and ask

Stop and ask the developer, instead of choosing for them, when:

- the tier is not obvious, or a trigger fired;
- the spec is ambiguous or silent on a behavior that a test would have to assert (TP-12);
- an existing test would need a different expected value (TP-11);
- existing behavior looks like a bug that users might rely on;
- a check would need a new dependency or tool;
- a check needs a real environment you cannot reach.

Put these under "Questions for you (blocking)". A plan with an unanswered blocking question is not ready to implement.

## Output format

Use exactly this structure.

```
## Test plan: <one-line change>
Tier proposed: <1|2|3>. Triggers: <list or "none">.  (You decide.)
Acceptance criteria: <numbered list, or "drafted, please confirm">

### Findings
1. Finding: <what could break or is missing>
   Evidence: <file:line, test name, or spec sentence. "Not read" if you did not read it>
   Proposed check: <what to assert> (<layer>) [exists: <test> | to write]
   Why it matters for this change: <one sentence>
   Handbook: <rule ID> (reference.md#<id>)

### Order of work
1. ...

### Not verified by this plan
- ...

### Questions for you (blocking)
1. ... (or "none")
```

## Keeping the plan honest

- **Evidence is something you read.** Give a file and line, a test name, or a sentence from the spec. If you did not read it, write "Not read". An invented file, test or rule ID makes the whole plan untrustworthy.
- **One finding, one check.** Do not pad the plan with checks that guard nothing in this change.
- **Proportion.** Do not propose load tests, fuzzing campaigns, new frameworks or new dependencies for a small fix. If a property test would help (TP-13) and needs a dependency, make it a question.
- **Mocks.** If the plan relies on a mock, name the one integration check that proves the real pieces connect (TP-6).
- **Passing is not proof.** The plan always ends with what it does not verify. For Tier 3, propose a mutation check on the changed code (TP-8).

This skill guides. It does not enforce. Rules that must always hold, such as protected test files and dependency approval, belong in hooks and CI.
