# Risk tiers: how much process a change needs

**Decision this page supports:** How much process does this change need?

**CSE 370:** [Unit 04](../cse370-prepare/unit04-testing.md) supports "testing is about managing risk" and "how much is enough depends on the cost of failure". [Unit 07](../cse370-prepare/unit07-sdlc-revisited.md) supports "a methodology never lets you skip a phase".

## Rules

<a id="rt-1"></a>**RT-1** Depth scales with risk (likelihood times impact), not with the size of the diff. [Standard] ISTQB CTFL v4.0.1 (2024) §5.2.3 ([S15](sources.md#s15)). [Standard] NIST SP 800-218 v1.1 (2022) §2 says to apply practices by risk, not as a fixed checklist ([S13](sources.md#s13)). [Established] CSE 370 [Unit 04](../cse370-prepare/unit04-testing.md) warns against calling a part "non-critical" to excuse skipping tests.

<a id="rt-2"></a>**RT-2** No tier skips a stage. A tier changes how deep a stage goes, never whether it happens. [Established] CSE 370 [Unit 07](../cse370-prepare/unit07-sdlc-revisited.md) (a methodology may change the timing or style of a phase, never whether it happens).

<a id="rt-3"></a>**RT-3** An escalation trigger raises the tier. Only the developer lowers one, and writes down why. [Proposed]

<a id="rt-4"></a>**RT-4** The agent may propose a tier. The developer decides. [Proposed] See [working-with-agents.md](working-with-agents.md).

## The tier table

The three tier names and the escalation triggers below are [Proposed]. No source read for this handbook lists them (claim-check C16). Only the idea that depth follows risk (RT-1) and that no stage is skipped (RT-2) is sourced.

| Stage | Tier 1: low risk (typical small bug fix) | Tier 2: normal (typical feature) | Tier 3: high risk (hard to reverse, or a trigger fired) |
|---|---|---|---|
| Problem | One line | One line plus "not included" | Same, plus who is affected if it goes wrong |
| Requirements | One failing example of the bug | Given/When/Then criteria with negative cases | Same, plus explicit non-functional criteria (data integrity, security) |
| Design | In your head | Short plan: steps, each with "done when" | ADR per significant decision; steps with done-when, dependencies, main risk |
| Test plan | A regression test that fails before the fix, plus nearby boundary cases | Checks per criterion at the right layer; integration check named | Same, plus a rehearsal on production-like data and a "cannot be proven in CI" list |
| Implementation | One small change | Small changes; refactors separate | Small changes behind a reversible step where possible |
| Verification | Deterministic checks; developer reads the diff | Adds independent AI review; mutation check on changed code optional | Adds mutation check on changed code and a line-by-line human review; agent review is extra, never a substitute |
| Release | Normal release | Normal release, note what changed | Rollback note, a named point of no return, a watch list |
| Maintenance | Nothing extra | Check the change log entry | Check the watch list after release; write up what happened |

The tier table is a [Proposed] synthesis. Its stage names follow [workflow.md](workflow.md). For what each verification step can and cannot show, see [verify-a-change.md](verify-a-change.md). For test plan content by tier, see [plan-tests.md](plan-tests.md). The shared quality bar applies at every tier: [quality-bar.md](quality-bar.md).

## Escalation triggers [Proposed]

A trigger can raise a tier. Nothing lowers one except the developer writing down why (RT-3).

**Raise to Tier 3 if the change:**
- touches authentication, authorization, sessions, secrets or cryptography
- migrates or deletes stored data, or changes a schema
- touches money (billing, payments, balances)
- cannot be undone by redeploying the last-good version
- adds or changes `unsafe` code or FFI

**Raise to at least Tier 2 if the change:**
- changes a public API, a file format or a wire contract
- adds or upgrades a dependency
- touches concurrency (locks, channels, shared state)
- changes code that has no tests (write characterization tests first)
- is larger than about 400 changed lines or 10 files (split it, or treat it as Tier 2 or higher)

Most of these triggers can be detected from the paths and contents of the diff, such as a schema file, a lockfile or an `unsafe` block. That lets a script or checklist flag them, so a tier cannot be argued down in conversation. [Proposed] The line and file counts are starting values, not measured thresholds. [Proposed]

## Related

- [workflow.md](workflow.md)
- [plan-tests.md](plan-tests.md)
- [verify-a-change.md](verify-a-change.md)
- [working-with-agents.md](working-with-agents.md)
- [quality-bar.md](quality-bar.md)
- [sources.md](sources.md)
- [../skills/test-plan/SKILL.md](../skills/test-plan/SKILL.md)
