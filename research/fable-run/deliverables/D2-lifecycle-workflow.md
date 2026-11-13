# D2. Lifecycle workflow

Supports **Decision 2** (which everyday workflow to adopt first). Source numbers (S1 to S18) are listed with links in [D3](D3-source-comparison.md). Claim numbers (C1 to C47) are rows in [04-claim-check.md](../04-claim-check.md).

Labels: **[Standard]** defined by a named standard (consensus, not proof that it works). **[Established]** long-standing practitioner guidance. **[Observed]** an empirical result; its study card is in D3. **[Proposed]** this report's own synthesis.

## 1. Main table: one change through the lifecycle

| Stage | Developer decision | Agent contribution | Minimum artifact | Evidence to proceed | Sign-off |
|---|---|---|---|---|---|
| 1. Problem | Is this worth doing, and what is out of scope? | Restate the problem; list what it could not find out | One-line problem statement plus a "not included" line | The developer can say who is affected and what "fixed" looks like | Developer |
| 2. Requirements | What must be true when it is done? Which tier is this (§2)? | Draft acceptance criteria; list ambiguities as questions; **propose** a tier | Given/When/Then acceptance criteria, including negative cases **[Standard: ISTQB CTFL v4.0.1 §4.5.2; S15]** | Each criterion is testable and no open question changes behavior | Developer (the tier is the developer's call) |
| 3. Design | What is the simplest design that meets the criteria? Is any decision hard to reverse? | Propose options and trade-offs; find existing code to reuse | Tier 1: none. Tier 2: a short plan (steps with a "done when" each). Tier 3: an ADR for each significant decision **[Established: Nygard 2011; S8]** | Each step has an exit check **[Standard: ISTQB §5.1.3 exit criteria; S15]** | Developer |
| 4. Test plan (written before the code changes) | What could break, which check would catch it, at which layer, and what stays unverified? | Run the `test-plan` skill (D5): derive checks from the criteria, not from the implementation | Test plan sized to the tier | Every acceptance criterion maps to at least one check; open product questions are answered | Developer |
| 5. Implementation | Accept, change or reject each diff | Write code and tests in small single-purpose changes **[Established: Google "Small CLs"; S6]** | The diff, with tests in the same change | Deterministic checks pass (§4, steps 1 to 5) | Agent OK for Tier 1 and 2 edits. Developer approves any changed test expectation or new dependency |
| 6. Verification | Is the evidence enough for this tier? | Run checks; run an independent review in a fresh context; report what was **not** verified | PR description: what changed, why, how it was verified, what was not | The verification order (§4) completed to the depth the tier requires | Developer. Tier 3 needs a line-by-line human review |
| 7. Release | Ship now? How do we undo it? | Draft the release note and rollback note | Tier 1 and 2: tagged release and a note. Tier 3: rollback note and a list of things to watch | A known last-good version exists to redeploy | Developer |
| 8. Maintenance | Did it work for users? What did we learn? | Summarize errors and issues; propose follow-up tasks | A line in the change log; an issue for anything deferred | The watch list from stage 7 is quiet, or an issue is open | Developer |

A passing stage produces evidence. It does not produce proof. "Tests pass" proves that the tests pass **[Standard: ISTQB §1.3, principles 1 and 7; S15]**.

## 2. Tier table

Depth scales with **risk** (likelihood times impact), not with the size of the diff **[Standard: ISTQB §5.2.3; S15]**. Practices are applied by risk, not as a fixed checklist **[Standard: NIST SP 800-218 v1.1 §2; S13]**. No tier skips a stage: a tier changes how deep a stage goes, never whether it happens **[Established: CSE 370 Unit 07]**. The three tier names and the triggers are **[Proposed]**.

| Stage | Tier 1: low risk (typical small bug fix) | Tier 2: normal (typical feature) | Tier 3: high risk (hard to reverse, or a trigger fired) |
|---|---|---|---|
| Problem | One line | One line plus "not included" | Same, plus who is affected if it goes wrong |
| Requirements | One failing example of the bug | Given/When/Then criteria with negative cases | Same, plus explicit non-functional criteria (data integrity, security) |
| Design | In your head | Short plan: steps with "done when" | ADR per significant decision; steps with done-when, dependencies and main risk |
| Test plan | A regression test that fails before the fix; boundary cases next to it | Checks per criterion at the right layer; the integration check named | Same, plus a rehearsal against production-like data and an explicit "cannot be proven in CI" list |
| Implementation | One small change | Small changes, refactors separate | Small changes behind a reversible step where possible |
| Verification | Deterministic checks; developer reads the diff | Adds independent AI review; mutation check on changed code is optional | Adds mutation check on changed code and a line-by-line human review. Agent review is extra, never a substitute |
| Release | Normal release | Normal release, note what changed | Rollback note, a point of no return named, a watch list |
| Maintenance | Nothing extra | Check the change log entry | Check the watch list after release; write up what happened |

### Escalation triggers [Proposed]

A trigger can raise a tier. Nothing lowers one except the developer writing down why.

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

Most triggers can be detected mechanically from the paths and contents of the diff, so a tier cannot be argued down in conversation. The agent may propose a tier. The developer decides.

## 3. Delegation ladder [Proposed]

No source defines a ladder (C21). This one is built from what the sources do show:
- Models asked to make tests pass will sometimes change or special-case the tests **[Observed: ImpossibleBench; S3]**.
- A developer's own sense of being faster with AI was wrong in a controlled trial **[Observed: METR; S1]**, so trust has to come from recorded results, not from how a session felt.
- Instruction files are not guaranteed to be followed; hooks are deterministic **[T3: Claude Code docs; S10, S12]**.

| Level | The agent may | The agent may not | Evidence that earns the next level |
|---|---|---|---|
| L0 Suggest | Explain, propose a plan, draft acceptance criteria and test plans | Edit files | The developer finds the plans mostly usable as written on 5 changes |
| L1 Supervised edits | Edit code and tests in an interactive session; the developer reviews every diff | Change test expectations, add dependencies, or commit without review | 10 changes with no blocking review finding and no test-integrity violation |
| L2 Bounded autonomous task | Work alone on a branch on a Tier 1 or 2 task that has written acceptance criteria and a pass/fail check **[T3: S10]** | Merge; touch protected paths; continue when tests conflict with the spec (it must stop and flag instead **[Observed: S3 §5.3]**) | 10 tasks merged after review with no blocking finding; a mutation check on a sample shows the tests are sensitive |
| L3 Pre-approved task classes | Run a named class of low-risk task end to end and open a PR (for example a patch-level dependency bump with a green suite) | Anything outside the named class | Not applicable. Review this level every quarter |

**Decisions the developer always keeps:** the tier; the acceptance criteria; any change to a test's expected value; any new dependency; merging Tier 3 work; releases and data migrations.

The counts (5, 10) are starting values to adjust, not measured thresholds. Keep a simple log (date, task, level, review findings). A blocking finding or a test-integrity violation drops the task class one level.

## 4. Verification order

Cheap, deterministic checks run first, so that review attention is spent on code that already builds and passes. No source opened in this run tests this order against running AI review in parallel, so the order itself is **[Proposed]**.

| # | Check | Tier | What it can show | What it cannot show |
|---|---|---|---|---|
| 1 | `cargo fmt --check`, `cargo check` | All | The code is formatted and compiles | Anything about behavior |
| 2 | `cargo clippy -- -D warnings`, plus a few chosen `restriction` lints such as `clippy::unwrap_used` **[T3: Clippy docs; S18; read by a worker only]** | All | Known bad patterns are absent | That the logic is right |
| 3 | `cargo test` | All | The tests pass | That the tests check the right behavior, or that users get the right result **[Established: S5 "Tests"]** |
| 4 | Test-integrity check: list changed or deleted tests, changed expected values, new `#[ignore]` | All | Whether the oracle moved during the change | Whether the move was right. The developer decides |
| 5 | `cargo deny check` (advisories, bans, licenses, sources) and confirmation of any new dependency **[T3: S18; read by a worker only]** | All | Dependencies meet policy; a new name is on the allowlist | That a dependency is the one you meant. Invented names are a measured risk in Python and JavaScript and **unmeasured for Rust** **[Observed: S2]** |
| 6 | `cargo mutants` on changed files **[T3: S18]** | Tier 2 optional, Tier 3 required | Places where the code can change without a test failing | Missing requirements. A surviving mutant is a prompt to look, not a defect count |
| 7 | Independent AI review in a fresh context. Inputs: acceptance criteria, diff, test results. Not the author's conversation **[T3: S10]** | Tier 2 and 3 | Mismatches with the spec, unhappy paths, tests that restate the implementation | It is not a control. Whether a same-model reviewer catches more than the author is untested (C34). Expect some false findings |
| 8 | Human review. Read every line **[Established: S5 "Every Line"]**; go deepest where the triggers point | All (depth by tier) | Design fit, simplicity, whether the change should exist | Runtime behavior in the real environment |
| 9 | Real-environment checks: real hardware, production-like data, real users | When CI cannot reach it | What simulation and CI cannot **[Established: CSE 370 Unit 04]** | Everything else. Keep a written list of these checks per project |

## 5. Enforcement map

A rule that must always hold is enforced mechanically or is explicitly left to a person. Skill text explains and recommends. It does not enforce **[T3: S10 "Enforce the rules that can't have exceptions"]**.

| Rule | Layer | Why it sits there |
|---|---|---|
| Code is formatted, compiles, and passes lints and tests | **CI** (and a local pre-commit run) | Deterministic and cheap; the result is a record |
| No `unwrap` or `expect` on input or I/O in production paths | **CI** lint (`clippy::unwrap_used`, `clippy::expect_used` outside tests); a justified `#[allow]` needs a comment and **human** approval | A lint finds the pattern; only a person can judge an exception |
| Dependencies have no known advisory, an allowed license and an allowed source | **CI** (`cargo deny check`) | Policy is data, so a tool can check it |
| No new dependency without the developer's approval | **Hook** (ask before an agent edits `Cargo.toml`) plus **CI** (allowlist) plus **human** | Invented package names are a measured failure **[Observed: S2]**; the approval is a judgment |
| An agent does not edit protected test files or change expected values on its own | **Hook** (block or ask on protected paths **[T3: S12]**) plus **CI** flag (step 4) plus **human** approval | Models do edit tests to pass **[Observed: S3]**. Read-only tests alone did not stop special-casing, so a person still checks |
| No secrets in the repository | **Hook or CI** secret scan | Deterministic. **[Proposed]**: no scanner was evaluated in this run |
| The tier and its triggers | **CI or skill checklist** detects triggers; the **human** decides the tier | Detection is mechanical; the decision belongs to the developer |
| Tests come from the acceptance criteria, not from the implementation's output | **Skill checklist** plus **human** | Needs judgment; no tool detects a mirrored test reliably |
| One concern per change; refactors separate | **Skill checklist** plus **human**; optional CI warning on diff size | Judgment, with a size signal |
| Comments and docs match the code | **AI review** plus **human** | Judgment |
| Tier 3 changes get a line-by-line human review and a rollback note before merge | **Human**; branch protection where the host offers it | The point is a person's attention. An AI reviewer does not satisfy it |
