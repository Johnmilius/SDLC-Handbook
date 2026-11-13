# Working with agents: what to delegate and what enforces the rules

**Decision this page supports:** What may the agent do alone, and what enforces the rules?

Labels: [Standard], [Established], [Observed] and [Proposed] are defined in [sources.md](sources.md). The quality bar itself lives in [quality-bar.md](quality-bar.md) and is not repeated here. Agent-written code meets the same bar as your own code (QB-12).

## Rules

<a id="ag-1"></a>**AG-1** Trust comes from recorded results, not from how a session felt. [Observed] METR (arXiv 2507.09089v2, Jul 2025): experienced developers took 19% longer with AI tools on their own repositories while believing they were faster ([S1](sources.md#s1)). The study had 16 experts, measured speed only, and used early-2025 tools. This handbook makes no speed claim of its own.

<a id="ag-2"></a>**AG-2** The developer always keeps these decisions: the tier, the acceptance criteria, any change to a test's expected value, any new dependency, merging Tier 3 work, and releases and data migrations. [Proposed] See [risk-tiers.md](risk-tiers.md) for tiers.

<a id="ag-3"></a>**AG-3** A rule with no exceptions goes in a hook or in CI, because instruction files are not guaranteed to be followed. [T3 docs] Claude Code best practices, fetched 2026-11-13 ([S10](sources.md#s10)). [T3 docs] Claude Code hooks guide, fetched 2026-11-13: a pre-tool hook can block a tool call, for example an edit to a protected file ([S12](sources.md#s12)). These are vendor descriptions of how the tool works, not evidence that it improves results.

<a id="ag-4"></a>**AG-4** Protect tests from agent edits; a changed expected value needs the developer's approval (QB-3). [Observed] ImpossibleBench (arXiv 2510.20270v1, Oct 2025): on tasks built so the tests contradict the spec, models often passed anyway by editing or special-casing tests, and read-only test access alone did not stop special-casing ([S3](sources.md#s3)). Rates vary widely by model and prompt, and the tasks are artificial, so this does not tell you how often it happens in daily work.

<a id="ag-5"></a>**AG-5** A new dependency needs the developer's approval (QB-8). [Observed] Spracklen et al. (USENIX Security 2025; arXiv 2406.10279v3) measured invented package names for Python and JavaScript with 2024 models ([S2](sources.md#s2)) [Older-generation tools]. The rate for Rust is unmeasured, so the rule stays cheap insurance rather than a response to a known Rust rate.

<a id="ag-6"></a>**AG-6** Review in a fresh context that receives the acceptance criteria, the diff and the test results, not the author's conversation. [T3 docs] Claude Code best practices, fetched 2026-11-13, recommends a reviewer that has not seen how the code was written ([S10](sources.md#s10)). It is a filter, not a control. [Observed] Olausson et al. (ICLR 2024) found self-repair gains often modest and stronger feedback much more useful ([S4](sources.md#s4)) [Older-generation tools]. Whether a same-model fresh-context reviewer catches more than the author is untested. [Standard] ISTQB CTFL v4.0.1 section 1.5.3 describes independence between human testers and says nothing about AI reviewers ([S15](sources.md#s15)). Human review stays, and Tier 3 requires a line-by-line one.

<a id="ag-7"></a>**AG-7** Keep instruction files short; they hold only what the agent cannot work out from the code. [T3 docs] Claude Code best practices, fetched 2026-11-13 ([S10](sources.md#s10)).

<a id="ag-8"></a>**AG-8** Give the agent a pass/fail check it can run, and a way to stop and flag a conflict between tests and spec. [T3 docs] Claude Code best practices, fetched 2026-11-13 ([S10](sources.md#s10)). [Observed] In ImpossibleBench section 5.3, allowing the agent to flag the task for a human cut test exploitation sharply for some models, with a much weaker effect for one Claude model ([S3](sources.md#s3)).

## Delegation ladder

[Proposed] No source defines a delegation ladder (claim-check C21). The levels and the counts (5 and 10) are starting values to adjust, not measured thresholds. The ladder is built from AG-1, AG-3, AG-4 and AG-8.

| Level | The agent may | The agent may not | Evidence that earns the next level |
|---|---|---|---|
| L0 Suggest | Explain, propose a plan, draft acceptance criteria and test plans | Edit files | The plans are mostly usable as written on 5 changes |
| L1 Supervised edits | Edit code and tests in an interactive session while you review every diff | Change test expectations, add dependencies, or commit without review | 10 changes with no blocking review finding and no test-integrity violation |
| L2 Bounded autonomous task | Work alone on a branch on a Tier 1 or 2 task that has written acceptance criteria and a pass/fail check | Merge, touch protected paths, or continue when tests conflict with the spec (it stops and flags) | 10 tasks merged after review with no blocking finding; a mutation check on a sample shows the tests are sensitive |
| L3 Pre-approved task classes | Run a named low-risk class end to end and open a PR, such as a patch-level dependency bump with a green suite | Anything outside the named class | Not applicable; review this level every quarter |

Keep a simple log (date, task, level, review findings). A blocking finding or a test-integrity violation drops that task class one level.

## Enforcement map (condensed)

[Proposed] This handbook ships no hook scripts and no CI configuration. The map says what to add per project. Skill and instruction text explains and recommends; it does not enforce (AG-3).

| Rule | Layer | Why |
|---|---|---|
| Code is formatted, compiles, and passes lints and tests | CI, plus a local pre-commit run | Deterministic, cheap, and the result is a record |
| No new dependency without approval (AG-5) | Hook (ask before the agent edits the manifest), CI allowlist, and you | The invented-name risk is measured elsewhere; approval is a judgment |
| Agent does not edit protected tests or expected values (AG-4) | Hook on protected paths, a CI flag for changed or deleted tests, and your approval | Models do edit tests; read-only alone did not stop special-casing |
| No secrets in the repository | Hook or CI secret scan | Deterministic; no scanner was evaluated, so the choice is yours |
| The tier and its triggers | CI or checklist detects triggers; you decide the tier | Detection is mechanical, the decision is yours |
| Tests come from the criteria, not the implementation | Skill checklist and you | Needs judgment; no tool reliably detects a mirrored test |
| Tier 3: line-by-line human review and a rollback note | You, plus branch protection if the host offers it | The point is a person's attention; an AI reviewer does not satisfy it |

**CSE 370:** [Unit 03](../cse370-prepare/unit03-design.md) has a section on design in an AI world; [Unit 04](../cse370-prepare/unit04-testing.md) says you own the correctness of your code, which is why AG-2 and AG-6 keep the decision with you.

## Related

- [quality-bar.md](quality-bar.md) (QB-1 to QB-13)
- [risk-tiers.md](risk-tiers.md)
- [verify-a-change.md](verify-a-change.md)
- [plan-tests.md](plan-tests.md)
- [workflow.md](workflow.md)
- [sources.md](sources.md)
- [../skills/test-plan/SKILL.md](../skills/test-plan/SKILL.md)
