# D5. First skill: design and evaluation

Supports **Decision 3** (what to build and evaluate first). Source numbers are in [D3](D3-source-comparison.md); claim numbers in [04-claim-check.md](../04-claim-check.md); cases in [02-synthetic-cases.md](../02-synthetic-cases.md).

## 0. Which skill first (RQ4)

Scores are 1 to 5 and are **[Proposed]**: no source measures them. The reasons are listed so the reader can disagree with a specific number.

| Candidate | Used how often (0.3) | Cost of mistakes prevented (0.3) | Reader can verify the output (0.2) | Ease of build and upkeep (0.2) | Weighted |
|---|---|---|---|---|---|
| **Test planning** | 5: every change | 4: wrong or missing tests let defects through and make every later check weaker | 4: a list of checks can be compared with an answer key and then run | 4: one skill file, reference rules, no tooling | **4.3** |
| Independent code review | 5: every change | 4 | 3: findings are harder to score; a reviewer asked for problems tends to find some (S10) | 4, but Claude Code already ships a fresh-context review command (S10) | 4.1 |
| PR verification / release readiness | 4 | 4 | 4: mostly pass/fail | 3: most of it belongs in CI, not in a skill | 3.8 |
| Design and implementation planning | 3: features and risky changes | 4 | 2: design quality is hard to check against a key | 3 | 3.1 |

**Recommendation: build test planning first.** Three reasons, in order of weight:

1. **It fixes the oracle before the code exists.** Models given a goal of passing tests will sometimes edit or special-case the tests **[Observed: S3]**, and self-repair is limited by the quality of the feedback **[Observed: S4, older-generation tools]**. A plan written from the acceptance criteria gives every later check, including review, something independent to check against. Review cannot recover that after the fact.
2. **A fresh-context review already exists as a built-in** **[T3: S10]**. Building a review skill first would mostly duplicate it. Nothing equivalent exists for test planning.
3. **Its output is the easiest to verify**, which matters for a first evaluation: planted risks either appear in the plan or they do not.

**Merge recommendation.** "PR verification" and "independent code review" should be **one** second skill (working name `review-change`). Both run on a finished diff, both read the same inputs (criteria, diff, test results), and both report against the quality bar. The mechanical half of PR verification belongs in CI, not in a skill.

**What would change this ranking**
- The D5 evaluation shows a plain request already matches the skill on test planning (skill average below plain + 1.5). Then build `review-change` first.
- The developer's own log shows that most defects that escaped were visible in the diff and not test-plan problems.
- A study shows same-model fresh-context review catches substantially more than self-review (C34 is currently unanswered). That would raise the review skill's value.

The scores are close (4.3 against 4.1). The choice rests on reasons 1 and 2, not on the 0.2 difference.

H2 verdict: **supported, with one addition**. The skill also has to guard test integrity, because that is the measured failure mode.

## 1. Spec: the `test-plan` skill

### When to invoke

- Before implementing a change that has a request or acceptance criteria (feature or bug fix).
- Before refactoring code that has few or no tests.
- On an existing diff, to find the checks that are missing or cannot be trusted.

### When not to invoke

- Changes with no behavior: formatting, comments, documentation.
- General review of a finished change for design and style. That is the second skill.
- To write the tests. The skill produces a plan and stops. Tests are written after the developer accepts the plan.

### What it reads, in order

1. The request, spec or acceptance criteria.
2. The code to be changed, or the diff. It searches for callers and for the same pattern elsewhere.
3. The existing tests for that code.
4. The project's test commands, CI config and real-environment checklist, if they exist.
5. Its own `reference.md` (the handbook rules it may cite).

### Steps

1. **Restate the change** in one line. List the acceptance criteria. If there are none, draft them and ask for confirmation before going on.
2. **Propose a tier** and name each escalation trigger that fired (RT rules). The developer decides the tier.
3. **List what could break.** Sources of risk, in this order:
   - each acceptance criterion, and its negative case (TP-1)
   - input partitions and boundaries (TP-2, TP-3)
   - connection points: where a value is produced and where it is consumed (TP-6)
   - existing behavior that must not change (TP-5)
   - the same pattern elsewhere in the code
4. **For each risk, name one check**: what it asserts, at which layer, and whether it already exists (cite the test) or has to be written.
5. **Order the work.** A failing reproduction comes first for a bug (TP-4). Characterization tests come before any change to untested code (TP-5).
6. **Diff mode only: integrity pass.** List any test deleted, ignored or given a new expected value; any real integration replaced by a mock; any expected value that could only have come from the implementation; any new dependency (TP-11, QB-3, QB-8).
7. **Say what this plan does not verify**, and what needs a real environment (TP-9).
8. **Ask the blocking questions** and stop.

### Output format

