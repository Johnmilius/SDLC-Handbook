# Review a PR: how deep, and is it ready to merge?

**Decision this page supports:** how deep should this review go, and is the PR ready to merge?

This page puts the [risk tiers](risk-tiers.md), the [quality bar](quality-bar.md) and the [verification checks](verify-a-change.md) in the order a reviewer uses them. It adds no new bar: the quality bar is the bar. What changes by tier is how deep the review goes, never whether there is one ([RT-1](risk-tiers.md#rt-1), [RT-2](risk-tiers.md#rt-2)).

Every rule on this page is [Proposed]. It is this handbook's synthesis of the pages it links, with no source of its own. Review date: 2027-02-13, at the first quarterly review.

## Rules

<a id="rv-1"></a>**RV-1** Triage before hunting for bugs. Propose a tier from the [escalation triggers](risk-tiers.md#escalation-triggers-proposed) and the project's path map, and put it in the first line of the review. The developer decides ([RT-4](risk-tiers.md#rt-4), [AG-2](working-with-agents.md#ag-2)). [Proposed]

<a id="rv-2"></a>**RV-2** Review to the depth the tier asks for, and no further. Walking every quality-bar rule on a Tier 1 change spends attention that Tier 3 changes need. [Proposed], from [RT-1](risk-tiers.md#rt-1).

<a id="rv-3"></a>**RV-3** Judge the evidence, not only the code. For each check the PR cites, ask whether it actually runs the changed code. A passing test that never executes the change is not evidence for it. [Proposed], from [VC-3](verify-a-change.md#vc-3) and [QB-2](quality-bar.md#qb-2).

<a id="rv-4"></a>**RV-4** Cite a rule ID when it explains a finding. Do not produce a rule-by-rule table below Tier 3. The IDs are there to make a finding precise, not to fill a form. [Proposed]

<a id="rv-5"></a>**RV-5** Every review ends with a verdict (merge, merge after named fixes, or not yet) and a "Not verified" list ([QB-13](quality-bar.md#qb-13)). [Proposed]

<a id="rv-6"></a>**RV-6** A tier is lowered only in writing ([RT-3](risk-tiers.md#rt-3)). A common case: a visual-only change that redeploying undoes, which the developer has seen working in the real environment ([VC-9](verify-a-change.md#vc-9)), but which trips "changes code that has no tests". [Proposed]

<a id="rv-7"></a>**RV-7** Keep the path-to-tier map and the project's own versions of the checks in the project's repository, as a project profile. This handbook holds only the method. [Proposed], following the README's rule that project decisions live with the project.

## Step 1: Triage (a few minutes)

1. Read the PR description and the list of changed files, not the diff yet.
2. Match the files and the description against the escalation triggers and the project profile's path map. The highest match sets the tier.
3. Write the first line of the review: `Tier: <n> (proposed). Triggers: <list or "none">.`
4. If the PR already states a tier, check it against the triggers. A trigger the PR missed is a finding.

## Step 2: Review to the tier

Each tier includes everything in the tiers above it in this table.

| Tier | Check | Rule |
|---|---|---|
| **Every tier** | The deterministic checks pass, with a recorded result. If only the author ran them, say so | [VC-1](verify-a-change.md#vc-1) to [VC-3](verify-a-change.md#vc-3), [AG-1](working-with-agents.md#ag-1) |
| | No test was deleted, skipped or given a new expected value without the developer's approval | [VC-4](verify-a-change.md#vc-4), [QB-3](quality-bar.md#qb-3) |
| | No new dependency without approval | [VC-5](verify-a-change.md#vc-5), [QB-8](quality-bar.md#qb-8) |
| | A person reads the diff | [VC-8](verify-a-change.md#vc-8) |
| | The PR has a "Not verified" list | [QB-13](quality-bar.md#qb-13) |
| | The project's own PR rules hold (from its instruction file) | Project profile |
| **Tier 1 adds** | A regression check that fails without the change, or the missing check written as an exception | [TP-4](plan-tests.md#tp-4), [quality bar](quality-bar.md) |
| | The checks the PR cites actually run the changed code | RV-3 |
| **Tier 2 adds** | Acceptance criteria, written or rebuilt from the PR, each with a result, including the negative cases | [QB-1](quality-bar.md#qb-1), [QB-4](quality-bar.md#qb-4) |
| | Each test would fail if the code were broken | [QB-2](quality-bar.md#qb-2), [TP-8](plan-tests.md#tp-8) |
| | An independent AI review in a fresh context, with each finding fixed or declined with a reason | [VC-7](verify-a-change.md#vc-7), [AG-6](working-with-agents.md#ag-6) |
| | One concern per change, and the simplest design that works | [QB-7](quality-bar.md#qb-7), [QB-6](quality-bar.md#qb-6) |
| **Tier 3 adds** | An ADR for each decision that is hard to reverse. A missing ADR is a finding on its own | [WF-3](workflow.md#wf-3) |
| | A test plan with a "cannot be proven in CI" list | [Risk tiers](risk-tiers.md), [plan-tests.md](plan-tests.md) |
| | A mutation check on the changed code, or a note that the project has no tool for it | [VC-6](verify-a-change.md#vc-6) |
| | A line-by-line human review. An AI review does not satisfy it | [VC-8](verify-a-change.md#vc-8), [AG-6](working-with-agents.md#ag-6) |
| | A rollback note, a named point of no return and a watch list | [WF-7](workflow.md#wf-7) |
| | Who is affected if it goes wrong, and how they would find out | [Risk tiers](risk-tiers.md), Problem row |

## Step 3: Write the review

**Tier 1**, about five lines:

```markdown
Tier 1 (proposed). Triggers: none.
Checks: <passed, CI run or "author-run">. Tests touched: none. Dependencies: none.
Read the diff: <one line, or "nothing to change">.
Not verified: <list>.
Verdict: <merge | merge after X | not yet>.
```

**Tier 2 and 3:**

```markdown
Tier <2|3> (proposed). Triggers: <list>.

## Verdict
<merge | merge after the items marked "before merge" | not yet>, in one or two sentences.

## Evidence
- Checks: <recorded result, or author-run> (VC-1 to VC-3)
- Tests touched: <none, or each one and whether it was approved> (VC-4)
- Dependencies: <none, or name and approval> (VC-5)
- Does the cited evidence run the changed code? <yes, or what it misses> (RV-3)
- Real environment: <what was tried, by whom> (VC-9)

## Acceptance criteria
| # | Criterion | Result |

## Findings
1. **<Before merge | Should fix | Later>** <claim>. <rule ID>. <how it breaks>. <suggested fix>.

## Not verified
- <list>
```

For Tier 3, add a line for each Tier 3 artifact (ADR, test plan, rollback note, line-by-line review) saying whether it exists.

## Signs the review is the wrong size

| Too heavy (mostly at Tier 1) | Too light (mostly at Tier 3) |
|---|---|
| The review is longer than the diff | No question about an ADR or a rollback note |
| A table of every quality-bar rule | "Tests pass" accepted without checking that they run the change |
| A fix plan for each cosmetic nit | No "Not verified" list |
| An AI review run by default (VC-7 is Tier 2 and 3) | An AI review accepted in place of a person's line-by-line read |

## Project profile

A project profile is a short file in the project's own repository (RV-7). It holds:

1. **Who decides:** who sets the tier and who merges.
2. **Path map:** which paths or kinds of change raise the tier, beyond the generic triggers.
3. **The project's checks:** the command or place for each of VC-1 to VC-9, and which ones CI records.
4. **The project's own PR rules,** from its instruction file.
5. **Standing tier exceptions** the developer has approved, written once instead of in every PR (RV-6).

Synthetic example, for the crate in [examples/handbook-examples](../examples/handbook-examples/):

| Paths or change | Tier |
|---|---|
| `src/auth/**`, `migrations/**`, anything that reads a secret | 3 |
| `Cargo.toml`, `Cargo.lock`, a public function's signature in `src/lib.rs` | 2 |
| `src/ui/**`, docs, comments | 1 |

## Worked examples

- [Case 1](../examples/case-1-max-retries.md) is Tier 2 (a public API contract). Its review needs acceptance criteria, an AI review and the QB-2 question.
- [Case 2](../examples/case-2-truncate-title.md) is Tier 1. Its review is the five-line form: the regression test, the diff read, and the "Not verified" list.

## Related

[Risk tiers](risk-tiers.md) · [Quality bar](quality-bar.md) · [Verify a change](verify-a-change.md) · [Workflow](workflow.md) · [Working with agents](working-with-agents.md) · [Plan tests](plan-tests.md) · [PR description template](../templates/pr-description.md)
