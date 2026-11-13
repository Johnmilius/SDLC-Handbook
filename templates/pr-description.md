# Template: PR description

Use at stage 6 of the [workflow](../handbook/workflow.md#wf-6), for every tier. It is the record of how the change was verified ([verify a change](../handbook/verify-a-change.md)) and of what was not ([QB-13](../handbook/quality-bar.md#qb-13)).

```markdown
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