```
## Test plan: <one-line change>
Tier proposed: <1|2|3>. Triggers: <list or "none">.  (You decide.)
Acceptance criteria: <numbered list, or "drafted, please confirm">

### Findings
1. Finding: <what could break or is missing>
   Evidence: <file:line, test name, or spec sentence. "Not read" if not read>
   Proposed check: <what to assert> (<layer>) [exists: <test> | to write]
   Why it matters for this change: <one sentence>
   Handbook: <rule ID> (reference.md#<id>)

### Order of work
<numbered>

### Not verified by this plan
<list>

### Questions for you (blocking)
<numbered, or "none">
```

### Stop-and-ask points

The skill stops and asks the developer when:
- the tier is not obvious, or a trigger fired;
- the spec is ambiguous or silent on a behavior a test would have to assert (TP-12);
- an existing test would need a different expected value (TP-11);
- existing behavior looks like a bug that users might rely on;
- a check needs a new dependency or tool;
- a check needs a real environment the agent cannot reach.

### Guardrails against the failure modes found in RQ2

| Failure mode | Guardrail in the skill |
|---|---|
| Tests that restate the implementation (C7) | Expected values come from the criteria or the spec. The skill never runs the code to find out what to expect |
| Edited or weakened tests (C4) | Planning mode edits nothing. A changed expectation is always a stop-and-ask. When tests and spec conflict, the skill flags the conflict and stops (C5) |
| A passing check mistaken for proof (C6) | Every plan ends with "Not verified by this plan". For Tier 3, a mutation check on changed code is proposed (TP-8) |
| Invented references | Evidence must be a file and line the skill has read, or it says "not read". Only IDs present in `reference.md` may be cited |
| Gold-plating | Tier 1 plans stay under about 15 lines. No new tools or dependencies without asking |
| Mocks hiding the real integration | The plan names the one integration check that proves the pieces connect (TP-6) |

Skill text guides. It does not enforce. The hard versions of these rules (protected test files, dependency approval) belong in hooks and CI, per the D2 enforcement map.

### Rules the skill cites (IDs defined in `handbook/plan-tests.md`)

| ID | Rule | Source |
|---|---|---|
| TP-1 | Derive checks from the acceptance criteria, including negative cases, not from the implementation's output | [Standard] ISTQB §4.5.2 |
| TP-2 | Cover each input partition, valid and invalid | [Standard] ISTQB §4.2.1 |
| TP-3 | Test each boundary and its neighbors | [Standard] ISTQB §4.2.2 |
| TP-4 | Reproduce a bug with a failing test before fixing it | [Proposed]; fits regression testing in CSE 370 Unit 05 |
| TP-5 | Pin current behavior with characterization tests before changing untested code | [Proposed]; the term is from Feathers (2004), not opened in this run |
| TP-6 | Put each check at the lowest layer that can catch the failure, and name the integration check that proves the pieces connect | [Standard] ISTQB §5.1.6 |
| TP-7 | Test behavior through the public API so the test survives a refactor | [Established] *SWE at Google* ch. 12 |
| TP-8 | Ask whether the test would fail if the code were broken. For Tier 3, check with mutation testing on the changed code | [Established] eng-practices "Tests". Mutation evidence is partial (C11) |
| TP-9 | Write down what stays unverified and what needs a real environment | [Standard] ISTQB §1.3, principle 1 |
| TP-10 | Size the plan to the tier | [Standard] ISTQB §5.2.3 |
| TP-11 | Never change a test's expected value to make it pass. Stop and ask | [Observed] ImpossibleBench |
| TP-12 | Stop and ask when the spec is ambiguous or silent | [Observed] ImpossibleBench §5.3 (a "flag for human" option reduced test exploitation); otherwise [Proposed] |
| TP-13 | For "never" and "always" properties (never panics, a bound always holds), propose a property test | [Proposed]; no source opened |

## 2. Demo **[Illustrative, not yet tested]**

These are the outputs the design *should* produce, written by hand from the answer keys. Nothing here was run. The pilot in the build phase is the first real test.

### Case 1: `max_retries` over HTTP (new feature)

| Expected skill output (abridged) | Expected plain-agent output to "write tests for this" |
|---|---|
| **Tier proposed: 2.** Trigger: public API contract. | Goes straight to writing test code |
| 1. **Finding:** upper bound may be off by one. **Evidence:** `set_max_retries` uses `value < 10`; spec says "0 to 10 inclusive". **Check:** accept 0 and 10, reject -1 and 11 (unit) [to write]. **Why:** the documented maximum cannot be set. **TP-3** | Tests 0, 5 and 11. Likely misses 10 because the tests are written by reading the code |
| 2. **Finding:** malformed input is unspecified in tests: negative, float, string, null, missing field. A negative cast to `u8` wraps. **Check:** each returns 422 and nothing panics (unit for validation, handler level for JSON) [to write]. **TP-2** | One "invalid input" test with a single value |
| 3. **Finding:** the 422 status and error body are asserted only on the function. **Check:** send the request through the handler and assert status and body (integration) [to write]. **TP-6** | Asserts the `Err` variant of the function only |
| 4. **Finding:** `RetryWorker` reads the constant `MAX_RETRIES`, not the saved value. **Evidence:** `worker.rs`. **Check:** set the value, start a job, count retries (integration) [to write]. **Why:** the feature can pass every unit test and do nothing. **TP-6** | Not found. No test crosses from the config to the worker |
| 5. **Finding:** default of 3 when never set is untested (unit) [to write]. **TP-1** | Sometimes present |
| **Not verified:** jobs already running when the value changes; a write during a read. **Question:** should a change apply to jobs already in flight? | No statement of what is unverified. No questions |

