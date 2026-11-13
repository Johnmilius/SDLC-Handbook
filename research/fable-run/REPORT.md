# SDLC handbook research report

Assembled 2026-11-13 05:35 MST from the seven deliverables in `deliverables/`, in order, followed by the claim-check table. Research plan v1.3.

| Part | File |
|---|---|
| D1 | [deliverables/D1-recommendation.md](deliverables/D1-recommendation.md) |
| D2 | [deliverables/D2-lifecycle-workflow.md](deliverables/D2-lifecycle-workflow.md) |
| D3 | [deliverables/D3-source-comparison.md](deliverables/D3-source-comparison.md) |
| D4 | [deliverables/D4-handbook-blueprint.md](deliverables/D4-handbook-blueprint.md) |
| D5 | [deliverables/D5-first-skill.md](deliverables/D5-first-skill.md) |
| D6 | [deliverables/D6-first-milestone.md](deliverables/D6-first-milestone.md) |
| D7 | [deliverables/D7-run-report.md](deliverables/D7-run-report.md) |
| Appendix | [04-claim-check.md](04-claim-check.md) |

Links inside the parts below were written relative to `deliverables/`. Open the files above to follow them.

---

# D1. Recommendation

The recommendation is the first two sections (about 250 words). The one-line verdicts follow. Details and evidence are in D2 to D6. Claim numbers refer to [04-claim-check.md](../04-claim-check.md).

## The three decisions

1. **What the handbook contains.** A short field guide plus self-contained agent skills. Twelve pages are planned and eight are built first. Each page answers one decision, and each rule has a stable ID, a label for its evidence, and a source. One page, the quality bar, is the single reference for "good enough to merge".
2. **Which workflow to adopt first.** Three risk tiers that change how deep each stage goes and never skip a stage. Deterministic checks run first, then an independent AI review, then human review. Hard rules sit in hooks and CI, not in instructions.
3. **What to build first.** A `test-plan` skill. For a spec or a diff it says what could break, which check would catch it and at which layer, and what stays unverified. It is evaluated on eight synthetic cases against a plain request.

## The tradeoffs that drove this

- **Test planning before review.** The scores are close (4.3 against 4.1). Test planning wins because it fixes the oracle before the code exists, and a fresh-context review command already ships with the tool.
- **Short and cited before complete.** Large documents go stale. The cost is that four pages wait for v2.
- **Honest standards coverage before broad coverage.** Only standards whose text was read are cited by clause. SOC 2 and the ISO/IEEE standards were not read and are marked so.

## Verdicts on the hypotheses

| # | Verdict | One line |
|---|---|---|
| H1 | Supported | Short guide plus skills (C1, C2, C3) |
| H2 | Supported, with an addition | Test planning first; it must also guard test integrity (C4) |
| H3 | Modified | Layout holds, but each skill carries its own `reference.md`, because links back to `handbook/` break when a skill is copied |
| H4 | Modified | Links run through stable rule IDs; a skill may cite only IDs in its `reference.md` |
| H5 | Modified | Scale by risk (likelihood and impact), not by size (C12) |
| H6 | Supported in part | "Done when" has standard backing (C15). Dependencies and risk per step are [Proposed], used for Tier 3 only |
| H7 | Supported as [Proposed] | The order is reasoned from cost. No source tests it against parallel review |
| H8 | Modified | Independence of inputs is vendor guidance, untested for a same-model reviewer (C34). It is a filter, not a control |
| H9 | Supported | Instruction files are not guaranteed; hooks are deterministic (C18, C19) |
| H10 | Supported as [Proposed] | Tiering scales depth and never skips a stage (C14). The triggers have no source (C16) |
| H11 | Supported in part | Works for the free standards that were read. Not shown for paywalled ones or SOC 2 (C42, C44) |

## Verdicts on the outside claims (§3b)

| Claim | Verdict |
|---|---|
| Intent and specs as code | Adapt. Write criteria first, at the smallest size the tier needs. No outcome evidence found (C37) |
| Steering and skills files auto-load guardrails | Modified. Skills load by description match, and instructions are not guaranteed to be followed. Enforce with hooks |
| TDD and DDD baked in | TDD: supported, with test-integrity guardrails. DDD: not researched; left out of v1 |
| AI picks the lifecycle and skips stages | Rejected. No evidence found, and it contradicts Unit 07 |
| Separate author and reviewer | Supported in direction. An AI reviewer does not replace human review |
| Deterministic anchors; validate dependencies | Supported for dependency validation (measured for Python and JavaScript, not Rust). The order is [Proposed] |
| Adversarial AI testing | Not researched. Use it as a source of test ideas and check test quality with mutation testing |
| Humans focus review on high-risk areas | Supported as "risk sets depth". It does not mean skipping lines |
| "Drastically accelerates velocity" | Unsupported. The one trial opened found experienced developers 19% slower (study card in D3) |
| Enterprise framing | Marked [Large-org only] and kept out of v1 |

---

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

---

# D3. Source and system comparison

Supports **Decision 1** (what the handbook contains). Claim numbers refer to [04-claim-check.md](../04-claim-check.md).

## 1. Core sources (18)

Tiers: T1 official standard, original practitioner guidance, or peer-reviewed. T2 labeled preprint or public repository artifact. T3 vendor documentation (how a feature works, not how well). "Read by" says who opened it: **L** the lead, **W** a worker only.

| # | Source | Date / version | Tier | Read by | Link |
|---|---|---|---|---|---|
| S1 | Becker et al. (METR), "Measuring the Impact of Early-2025 AI on Experienced Open-Source Developer Productivity" | arXiv 2507.09089v2, 25 Jul 2025 | T2 | L | https://arxiv.org/abs/2507.09089 |
| S2 | Spracklen et al., "We Have a Package for You! A Comprehensive Analysis of Package Hallucinations by Code Generating LLMs" | USENIX Security 2025; arXiv 2406.10279v3, 2 Mar 2025 | T1 | L | https://arxiv.org/abs/2406.10279 |
| S3 | Zhong, Raghunathan, Carlini, "ImpossibleBench: Measuring LLMs' Propensity of Exploiting Test Cases" | arXiv 2510.20270v1, 23 Oct 2025 | T2 | L | https://arxiv.org/abs/2510.20270 |
| S4 | Olausson et al., "Is Self-Repair a Silver Bullet for Code Generation?" | ICLR 2024; arXiv 2306.09896v5, 2 Feb 2024 | T1 | L (abstract) | https://arxiv.org/abs/2306.09896 |
| S5 | Google Engineering Practices, "What to look for in a code review" | Undated; fetched 2026-11-13 | T1 | L | https://google.github.io/eng-practices/review/reviewer/looking-for.html |
| S6 | Google Engineering Practices, "Small CLs" | Undated; fetched 2026-11-13 | T1 | L | https://google.github.io/eng-practices/review/developer/small-cls.html |
| S7 | *Software Engineering at Google*, ch. 12 "Unit Testing" (Kuefler; ed. Manshreck) | Online edition; fetched 2026-11-13 | T1 | L | https://abseil.io/resources/swe-book/html/ch12.html |
| S8 | Nygard, "Documenting Architecture Decisions" | 15 Nov 2011 | T1 | L | https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions |
| S9 | Google SRE book, ch. 8 "Release Engineering" | Online edition; fetched 2026-11-13 | T1 | L | https://sre.google/sre-book/release-engineering/ |
| S10 | Anthropic, "Best practices for Claude Code" | Fetched 2026-11-13 | T3 | L | https://code.claude.com/docs/en/best-practices |
| S11 | Anthropic, Claude Code skills documentation | Fetched 2026-11-13 | T3 | L | https://code.claude.com/docs/en/skills |
| S12 | Anthropic, Claude Code hooks guide | Fetched 2026-11-13 | T3 | L | https://code.claude.com/docs/en/hooks-guide |
| S13 | NIST SP 800-218, Secure Software Development Framework (SSDF) | v1.1, Feb 2022 | T1 (standard) | L | https://csrc.nist.gov/pubs/sp/800/218/final |
| S14 | NIST SP 800-218A, SSDF Community Profile for Generative AI and Dual-Use Foundation Models | Final, 26 Jul 2024 | T1 (standard) | L | https://csrc.nist.gov/pubs/sp/800/218/a/final |
| S15 | ISTQB Certified Tester Foundation Level syllabus | v4.0.1, 15 Sep 2024 | T1 (standard) | L | https://www.istqb.org/certifications/certified-tester-foundation-level |
| S16 | SLSA specification | v1.2 | T1 (standard) | L | https://slsa.dev/spec/v1.2/ |
| S17 | Petrovic et al., "Does mutation testing improve testing practices?" | ICSE 2021; arXiv 2103.07189, 12 Mar 2021 | T1 | W (abstract) | https://arxiv.org/abs/2103.07189 |
| S18 | Rust tool documentation: Clippy lint groups; cargo-deny checks; cargo-mutants | Fetched 2026-11-13 | T3 | W | https://doc.rust-lang.org/clippy/lints.html · https://embarkstudios.github.io/cargo-deny/checks/index.html · https://mutants.rs/welcome.html |

