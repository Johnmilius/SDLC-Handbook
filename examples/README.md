# Worked examples

Two small synthetic cases the [`test-plan` skill](../skills/test-plan/SKILL.md) can be tried on. The code is in one dependency-free crate, [handbook-examples/](handbook-examples/).

| Case | Code | Walkthrough | Kind |
|---|---|---|---|
| 1. A bounded config value over an HTTP-style API | [src/config.rs](handbook-examples/src/config.rs) | [case-1-max-retries.md](case-1-max-retries.md) | New feature, Tier 2 |
| 2. Title truncation that crashes on emoji | [src/title.rs](handbook-examples/src/title.rs) | [case-2-truncate-title.md](case-2-truncate-title.md) | Bug fix in existing code, Tier 1 |

## The code has defects on purpose

Each module contains **planted defects**. The crate compiles, it is lint-clean, and its own tests pass. From `examples/handbook-examples`:

```bash
cargo test
```

That is the lesson of both cases. Green tests show that the tests pass. They do not show that the code meets its spec ([QB-2](../handbook/quality-bar.md#qb-2), [VC-3](../handbook/verify-a-change.md#vc-3)).

The walkthroughs list the planted defects and show the plan the skill is expected to produce. Those expected outputs are **[Illustrative]**: they were written by hand from the answer keys, not produced by a run. Results from real runs are in [eval/results/](../eval/results/).

To try a case yourself first, read only the request at the top of the walkthrough, run the skill on the source file, and then compare.