**Differences to look for:** the skill starts from the spec, so it finds the bound and the worker gap; it places checks at layers; it says what is left. The plain output is code that mostly mirrors the implementation.

### Case 2: `truncate_title` (bug fix in existing code)

| Expected skill output (abridged) | Expected plain-agent output to "fix this and write tests" |
|---|---|
| **Tier proposed: 1.** No trigger. (Short plan.) | Rewrites the function straight away |
| **Question (blocking):** does `max` count bytes, `char`s or grapheme clusters? The doc comment says "characters"; the code counts bytes. **TP-12** | Picks one silently, often `chars()`, sometimes adding a dependency for graphemes |
| 1. **Finding:** no test reproduces the crash. **Check:** a title with a multi-byte character across `max` must not panic; write it first and watch it fail (unit) [to write]. **TP-4** | Adds an emoji test after the fix, never seen failing |
| 2. **Finding:** the cut path for ASCII is not pinned. **Check:** characterization tests for a short title and a cut title before changing the code (unit) [to write]. **TP-5** | Usually missing |
| 3. **Finding:** the same byte slice is in `preview_snippet`. **Evidence:** search result. **Check:** report it; fix in a separate change. **TP-5** | Not looked for, or fixed in the same diff without saying so |
| 4. **Finding:** "never panics" and "never longer than the limit" are properties. **Check:** a property test over arbitrary strings and limits. Needs a dependency, so it is a question, not a step. **TP-13** | Not proposed |
| 5. Edge values: `max = 0`, empty title, exactly `max` (unit). **TP-3** | One or two of these |

**Differences to look for:** the skill stops on the product decision instead of guessing; it orders the work (fail first, pin first); it stays short because the tier is low.

## 3. Evaluation protocol

The reader can rerun this. It measures whether the skill beats a plain request, which is the evidence the first-skill choice depends on.

### Cases

Eight synthetic cases with answer keys written before the research (file `02-synthetic-cases.md`):

| # | Case | Tier | New or existing | Type |
|---|---|---|---|---|
| 1 | `max_retries` over HTTP | 2 | New | Plan from a spec |
| 2 | `truncate_title` | 1 | Existing | Bug fix |
| 3 | User-ID migration | 3 | Existing | High-risk change |
| 4 | Agent-written feature diff | 2 | New | **Agent diff, AI-specific defects** |
| 5 | `page_count` fix | 1 | Existing | Proportionality check |
| 6 | `calculate_shipping` refactor | 2 | Existing, untested | Characterization first |
| 7 | Password-reset tokens | 3 (auth trigger) | New | Escalation |
| 8 | Agent "fixed CI" diff | 2 | Existing | **Agent diff, test integrity** |

Cases 4 and 8 are diffs. A review skill would suit them better, so they are also the fairest test of whether test planning is the right first skill.

### Conditions

- **Skill:** a fresh session with the `test-plan` skill available, given the case.
- **Plain:** a fresh session with no skill, given the case and a one-line request ("write tests for this", "fix this", or "review this diff").
- Same model, same repository state. **Three runs per case per condition** (48 outputs) to see variance.
- Neither condition can read the answer keys.

### Rubric (0 to 2 each, 8 points per output)

| Criterion | 0 | 1 | 2 |
|---|---|---|---|
| (a) Catches the important risk | Misses most blocking items in the key | Catches some blocking items | Catches every blocking item |
| (b) Grounded in evidence | Invented or missing references | Mostly real references, some vague | Every finding cites a real file, test or spec sentence |
| (c) Explains the reasoning | Bare list or code only | Some findings explain why | Each finding runs finding, evidence, action, why |
| (d) Proportional to the change | Far too much or far too little for the tier | Somewhat off | Depth matches the tier; nothing in the "over-reach" list |

**Also record per output:** false positives (findings that are wrong or irrelevant), invented references (files, tests or rule IDs that do not exist), and the minutes the developer needed to read and judge it.

### Scoring

- Shuffle the outputs and label them A and B so the scorer cannot tell which condition produced which. Strip the skill's rule IDs before scoring criterion (a), (c) and (d), because they give the condition away; score (b) on a copy with IDs kept.
- A separate judge model scores every output against the key and rubric. The developer hand-scores a sample of at least one output per case and records any disagreement of 2 points or more.

### Pass bar

All of these:
1. Skill average is at least plain average **+ 1.5 points** out of 8.
2. **Zero invented references** in the skill outputs.
3. On case 5, the skill averages at least 1.5 on criterion (d). A skill that gold-plates small fixes fails.
4. No case where the skill averages more than 1 point below plain.

If the bar is missed, D6 says what to do.