**Supporting sources, read by a worker only and used with a caveat:** Cucumber Gherkin reference (https://cucumber.io/docs/gherkin/reference/); GitHub Spec Kit README (https://github.com/github/spec-kit); Huang et al., ICLR 2024, abstract (https://arxiv.org/abs/2310.01798); OWASP ASVS 5.0.0 landing page (https://asvs.owasp.org); Shape Up ch. 3 (https://basecamp.com/shapeup/1.2-chapter-03); Ogenrwot and Businge, MSR 2026 (https://arxiv.org/abs/2601.17581).

## 2. Study cards for every [Observed] source

| Source | Task | Participants | Tools / models, date | Measured | Result | Why it may not carry over to a junior solo Rust developer using 2026 agents |
|---|---|---|---|---|---|---|
| S1 METR | 246 real issues in mature open-source repositories, each randomly assigned to AI-allowed or AI-disallowed | 16 experienced developers working on their own repositories | Mainly Cursor Pro with Claude 3.5 and 3.7 Sonnet; Feb to Jun 2025 | **Speed** (time to complete) and forecasts of speed | 19% longer with AI allowed. Developers expected to be 24% faster and afterwards believed they had been 20% faster | Small sample of experts on code they know well. The authors say unfamiliar code and greenfield work may differ (untested). Tools are a generation old |
| S2 Spracklen et al. | Generate code from 2 prompt datasets and check whether referenced packages exist | No human participants. 16 code models, 576,000 samples | 2024 model set **[Older-generation tools]** | **Correctness / supply-chain security**: rate of non-existent package names | At least 5.2% for commercial models and 21.7% for open-source models; 205,474 unique invented names; 43% of invented names recurred on all 10 repeats | **Python and JavaScript only.** Rust appears only as a source of cross-language confusion. Single-shot generation, not an agent that builds and sees the error |
| S3 ImpossibleBench | Coding tasks altered so the tests contradict the specification; any pass means the tests were exploited | No human participants. Several frontier models | 2025 models, including GPT-5, o3, Claude models | **Correctness / integrity**: rate of passing impossible tasks | GPT-5 passed 76% of one SWE-bench variant and 2.9% of a LiveCodeBench variant. Methods: editing tests, overloading comparison, special-casing. A strict prompt, hidden tests, or a "flag for human" option each cut the rate sharply for some models | Tasks are built to be impossible, so rates do not predict everyday frequency. Large differences between models and prompts |
| S4 Olausson et al. | Repair failing generated programs on HumanEval and APPS | No human participants, plus a small human-feedback study | Code Llama, GPT-3.5, GPT-4 **[Older-generation tools]** | **Correctness** (pass rate, with the cost of repair counted) | Gains from self-repair are often modest. Feedback from a stronger model gives much larger gains | Benchmark puzzles and 2023 models. Abstract only was read |
| S17 Petrovic et al. | Mutation testing shown to developers during code review | Google developers (count not seen) | Google's internal system, to 2021 | **Test quality**: tests written, surviving mutants | Developers exposed to mutants wrote more tests; mutants are coupled with real faults | Abstract only, read by a worker. One company, observational |

Speed (S1), correctness (S2, S3, S4) and test quality (S17) are different measures. This report never combines them into one "productivity" claim, and it makes no speed claim for the reader.

## 3. System comparison

| System / source | Area | What it supports | Evidence (tier) | Portability | Maintenance cost | Fit at my scale | Verdict | Why |
|---|---|---|---|---|---|---|---|---|
| Given/When/Then acceptance criteria (ISTQB §4.5.2; Gherkin) | Requirements | Criteria as test conditions, with negative cases | T1 standard; consensus | High: plain text | Low | High | **Adopt** | Already in the reader's course notes; feeds the test plan directly |
| ADR (Nygard, S8) | Design | A one-to-two-page record per significant decision | T1 guidance; no outcome study | High: one markdown file | Low if only for significant decisions | High for Tier 3, wasteful for Tier 1 | **Adapt** | Use for Tier 3 and hard-to-reverse decisions only |
| Spec-driven tooling (Spec Kit) | Requirements, agents | A fixed sequence: principles, spec, plan, tasks, implement, check | T2 method only; **no outcome evidence opened** (C37) | Tied to specific agent tooling | Medium to high: several files per feature can drift | Low for a solo developer as packaged | **Leave** the tool; **adapt** the idea | Keep "write the criteria before the code, in the repo", at the smallest size the tier needs |
| Google code-review guidance (S5, S6) | Verification | What a reviewer looks for; small single-purpose changes | T1 guidance from one large organization | High | Low | High, except that it assumes a second person | **Adopt** | The quality bar is built on it |
| *Software Engineering at Google* ch. 12 (S7) | Testing | Test behaviors through the public API; avoid brittle tests | T1 guidance | High | Low | High | **Adopt** | Directly counters tests that restate the implementation |
| ISTQB CTFL (S15) | Testing | Named techniques, risk-based depth, exit criteria, independence levels | T1 consensus syllabus; free | High | Low | High as vocabulary; the certification itself is not needed | **Adopt** (as citable criteria) | Free, sectioned, and matches CSE 370 Unit 04 |
| Mutation testing (S17; cargo-mutants) | Testing | An objective signal of test sensitivity | T1 abstract (partly, C11); T3 tool docs | Rust tool exists | Medium: slow on large code | Good on changed files only | **Adapt** | Run on changed code for Tier 3; optional for Tier 2 |
| Google SRE release chapter (S9) | Release | Small frequent releases; define a release process at any size | T1 guidance | Ideas are portable; the tooling is not | Low for the ideas | Canary and progressive rollout need several instances | **Adapt** | Keep tagged releases and a last-good version. Mark fleet rollout **[Large-org only]** |
| NIST SSDF (S13) | Secure development | Named practices for review, testing, component vetting, protecting code, keeping records | T1 standard; consensus | High | Low as a crosswalk | High if used by risk, as §2 intends | **Adapt** | Cite the practice IDs the workflow already satisfies |
| SLSA (S16) | Supply chain | Levels for build provenance and source control | T1 specification | Depends on the CI host | Low at Build L1 to L2 | Build L1 to L2 is reachable. Source L4 (two-party review) is not, for one person | **Adapt** | State the level honestly; do not claim Source L4 |
| Claude Code best practices, skills, hooks (S10 to S12) | Agent workflow | Short instruction files; skills loaded by description; hooks for hard rules; reviewer in a fresh context | T3: how the product works | Skills use an open format; hooks are tool-specific | Medium: vendor docs change | High | **Adopt** for mechanics | Not evidence that any of it improves outcomes |
| "AI picks the lifecycle and skips stages" (outside summary) | Agent workflow | Nothing found | None | n/a | n/a | n/a | **Leave** | Contradicts Unit 07, and S3 shows models taking shortcuts when goals conflict |
| DORA metrics, DORA AI report, GitClear, Perry et al., Peng et al. | AI and delivery | Not opened in this run | n/a | n/a | n/a | n/a | **Leave for now** | No claim is made from them. See gaps |

## 4. D3b. Standards crosswalk

A standard records expert consensus or defines what an auditor checks. **It is not evidence that a practice improves outcomes.** "Read" means the lead read the clause text in this run. Standards that were not read have no clause cited.

| Standard (edition) | Body | Governs | Authority type | Access | Read? | Clause(s) to borrow | Handbook page it backs | Solo take | Large-org only? | AI relevance |
|---|---|---|---|---|---|---|---|---|---|---|
| NIST SP 800-218 SSDF (v1.1, 2022) | NIST | Secure development practices | Consensus guidance; not an audit | Free | **Yes** | §2 (risk-based, not a checklist); PO.3.3 (tools generate an audit trail); PW.4.4 (verify third-party components); PW.7 (review and/or analyze code); PW.8 (test executable code); PS.1 (protect code); PS.3 (archive releases); RV.1 (find vulnerabilities continuously) | Quality bar; Verify a change; Release | Your CI log and PR history already are the PO.3.3 audit trail | No. Role and training tasks in PO.2 are organizational | Indirect, through SP 800-218A |
| NIST SP 800-218A (2024) | NIST | SSDF profile for building AI models and AI systems | Consensus guidance | Free | **Yes** (audience, scope) | Scope statement: no distinction between human-written and AI-generated code; all code is evaluated before use | Quality bar; Working with an agent | Review agent-written code to the same bar as your own | The practices target AI model and system producers | Direct, but not for users of coding tools |
| ISTQB CTFL syllabus (v4.0.1, 2024) | ISTQB | Testing vocabulary and techniques | Consensus syllabus (certification body) | Free | **Yes** | §1.3 (principles); §1.5.3 (independence of testing); §4.2.1 and §4.2.2 (partitions, boundaries); §4.5.2 (acceptance criteria); §5.1.3 (exit criteria); §5.1.6 (test pyramid); §5.2.3 and §5.2.4 (product risk analysis and control) | Plan tests; Risk tiers; Verify a change | Names and section numbers for techniques you already learned in Unit 04 | No | None |
| SLSA (v1.2) | OpenSSF | Supply-chain integrity levels | Specification, self-assessed | Free | **Yes** | Build L1 (provenance exists), L2 (hosted build signs provenance), L3 (hardened); Source L1 to L4 (L4 is two-party review) | Release | Build on hosted CI and keep provenance: Build L1 to L2 | Build L3 and Source L4 need more than one person | None seen |
| OWASP ASVS (5.0.0, May 2025) | OWASP | Application security requirements | Community consensus | Free (CC BY-SA 4.0) | Edition only (worker) | **None cited.** Requirement text not read | (v2: security review checklist) | Use chapters as a review checklist for web and API code | No | None seen |
| SOC 2 / AICPA Trust Services Criteria (2017, revised points of focus 2022) | AICPA | Criteria an auditor reports against | Audit / attestation framework | Free with a login | **No** (login wall) | **None cited.** CC8.1 and CC7.x were not read | (audit-evidence map below, marked unverified) | Read the free PDF before relying on any mapping | The audit itself is organizational | Unknown |
| ISO/IEC/IEEE 29148 (requirements) | ISO/IEC/IEEE | Requirements engineering | Consensus standard | Paywalled | **No** | None cited | Would back "Define the change" | CSE 370 Unit 02's four criteria are used instead | No | Unknown |
| ISO/IEC 25010 (2023) (product quality) | ISO/IEC | Quality model | Consensus standard | Paywalled (HTTP 403) | **No**; characteristic names seen only on a third-party page | None cited | Could structure the quality bar | Not used in v1 | No | Unknown |
| ISO/IEC/IEEE 12207, 42010, 29119; IEEE 1016 | ISO/IEC/IEEE | Life cycle, architecture description, testing, design descriptions | Consensus standards | Paywalled | **No** | None cited | n/a | Not used in v1 | Process standards lean large-org | Unknown |
| SWEBOK v4; OWASP SAMM; CISA Secure by Design; OpenSSF Scorecard | Various | Body of knowledge; maturity model; principles; automated repo checks | Consensus / automated checks | Believed free | **No** | None cited | n/a | Candidates for v2 | SAMM leans large-org | Unknown |
| ISO/IEC 27001 Annex A; ISO/IEC 42001; NIST AI RMF and AI 600-1 | ISO/IEC; NIST | Security management; AI management; AI risk | Certifiable management systems; risk framework | ISO paywalled; NIST free | **No** | None cited | n/a | Not used in v1 | **[Large-org only]** (management systems) | Organizational AI governance |

### Audit-evidence map

SOC 2 is, as the plan describes it, an attestation report by an auditor against the AICPA Trust Services Criteria, not a certification or an engineering standard. **The criteria text was not read in this run**, so this map does not state what CC8.1 requires. It maps the everyday workflow to the record-keeping practices that *were* read (SSDF and SLSA). The SOC 2 column stays empty until the developer reads the free PDF.

| Expectation that was read | Clause | Everyday artifact that already provides it | Gap for a solo developer |
|---|---|---|---|
| Tools produce an audit trail of the development practices | SSDF PO.3.3 | CI run logs; PR history; commit history | None, if CI logs are kept |
| Code is reviewed and/or analyzed, and issues are recorded and triaged | SSDF PW.7.1, PW.7.2 | Clippy and review findings in the PR; issues opened for deferred items | The reviewer is the author. See below |
| Testing is scoped, designed, performed and its results documented | SSDF PW.8.2 | The test plan; the CI test log attached to the PR | None |
| Third-party components are verified against your requirements | SSDF PW.4.4 | `cargo deny` log; the dependency-approval line in the PR | None |
| Each release is archived with integrity and provenance data | SSDF PS.3.1; SLSA Build L1 to L2 | Tagged release; build provenance from hosted CI | Build L3 is out of reach without a hardened platform |
| Code is protected from unauthorized change | SSDF PS.1.1; SLSA Source L1 to L3 | Version control with protected branches | None |
| Changes to protected branches are reviewed by two trusted people | SLSA Source L4 | **Nothing.** | **Cannot be closed by one person** |
| Significant decisions are recorded | (no clause read) | ADRs | n/a |

**Separation of duties.** One person cannot be both author and independent approver, and no workflow change fixes that. An AI reviewer is not a second trusted person: SLSA Source L4 asks for two trusted *persons*, and ISTQB §1.5.3 describes independence between people. This report found no standard that accepts an AI review as a control, and it does not claim one. What a solo developer can credibly do is **[Proposed]**:
- keep the evidence that a second person would look at (criteria, test plan, CI logs, review findings, the decision);
- let time pass and re-read Tier 3 diffs in a fresh session before merging;
- bring in a human reviewer for Tier 3 changes when a customer or auditor requires it, and say plainly in any questionnaire that routine changes are self-approved.

### Where standards and practice disagree

- **Read every line, or focus on risk?** Google's guidance says to look at every line of human-written code you are asked to review (S5, "Every Line"). ISTQB §5.2.3 says risk decides depth. These fit together: risk decides how deep you go, not which lines you skip. The outside summary's "humans focus review on high-risk areas" is safe only in that sense.
- **Standards assume more than one person.** SLSA Source L4 and ISTQB's higher independence levels cannot be met alone. The handbook states the gap instead of papering over it.
- **Heavy documentation versus lean artifacts.** Nygard argues that large documents are not kept current (S8). The formal documentation standards (IEEE 1016, ISO/IEC/IEEE 29119) were not read, so this report cannot compare them fairly. A known criticism of 29119 from the context-driven testing community was recalled by a worker but not sourced (C45). It is recorded as an unverified lead only.

## 5. Disagreements and gaps

| Topic | What the sources say | Compared on | Effect on the handbook |
|---|---|---|---|
| Speed | Only S1 was opened: experienced developers were slower. Vendor studies in the plan's seed list report gains but were not opened | Participants (experts on their own code vs others), task (real issues vs bounded tasks), tool generation | The handbook makes **no speed claim** either way and tells the reader to measure their own tasks |
| Self-review | Vendor guidance recommends a fresh-context reviewer (S10). Research shows self-repair is weak and stronger or human feedback helps (S4). Nothing tests a same-model fresh-context reviewer (C34) | Vendor guidance vs benchmark study; 2023 models | Independent AI review is an inexpensive extra filter, **not a control**. Human review stays for Tier 3 |
| Agent test-gaming | S3 measures it on impossible tasks. No field study of everyday frequency was opened | Benchmark vs real work | Enough to justify protecting tests and planning them from the spec. Not enough to state how often it happens |

**Gaps that could change the handbook design**

| Gap | Could change | How much |
|---|---|---|
| No outcome evidence for spec-first agent workflows (C37) | How much specification the workflow asks for | Low: the handbook already asks for the smallest artifact per tier |
| No measured evidence for most agent-specific defects (C31): duplication, scope creep, mirrored tests, wrong comments | Which items the quality bar treats as agent-specific | Medium: they stay in the bar as review items, labeled unmeasured |
| SOC 2 criteria and the ISO/IEEE standards were not read (C42, C44) | The crosswalk's coverage and the audit-evidence map | Medium for RQ6: the crosswalk is narrower than the plan intended. Low for the workflow |
| No evidence on whether a same-model fresh-context review beats self-review (C34) | Whether the second skill (review) is worth building | Medium: the reader's own evaluation has to answer it |
| No field data on how often the first skill beats a plain request | The first-skill choice | High, and it is exactly what the D5 evaluation measures |
| Rust-specific package hallucination rate (C26) | How strict the dependency rule is | Low: the rule is cheap either way |

---

# D4. Handbook blueprint

Supports **Decision 1** (what the handbook contains and how it is organized). Source numbers are in [D3](D3-source-comparison.md); claim numbers in [04-claim-check.md](../04-claim-check.md).

## 1. The quality bar page (in full)

This is the text of `handbook/quality-bar.md`. The handbook, the skills and CI all point to it. No other page restates it.

---

> # Quality bar
>
> **Decision this page supports:** is this change good enough to merge?
>
> "The tests pass" is the entry ticket, not the bar. A change meets the bar when every row below holds, or the exception is written in the PR.
>
> | ID | Rule | Enforced by | Source |
> |---|---|---|---|
> | QB-1 | It does what the acceptance criteria say, including the negative cases | Skill checklist, human | [Standard] ISTQB CTFL v4.0.1 §4.5.2 |
> | QB-2 | Its tests would fail if the code were broken. They check behavior through the public API, not the implementation's current output | Skill checklist, human. Mutation check in CI for Tier 3 | [Established] Google eng-practices, "Tests"; *SWE at Google* ch. 12 |
> | QB-3 | No test was deleted, ignored or given a new expected value without the developer's explicit approval | Hook, CI flag, human | [Observed] ImpossibleBench (2025): models edit or special-case tests to pass |
> | QB-4 | Unhappy paths are covered: invalid, empty, boundary and failing inputs | Skill checklist, human | [Standard] ISTQB §4.2.1, §4.2.2. [Established] eng-practices, "Functionality" |
> | QB-5 | Errors are handled, not swallowed. No `unwrap` or `expect` on input or I/O in production paths | CI lint; a human approves any `#[allow]` | [Proposed]. Lint mechanics: Clippy docs (T3) |
> | QB-6 | It is the simplest design that works. No speculative features | Human, with AI review as a prompt | [Established] eng-practices, "Complexity" |
> | QB-7 | One concern per change. Refactors are separate. Tests are in the same change. No dead code, leftover debugging or unrelated edits | Skill checklist, human. Optional CI size warning | [Established] eng-practices, "Small CLs". Dead-code item is [Proposed] |
> | QB-8 | Every new or upgraded dependency exists, is the one intended, is justified in the PR, and passes policy | Hook, CI (`cargo deny`), human | [Observed] Spracklen et al. (2025): invented package names, measured for Python and JavaScript, unmeasured for Rust. [Standard] NIST SP 800-218 v1.1 PW.4.4 |
> | QB-9 | It is formatted, compiles, is lint-clean, and the tests pass | CI | [Standard] NIST SP 800-218 v1.1 PW.7, PW.8 |
> | QB-10 | Names, comments and docs say what the code actually does. Comments explain why | AI review, human | [Established] eng-practices, "Naming", "Comments", "Documentation" |
> | QB-11 | Any `unsafe` is minimal, justified in a comment, and checked | Human. Miri in CI where it applies | [Proposed]. Miri docs were not opened in the research run |
> | QB-12 | Agent-written code meets the same bar as hand-written code. "An agent wrote it" and "CI is green" are not reasons to approve | Human | [Standard] NIST SP 800-218A, Scope. [Established] eng-practices, "Tests" |
> | QB-13 | What was not verified is written down | Skill output, PR description | [Standard] ISTQB §1.3, principle 1 |
>
> ## Defects to look for in agent-written code
>
> | Defect | Evidence status |
> |---|---|
> | Invented or near-miss package names | **Measured** for Python and JavaScript; not for Rust (Spracklen et al.) |
> | Tests edited, special-cased or weakened so they pass | **Measured** on benchmark tasks built to be impossible; everyday rate unknown (ImpossibleBench) |
> | Tests that restate the implementation; happy-path-only logic; duplicated code; scope creep; plausible but wrong API use; comments that do not match the code | **Not measured in the research run.** Treat as review prompts, not as known rates |

---

That page is 13 rules and one small table, which fits on one page. Every rule names its enforcement layer and its source, and every "must always" rule (QB-3, QB-5, QB-8, QB-9) has a mechanical layer.

ISO/IEC 25010 was considered as the structure for this page. It was not read (paywalled), so it is not used (C44).

## 2. Table of contents

Twelve short pages. **v1** builds the eight marked in the last column (D6). Each rule on a page gets a stable ID with the page's prefix.

| # | Page | Prefix | Decision it supports | Main sources | Standard clause it aligns with | CSE 370 unit (checked in the notes) | v1? |
|---|---|---|---|---|---|---|---|
| 1 | `handbook/quality-bar.md` | QB | Is this change good enough to merge? | S5, S6, S7, S2, S3 | SSDF PW.4.4, PW.7, PW.8; SP 800-218A Scope; ISTQB §4.2, §4.5.2 | 04 (code review, static analysis, coverage is gameable) | **Yes** |
| 2 | `handbook/workflow.md` | WF | What do I do next on this change, and what do I need before moving on? | D2 main table; S8, S15 | ISTQB §5.1.3 | 07 (every phase gets attention), 02, 03 | **Yes** |
| 3 | `handbook/risk-tiers.md` | RT | How much process does this change need? | S15, S13 | ISTQB §5.2.3; SSDF §2 | 04 ("how much testing is enough"), 07 | **Yes** |
| 4 | `handbook/plan-tests.md` | TP | What could break, and which check would catch it? | S7, S15, S5, S17 | ISTQB §4.2.1, §4.2.2, §4.5.2, §5.1.6 | 04 (boundary values, partitions, pyramid, traceability) | **Yes** |
| 5 | `handbook/verify-a-change.md` | VC | Is the evidence enough for this tier? | D2 verification order; S5, S18, S10 | SSDF PW.7, PW.8; ISTQB §1.3 | 04 (where tests run), 07 (shift left) | **Yes** |
| 6 | `handbook/working-with-agents.md` | AG | What may the agent do alone, and what enforces the rules? | S1, S3, S4, S10, S11, S12 | SP 800-218A Scope; ISTQB §1.5.3 | 03 ("design in an AI world"), 04 (you own correctness) | **Yes** |
| 7 | `handbook/standards-crosswalk.md` | SC | Which recognized standard backs this practice, and what evidence do I already have? | D3b; S13, S14, S15, S16 | As listed in D3b | 04 (traceability, audit trail) | **Yes** |
| 8 | `handbook/sources.md` | SRC | Where did this claim come from, and how strong is it? | D3 source list and study cards | n/a | n/a | **Yes** |
| 9 | `handbook/define-the-change.md` | DC | What exactly am I building, and what is out of scope? | S15; Gherkin reference | ISTQB §4.5.2 | 02 (Given/When/Then, the four quality criteria) | v2 |
| 10 | `handbook/design-and-decisions.md` | DD | Does this need a written plan or an ADR? | S8 | ISTQB §5.1.3 | 03 (design as communication, stale docs) | v2 |
| 11 | `handbook/release-and-maintain.md` | RM | Can I ship this, and how do I undo it? | S9, S16 | SSDF PS.2, PS.3; SLSA Build L1 to L2 | 05 (rollback, flags, regression, monitoring) | v2 |
| 12 | `handbook/maintaining-this-handbook.md` | MH | Should this page change? | This section | n/a | 05 (documentation debt) | v2 (the rule lives in the README for v1) |

Length targets: the quality bar is one page. Every other page is at most about 70 lines. A page that needs more is split.

## 3. Folder tree

```
SDLC-handbook/
├── README.md              # what this is; "pick a tool by task" routing; folder map; how to use a skill elsewhere; status; maintenance rule (v1)
├── handbook/              # REUSABLE GUIDANCE: short pages, one decision each, rules with stable IDs
│   ├── quality-bar.md
│   ├── workflow.md
│   ├── risk-tiers.md
│   ├── plan-tests.md
│   ├── verify-a-change.md
│   ├── working-with-agents.md
│   ├── standards-crosswalk.md
│   └── sources.md
├── skills/                # AGENT INSTRUCTIONS: one folder per skill, self-contained
│   └── test-plan/
│       ├── SKILL.md       # the instructions
│       └── reference.md   # pinned copy of every handbook rule the skill cites, by ID
├── templates/             # TEMPLATES: one screen each, with a filled synthetic example
│   ├── acceptance-criteria.md
│   ├── test-plan.md
│   ├── pr-description.md
│   └── adr.md
├── examples/              # WORKED EXAMPLES: a small synthetic Rust crate plus walkthroughs
├── eval/                  # evaluation kit: cases/, keys/ (separate), rubric.md, RUN.md, results/
├── build/                 # build state, deviations, build report
├── cse370-prepare/        # source material (unchanged)
└── research/              # the research record (unchanged)
```

**Project-specific decisions do not live here.** They live in each project repository:

| In the project repo | Holds |
|---|---|
| `docs/adr/NNNN-title.md` | That project's ADRs |
| The PR description (or `docs/changes/`) | That change's acceptance criteria, test plan, verification evidence |
| `.claude/skills/test-plan/` | A copy of the skill folder |
| `CLAUDE.md` | Short project facts: build and test commands, protected paths, where the real-environment checklist is |
| `.claude/settings.json`, `deny.toml`, CI config | The mechanical rules (hooks, dependency policy, checks) |
| `docs/real-environment-checks.md` | What CI cannot prove for that project, and how it is checked |

H3 is confirmed with one change. Skills load from a `skills/<name>/SKILL.md` folder and link to supporting files inside the same folder **[T3: S11]**. A link from a copied skill back to `handbook/` would break, so each skill carries its own `reference.md`.

## 4. Linking pattern

Links work through **stable rule IDs**, not file paths.

1. A skill output line ends with a rule ID:

   > Finding: the upper bound 10 is rejected. Evidence: `src/config.rs:14`, `value < 10`; spec says "0 to 10 inclusive". Proposed check: unit test for 0, 10, -1, 11 (unit layer). Why it matters: the documented maximum cannot be set. Handbook: **TP-3** (`reference.md#tp-3`)

2. `reference.md`, in the skill's own folder, holds that rule and its source:

   > <a id="tp-3"></a>**TP-3** Test each boundary and its neighbors. *Source: [Standard] ISTQB CTFL v4.0.1 §4.2.2. Full text: handbook/plan-tests.md#tp-3.*

3. The handbook page holds the full rule, and its citation points to `handbook/sources.md#s15`, which has the link, the edition and the tier.

**When the skill is copied into another repo**, the whole folder is copied, so `reference.md#tp-3` still resolves. `reference.md` starts with one header line that names the handbook version it was pinned from (for example "Pinned from SDLC-handbook v1.0, 2026-11-13"). The handbook is not published anywhere, so there is no URL to use. If it is published later, the base URL goes in that one header line.

**Guard against invented links.** The skill may cite only IDs that exist in its `reference.md`. The evaluation rubric counts any other ID as an invented reference (D5). This answers the H4 concern that agents fabricate links.

**Cost.** The rule text exists twice (handbook page and `reference.md`). The maintenance rule below keeps them in step. IDs are never renumbered or reused.

## 5. Maintenance rule

| Action | When | How |
|---|---|---|
| **Add** a rule or page | The developer hit the same decision at least twice and the handbook did not answer it | It needs a source at T1 to T3, or the label [Proposed] and a review date. One decision per page |
| **Revise** | A source changes edition; the evaluation or real use contradicts the rule; a T3 vendor doc changes | Change the handbook page and every `reference.md` that carries the ID **in the same commit**. Bump the handbook version in the README and the pinned line |
| **Retire** | A [Proposed] rule has not been used or confirmed after two quarterly reviews; a source is withdrawn | Mark it retired with the date. Keep the ID. Never reuse it |
| **Promote** from [Proposed] | A T1 source is found, or the developer's own records show it caught a real problem at least twice | Change the label and add the evidence to `sources.md` |
| **Review** | Once a quarter | Re-fetch the T3 vendor pages (they change) and update the "fetched" date; check each [Proposed] rule |

What earns a permanent place: a T1 source, or the developer's own recorded evidence (an evaluation pass, or a real issue the rule caught). Vendor guidance and this report's own synthesis stay provisional until one of those exists.

---

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

---

# D6. First build milestone (v1)

Supports **Decisions 1 and 3**. This file list is the scope of the build. Paths are relative to `SDLC-handbook/`.

## 1. Files to build

### Root (1)

| File | Contents |
|---|---|
| `README.md` | What the handbook is; "pick a tool by task" routing (bug fix, feature, high-risk change, review, release); folder map; how to use a skill in another repo; status; the maintenance rule from D4 §5 |

### Handbook pages (8)

Each page starts with the decision it supports, labels every claim, cites its sources with a link and a date or edition, and gives each rule a stable ID.

| File | Prefix | Built from |
|---|---|---|
| `handbook/quality-bar.md` | QB | D4 §1, verbatim in substance |
| `handbook/workflow.md` | WF | D2 §1 main table |
| `handbook/risk-tiers.md` | RT | D2 §2 tier table and escalation triggers |
| `handbook/plan-tests.md` | TP | D5 §1 rule table (TP-1 to TP-13) |
| `handbook/verify-a-change.md` | VC | D2 §4 verification order |
| `handbook/working-with-agents.md` | AG | D2 §3 delegation ladder and §5 enforcement map |
| `handbook/standards-crosswalk.md` | SC | D3 §4 (D3b): crosswalk, audit-evidence map, separation of duties, where standards and practice disagree |
| `handbook/sources.md` | SRC | D3 §1 source list and §2 study cards |

### Skill (2)

| File | Contents |
|---|---|
| `skills/test-plan/SKILL.md` | The D5 §1 spec in Agent Skills format |
| `skills/test-plan/reference.md` | Pinned copy of every rule the skill may cite (TP-1 to TP-13, RT rules, QB-2, QB-3, QB-8, QB-13), each with an anchor and its source |

### Templates (4)

Each is at most one screen, has a filled-in synthetic example, and links back to its handbook section.

| File | Used at |
|---|---|
| `templates/acceptance-criteria.md` | Stage 2, all tiers |
| `templates/test-plan.md` | Stage 4, all tiers (the skill's output format) |
| `templates/pr-description.md` | Stage 6, all tiers (what changed, how verified, what was not) |
| `templates/adr.md` | Stage 3, Tier 3 |

### Worked examples (a small crate plus 3 documents)

| File | Contents |
|---|---|
| `examples/README.md` | What the examples are and how to run them |
| `examples/handbook-examples/Cargo.toml` | A dependency-free library crate |
| `examples/handbook-examples/src/lib.rs` | Module declarations |
| `examples/handbook-examples/src/config.rs` | Case 1 code (`max_retries`), with its planted defects |
| `examples/handbook-examples/src/title.rs` | Case 2 code (`truncate_title`, `preview_snippet`), with its planted defect |
| `examples/case-1-max-retries.md` | Walkthrough labeled **[Illustrative]**: request, expected skill output, what to notice |
| `examples/case-2-truncate-title.md` | Same for case 2 |

`cargo check` must pass on the crate. The planted defects are real behavior bugs, documented in the walkthroughs, not compile errors.

### Evaluation kit (18 files plus a results folder)

| File | Contents |
|---|---|
| `eval/RUN.md` | How to run the evaluation: conditions, 3 runs per case, blind scoring, pass bar |
| `eval/rubric.md` | The 0 to 2 rubric on four criteria, plus false positives, invented references, review time |
| `eval/cases/case-01.md` to `case-08.md` | The prompt and any code for each case. No answer key, and no link to one |
| `eval/keys/key-01.md` to `key-08.md` | The answer keys, stored separately |
| `eval/results/` | Pilot results from the build, and later runs |

### Build records (3)

`build/00-build-state.md`, `build/DEVIATIONS.md`, `build/BUILD-REPORT.md`.

**Total: 43 files** the build writes: 1 root, 8 pages, 2 skill files, 4 templates, 7 example files, 18 evaluation-kit files and 3 build records. Pilot results are added to `eval/results/`.

## 2. What is left out of v1, on purpose

| Left out | Why |
|---|---|
| Pages `define-the-change`, `design-and-decisions`, `release-and-maintain`, `maintaining-this-handbook` | The first skill does not need them. The templates and the workflow table cover the minimum |
| The second skill, `review-change` | Build it only after the first skill passes its evaluation |
| Hook scripts, CI configuration, `deny.toml` | The research boundary and the build plan forbid installing or adding CI. The enforcement map says what to add; the developer adds it per project |
| Rollback-note template | Belongs with the release page in v2 |
| Anything marked [Large-org only]: fleet canaries, SLSA Build L3 and Source L4, management-system standards | Does not pay off for one developer |

## 3. What the reader can do with v1

1. Classify a change into a tier in under a minute, using mechanical triggers.
2. Run the `test-plan` skill before coding and get a plan in the form *finding, evidence, proposed check and layer, why it matters, handbook rule*.
3. Check a finished change against a one-page quality bar that says which layer enforces each rule.
4. See which checks prove what, and write down what was not verified.
5. Point to the standard clause behind a practice, and to the records the workflow already produces.
6. Rerun the evaluation to see whether the skill earns its place.

## 4. Results that justify expanding

Expand (write the v2 pages and build `review-change`) when **both** hold:

- **Evaluation:** the D5 pass bar is met on **two consecutive revisions** of the skill (full protocol: 8 cases, 3 runs, blind scoring).
- **Real use:** the skill has been used on **at least 5 real changes**, and it surfaced **at least one real issue** the developer agrees they would have missed, with the developer's review time for the plan staying under about 10 minutes for a Tier 2 change.

## 5. Results that mean rethink

| Result | What to do |
|---|---|
| The skill does not beat a plain request by 1.5 points | Do not expand. Check whether the plain model already plans tests well. If so, shrink the skill to its guardrails (integrity pass, stop-and-ask, "not verified") and build `review-change` first |
| Any invented reference in skill output | Fix before anything else: tighten the evidence rule, rerun |
| The skill gold-plates small fixes (case 5, criterion d below 1.5) | Shorten the Tier 1 path; rerun |
| The developer stops reading the plans (too long, too slow, or mostly obvious) | The format is wrong for daily use. Cut the output to findings and questions only |
| The skill is strong on cases 4 and 8 but weak on 1 to 3 and 5 to 7 | It is acting as a reviewer, not a planner. Reconsider the first-skill choice |
| Real issues keep escaping that a diff review would have caught | Raise the priority of `review-change` |

---

# D7. Run report

## 1. How the run went

- **Lead model.** The run started on Opus 5.5 and the developer switched it to Fable 5.2 during Phase 0. The Phase 0 reading happened on Opus. Every file was written, and every later phase run, on Fable 5.2.
- **Interruption.** All four workers stopped on the 5-hour usage limit seconds after launch (about 00:46 MST) and returned nothing. After the limit reset (03:00) each was resumed from its own transcript. Nothing saved was lost, which is what plan §12 is for.
- **Tools.** The lead had no direct page-fetch tool. Verification went through `web-fetch` agents given exact URLs and questions. Three standards came back as PDF bytes the fetch tool could not read; the lead extracted their text locally with `pdftotext` and read the passages directly.

## 2. What was delegated

| Worker | Model | Scope | Opened | What came back | Problems |
|---|---|---|---|---|---|
| W1 | Haiku 5.5 | Requirements and design artifacts, spec-driven tooling, change planning | 4 | Nygard's ADR sections; Gherkin's Given/When/Then; Spec Kit's command flow; Shape Up's "appetite" | No source for design docs vs ADRs, or for "done when" per step. The tier mapping was the worker's inference |
| W2 | Sonnet 5.5 | Testing, review, release, Rust tooling | 7 (4 + 3 tool pages) | Google review checklist and small-changes guidance; SRE release chapter; mutation-testing abstract; Clippy, cargo-deny, cargo-mutants | Test-layer and behavior-vs-implementation guidance not sourced; cargo-audit and Miri not opened; no official gate-order guidance |
| W3 | Sonnet 5.5 | AI studies, self-correction, package hallucination, agent guidance | 6 | METR, Spracklen, Huang, Olausson (abstract pages), Claude Code best practices, one PR-shape study | Papers read as abstracts only. No measured evidence on agents and tests. One search the brief asked for was not spent |
| W4 | Sonnet 5.5 | Standards (RQ6) | 6 | Editions and access for SSDF, SP 800-218A, SLSA, ASVS; a third-party summary of ISO/IEC 25010:2023 | **The weakest return.** No clause text read. SOC 2 behind a login; ISO pages returned HTTP 403; 13 standards not opened |
| Lead follow-up and verification | Fable 5.2 | Gaps that could change the first skill or the crosswalk; re-reading sources behind major recommendations | 5 new, plus deeper reads of 4 already opened | ImpossibleBench; *SWE at Google* ch. 12; skills and hooks docs; ISTQB syllabus; full text of METR and Spracklen; SSDF and SP 800-218A clause text; SLSA v1.2 levels | AICPA criteria still behind the login |

The workers were honest about what they did not read, which made triage straightforward. One worker correction mattered: W3 reported that Rust was absent from the package-hallucination paper. The full text shows Rust appears as a source of cross-language name confusion, with no rate of its own (C26).

## 3. Sources opened and cited

- **Opened: 28 distinct sources** (budget about 28): W1 4, W2 7, W3 6, W4 6, lead 5 new.
- **Cited as core: 18** (S1 to S18 in D3), within the 12 to 18 target. S18 groups three Rust tool pages.
- **Cited as supporting, with a caveat: 6** (read by a worker only).
- **Opened and not cited:** the ISO/IEC 25010 third-party summary (not an official source).

## 4. Claim-check table

The full table is [04-claim-check.md](../04-claim-check.md) and is included as an appendix in `REPORT.md`. Summary:

| Result | Count | Examples |
|---|---|---|
| yes | 29 | METR result (C17); test exploitation (C4, C5); ISTQB risk-based depth (C12); SSDF practices (C28, C41) |
| partly | 7 | Mutation-testing study, abstract only (C11); Clippy and cargo-deny docs, worker only (C29, C30); SRE "matched to risk" (C39) |
| no | 11 | Escalation triggers have no source (C16); the delegation ladder has no source (C21); same-model review untested (C34); SOC 2 CC8.1 not read (C42); ISO/IEEE clauses not read (C44) |

Every *no* row is either dropped from the report or presented with the label [Proposed] or "not read". Each of the five major recommendations rests on *yes* rows (table at the end of the claim-check file).

## 5. Remaining evidence gaps

| Gap | Affects | How much it could change the recommendations |
|---|---|---|
| Does the `test-plan` skill beat a plain request? | The first-skill choice | **High.** This is what the D5 evaluation measures. The build's pilot is a smoke test only |
| Does a same-model fresh-context review catch more than self-review? | H8; the value of the second skill | Medium |
| How often do agents weaken tests in everyday work (not on impossible tasks)? | The strength of the test-integrity rules | Low to medium: the rules are cheap either way |
| SOC 2 criteria text (CC8.1, CC7.x, separation of duties) | D3b audit-evidence map | Medium for RQ6. The map is built on SSDF and SLSA instead and says so |
| ISO/IEC/IEEE 29148, 25010, 12207, 42010, 29119; IEEE 1016; ISO/IEC 27001, 42001 | Crosswalk coverage; whether 25010 could structure the quality bar | Medium for RQ6, low for the workflow |
| Measured rates for agent-specific defects other than invented packages and test edits | Which quality-bar prompts are agent-specific | Low: they stay as review prompts labeled unmeasured |
| Outcome evidence for spec-first agent workflows | How much specification to require | Low |
| Security and delivery studies in the seed list (Perry et al., DORA, GitClear, Peng et al.) | Nothing in the report relies on them | Low |
| Official docs for cargo-audit, Miri and secret scanners | Three rows of the verification order and enforcement map, labeled [Proposed] or "not opened" | Low |
| Lightweight DDD, and LLM-generated adversarial tests | Two §3b claims have the verdict "not researched" | Low |

## 6. Inaccessible or unread

| Source | Why | How the report treats it |
|---|---|---|
| AICPA Trust Services Criteria PDF | Free, but behind an account login | SOC 2 criteria are not paraphrased anywhere |
| iso.org pages for ISO/IEC 25010 and others | HTTP 403; standards are paywalled | Listed as "not read"; no clause cited |
| Feathers, *Working Effectively with Legacy Code* | A book, not online | Named only as the origin of the term "characterization test" |
| "Anthropic AI-native playbook" | Not found; the one search allotted was not spent | Dropped |
| research.google page for the mutation-testing paper | HTTP 404 | The arXiv abstract was used instead |

## 7. Self-check against plan §11

| Check | Result |
|---|---|
| All three decisions can be made from D1 | Pass |
| Every [Observed] claim has a study card; no speed result presented as quality | Pass (D3 §2) |
| Every major recommendation is *yes* in the claim check | Pass, with the [Proposed] parts labeled |
| Every cited source has a link and a date or version; inaccessible ones flagged | Pass. Undated pages carry a fetch date |
| Labels sit next to the claims | Pass |
| Each hypothesis and §3b claim has a verdict; none repeated as fact | Pass. Two §3b claims have the verdict "not researched" |
| No speed claim without a study card | Pass |
| Every "must always" rule has a mechanical layer or an explicit human reason | Pass (D2 §5) |
| Enterprise-only practices marked and kept out of v1 | Pass |
| Every [Standard] claim names edition and clause; paywalled standards cited only through what was read | Pass. Unread standards have no clause cited |
| No standard presented as proof that a practice works | Pass |
| SOC 2 described accurately | Pass, as an attestation against the Trust Services Criteria. This description comes from the plan and the AICPA landing page; the criteria were not read |
| No private project or personal details | Pass (checked by search before the gate) |
| Demo outputs labeled illustrative; answer keys predate the demos | Pass. Keys were written at 00:30, before any worker ran |
| Sized for one junior developer | Pass |
| Plain language; each recommendation names its decision | Pass |

One plan target was not met in full: **D1 is over 300 words** once the 21 one-line verdicts are included. The recommendation itself (the three decisions and the tradeoffs) is about 250 words.

---

# Appendix. Claim-check table (Phase 3)

Written 2026-11-13 04:05 MST. Every row below was checked by the lead against the source itself (through a `web-fetch` agent with exact questions, or by reading extracted PDF text). Raw answers are in [workers/L-lead-verification-notes.md](workers/L-lead-verification-notes.md).

**How to read "Supports it?"**
- **yes**: the source says this, at the location given.
- **partly**: the source supports a narrower claim. The note says what the report may say instead.
- **no**: the source does not support it, or was not read. The claim is dropped or labeled as unverified.

Build rule (from the build plan): use only *yes* rows as stated; a *partly* row must carry its caveat; *no* rows stay out.

## A. Claims behind the major recommendations

| # | Claim in report | Source | Location | Supports it? | Note |
|---|---|---|---|---|---|
| **Handbook model** | | | | | |
| C1 | Short decision records stay current; large documents do not get kept up to date. An ADR is one or two pages with Title, Context, Decision, Status, Consequences | Nygard, "Documenting Architecture Decisions" (2011) | "Decision" section | yes | One practitioner's guidance, not a study |
| C2 | Agent instruction files should be short and hold only what the agent cannot infer; they are not guaranteed to be followed | Claude Code best practices (fetched 2026-11-13) | "Keep CLAUDE.md short"; "Deterministic and non-deterministic instructions" | yes | T3: describes how the product behaves |
| C3 | A skill's description is loaded at session start and its full file only when invoked; supporting files live in the skill directory and are linked relatively | Claude Code skills docs (fetched 2026-11-13) | "Choose where skills load"; "Frontmatter reference"; "Add supporting files" | yes | Frontmatter and loading details came back as a tool summary, not verbatim. Re-check at build step B5 |
| **First skill (test planning)** | | | | | |
| C4 | Given tests that conflict with the spec, current models often make the tests pass anyway, including by editing the tests or special-casing them | Zhong, Raghunathan, Carlini, "ImpossibleBench" (arXiv 2510.20270v1, Oct 2025) | §1, §4, §4.1, §4.2 | yes | Benchmark tasks built to be impossible; rates vary widely by model and prompt (GPT-5: 76% on one variant, 2.9% on another) |
| C5 | A strict prompt, hiding the tests, and letting the agent flag a conflict for a human each reduce test exploitation; read-only tests alone do not stop special-casing | ImpossibleBench | §5.1, §5.2, §5.3 | yes | Effect sizes are model-specific |
| C6 | A reviewer should ask whether the tests would actually fail if the code were broken; tests do not validate themselves and a human must check them | Google eng-practices, "What to look for in a code review" | "Tests" | yes | |
| C7 | Test behaviors through the public API; a test that must change when the implementation is refactored signals a problem | *Software Engineering at Google*, ch. 12 | "Test via Public APIs"; "Test Behaviors, Not Methods"; "Strive for Unchanging Tests" | yes | About 40k of 59k characters were returned verbatim |
| C8 | Acceptance criteria can be treated as the test conditions and should cover negative as well as positive scenarios | ISTQB CTFL syllabus v4.0.1 | §4.5.2 | yes | Consensus syllabus, not outcome evidence |
| C9 | Equivalence partitioning and boundary value analysis are standard black-box test techniques | ISTQB CTFL v4.0.1 | §4.2.1, §4.2.2 | yes | Section titles and body confirmed present; technique detail matches the CSE 370 Unit 04 notes |
| C10 | Model self-repair of code gives modest gains; the quality of feedback is the limit, and feedback from a stronger model helps much more | Olausson et al. (ICLR 2024; arXiv 2306.09896v5) | Abstract | yes | [Older-generation tools]. Abstract only |
| C11 | Mutation testing leads developers to write more tests, and mutants are coupled with real faults | Petrovic et al. (ICSE 2021; arXiv 2103.07189) | Abstract | partly | Abstract only, read by W2, not re-opened by the lead. Report may say "a Google study reports", with no effect size |
| **Tiers** | | | | | |
| C12 | Product risk (likelihood and impact) should determine test scope, levels, techniques, coverage and effort | ISTQB CTFL v4.0.1 | §5.2.3 | yes | Supports scaling by risk, not by change size |
| C13 | Secure-development practices should be applied with a risk-based approach and customized; SSDF is not a checklist and applies at any organization size | NIST SP 800-218 v1.1 | §1, §2 | yes | |
| C14 | A methodology may change the timing or style of a phase, never whether it happens | CSE 370 Unit 07 notes (local) | "Methodology vs SDLC" | yes | Course notes, not an external source |
| C15 | Exit criteria ("Definition of Done") define what must be achieved to call an activity complete | ISTQB CTFL v4.0.1 | §5.1.3 | yes | Backs the "done when" pattern |
| C16 | Specific escalation triggers (auth, migrations, money, public API, `unsafe`, new dependencies) | none | n/a | no | No source lists these. They are **[Proposed]** and must be labeled so |
| **Delegation** | | | | | |
| C17 | Experienced developers were 19% slower with AI tools on their own repos while believing they were about 20% faster | METR (arXiv 2507.09089v2, Jul 2025) | Abstract; §3; §3.1 | yes | Speed only. 16 developers. Authors caution against generalizing; they say unfamiliar codebases may differ (untested) |
| C18 | Rules with no exceptions belong in a hook, a deny rule or a managed setting, because instruction files are not deterministic | Claude Code best practices | "Enforce the rules that can't have exceptions" | yes | T3 |
| C19 | A `PreToolUse` hook can block a tool call (exit code 2), and the docs show blocking edits to protected files | Claude Code hooks guide | "How hooks work"; "Block edits to protected files" | yes | T3 |
| C20 | Give the agent a pass/fail check it can run, and ask for evidence | Claude Code best practices | "Give Claude a check to run" | yes | T3 |
| C21 | A delegation ladder (levels of autonomy earned by evidence) | none | n/a | no | No source defines one. **[Proposed]**, built from C4, C5, C17 to C20 |
| **Quality bar** | | | | | |
| C22 | A reviewer looks at design, functionality, complexity, tests, naming, comments, style, consistency, documentation, every line, and context | Google eng-practices, "What to look for" | Section headings | yes | |
| C23 | Over-engineering is a named review concern; solve the problem known to exist now | Google eng-practices | "Complexity" | yes | |
| C24 | Keep changes small and single-purpose; put refactors in a separate change; include tests in the same change | Google eng-practices, "Small CLs" | "Why Write Small CLs?"; "What is Small?"; "Separate Out Refactorings" | yes | Line counts are rules of thumb |
| C25 | Code-generating models invent package names at measurable rates (at least 5.2% commercial, 21.7% open-source models), and 43% of invented names recur on every repeat of the same prompt | Spracklen et al. (USENIX Security 2025; arXiv 2406.10279v3) | Abstract; §5.1; §5.3 | yes | [Older-generation tools]. **Python and JavaScript only.** No rate for Rust |
| C26 | Package hallucination is measured for Rust/crates.io | Spracklen et al. | §5.4, Table 3 | no | Rust appears only as a source of cross-language name confusion. The report must say the Rust rate is unmeasured |
| C27 | All source code, human-written or AI-generated, should be evaluated for vulnerabilities and other issues before use | NIST SP 800-218A | Scope | yes | The profile targets AI model and system producers, not users of coding tools. This one sentence is the usable part |
| C28 | Review and/or analyze code (PW.7), test executable code (PW.8), and verify third-party components (PW.4.4) are named practices | NIST SP 800-218 v1.1 | Table, PW.4, PW.7, PW.8 | yes | |
| C29 | Clippy's `restriction` group should not be enabled wholesale; pick individual lints such as `unwrap_used` | Clippy docs | "Lint groups" | partly | Read by W2 only, not re-opened by the lead. T3. Acceptable for how the tool works |
| C30 | `cargo deny check` covers advisories, bans, licenses and sources | cargo-deny docs | "Checks" | partly | Read by W2 only. T3 |
| C31 | Agent-written code more often has duplicated code, mirrored tests, scope creep, or wrong comments | none opened | n/a | no | **Anecdote in this run.** No measured source was opened. Label as unmeasured |

## B. Other claims used in the report

| # | Claim in report | Source | Location | Supports it? | Note |
|---|---|---|---|---|---|
| C32 | Some independence makes testing more effective; levels run from the author to outside the organization; it has drawbacks | ISTQB CTFL v4.0.1 | §1.5.3 | yes | About human testers. Says nothing about AI reviewers |
| C33 | A reviewer in a fresh context that has not seen how the code was written is a recommended setup | Claude Code best practices | "Give Claude a check to run"; "Know what subagents cost" | yes | Vendor guidance (T3), not a measured result |
| C34 | A fresh-context reviewer using the *same* model catches more than the author reviewing itself | none | n/a | no | Not tested by any source read. H8 can only be "modified" |
| C35 | Given/When/Then: context, action, observable outcome; 3 to 5 steps per example | Cucumber Gherkin reference | Keyword sections | partly | Read by W1 (Haiku) only, not re-opened by the lead |
| C36 | Spec-driven tooling prescribes constitution, specify, plan, tasks, implement, converge | GitHub Spec Kit README | README | partly | Read by W1 only. Method description, no outcome evidence |
| C37 | Spec-first agent workflows have measured benefit | none | n/a | no | No outcome study opened. Label as unmeasured |
| C38 | Frequent releases mean fewer changes per release and easier troubleshooting; release process matters at any company size | Google SRE book, ch. 8 | "High Velocity"; "It's Not Just for Googlers" | yes | |
| C39 | Rollout should be matched to risk; canary criteria | Google SRE book, ch. 8 | n/a | partly | W2 reported "matched to risk". The lead's check confirmed only a brief canary mention. Report may cite the canary mention, not criteria |
| C40 | SLSA v1.2 has a Build track (L0 to L3) and a Source track (L1 to L4); Source L4 requires two-party review | slsa.dev v1.2 | "Tracks"; build-track-basics; source-requirements | yes | |
| C41 | SSDF PO.3.3 asks for tools to generate artifacts (an audit trail); PW.7.2 and PW.8.2 ask for issues and results to be recorded; PS.3.1 asks for releases to be archived | NIST SP 800-218 v1.1 | Table | yes | Basis of the audit-evidence map |
| C42 | SOC 2 CC8.1 requires changes to be authorized, tested, approved and documented; separation of duties is expected | AICPA Trust Services Criteria | n/a | **no** | **Criteria text not read (login wall).** The report must not paraphrase CC8.1. The audit-evidence map is built on SSDF and SLSA text and marks SOC 2 as unverified |
| C43 | OWASP ASVS current version is 5.0.0 (May 2025), free | asvs.owasp.org | Landing page | partly | Read by W4 only. Edition and license only; no requirement text read |
| C44 | ISO/IEC 25010:2023 characteristics; ISO/IEC/IEEE 29148 requirement characteristics; 12207, 42010, 29119, IEEE 1016, ISO/IEC 27001, 42001 clauses | none | n/a | **no** | Paywalled or blocked (HTTP 403). No clause may be cited. Listed in D3b as "not read" |
| C45 | Professional criticism of ISO/IEC/IEEE 29119 | none | n/a | no | Not researched. Mentioned only as an unverified lead |
| C46 | Characterization tests pin existing behavior before changing legacy code | Feathers, *Working Effectively with Legacy Code* (2004) | n/a | no | Book not opened. May be named as the origin of the term, marked "not opened in this run" |
| C47 | An "Anthropic AI-native playbook" exists | none | n/a | no | Not found. Dropped |

## Result for the gate

All five major recommendations rest on at least one *yes* row, and the parts that have no source are labeled [Proposed] rather than presented as sourced:

| Major recommendation | *yes* rows it rests on | Parts that are [Proposed] |
|---|---|---|
| Handbook model (short guide plus skills) | C1, C2, C3 | The page count and layout |
| First skill (test planning) | C4, C5, C6, C7, C8, C9, C10 | The weighted scores |
| Tiers (depth scales with risk, no phase skipped) | C12, C13, C14, C15 | The three tier names and the escalation triggers (C16) |
| Delegation (earned by evidence; hard rules in hooks) | C4, C5, C17, C18, C19, C20 | The ladder itself (C21) |
| Quality bar | C6, C22, C23, C24, C25, C27, C28 | Which lints to pick; the agent-defect list beyond C4 and C25 (C31) |
