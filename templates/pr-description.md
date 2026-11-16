# Template: PR description

Use at stage 6 of the [workflow](../handbook/workflow.md#wf-6), for every tier. It is the record of how the change was verified ([verify a change](../handbook/verify-a-change.md)) and of what was not ([QB-13](../handbook/quality-bar.md#qb-13)).

Open with the TL;DR for people ([RV-8](../handbook/review-a-pr.md#rv-8)). Everything after the line is the record for agents and handoffs.

```markdown
## TL;DR
- **What and why:** <one line>
- **Tier:** <1|2|3>
- **Risks:** <what could go wrong for users, and how likely>
- **Debt:** <shortcuts, missing tests, manual steps, things to revisit>
- **Pros / cons:** <what we gain, and what it costs>
- **Needs a person:** <approve, test, set something, decide>
- **Not checked:** <the most important gaps>

---

## What changed and why
<two or three sentences. Link the acceptance criteria>

Tier: <1|2|3>  Triggers: <list or "none">

## How it was verified
- [ ] Format, build, lints, tests pass (VC-1 to VC-3): <link to CI run>
- [ ] Test integrity (VC-4): <"no tests deleted, ignored or re-expected", or list each one and why>
- [ ] Dependencies (VC-5): <"none added", or name, version, why, and who approved>
- [ ] Mutation check on changed code (VC-6, Tier 3): <result, or "not required">
- [ ] Independent AI review (VC-7, Tier 2 and 3): <findings and what was done with each>
- [ ] Human review (VC-8): <who, and what they looked at hardest>
- [ ] Real-environment checks (VC-9): <which, or "none apply">

## Not verified
- <what no check covered>

## Quality-bar exceptions
- <rule ID and reason, or "none">
```

## Filled-in example (synthetic, Tier 2)

```markdown
## TL;DR
- **What and why:** lets an admin set the retry count (0 to 10, default 3), and the retry worker now uses it.
- **Tier:** 2 (it adds to the public API).
- **Risks:** jobs already running when the value changes keep the old count. Low impact.
- **Debt:** no test for a write during a read under load.
- **Pros / cons:** no redeploy needed to tune retries. One more setting to document.
- **Needs a person:** approve and merge.
- **Not checked:** jobs already running when the value changes; a write during a read under load.

---

## What changed and why
Adds PUT /config/max_retries (0 to 10, default 3) and makes the retry worker read the saved value.
Criteria AC-1 to AC-4 in docs/changes/max-retries.md.

Tier: 2  Triggers: public API contract

## How it was verified
- [x] Format, build, lints, tests pass: CI run 412
- [x] Test integrity: no tests deleted, ignored or re-expected
- [x] Dependencies: none added
- [ ] Mutation check: not required at Tier 2
- [x] Independent AI review: 2 findings. One fixed (missing-field case). One declined (naming), reason in thread
- [x] Human review: me. Read every line; spent longest on the worker reading the saved value
- [x] Real-environment checks: none apply

## Not verified
- Behavior for jobs already running when the value changes (out of scope, AC "Not included")
- A write during a read under load

## Quality-bar exceptions
- none
```
