# Quality bar

**Decision this page supports:** is this change good enough to merge?

"The tests pass" is the entry ticket, not the bar. A change meets the bar when every rule below holds, or the exception is written in the PR. This page is the single reference for the bar. Other pages, the skills and CI point here and do not restate it.

| ID | Rule | Enforced by | Source |
|---|---|---|---|
| <a id="qb-1"></a>**QB-1** | It does what the acceptance criteria say, including the negative cases | Skill checklist, human | [Standard] ISTQB CTFL v4.0.1 (2024) §4.5.2 ([S15](sources.md#s15)) |
| <a id="qb-2"></a>**QB-2** | Its tests would fail if the code were broken. They check behavior through the public API, not the implementation's current output | Skill checklist, human. Mutation check in CI for Tier 3 | [Established] Google eng-practices, "Tests" ([S5](sources.md#s5)); *Software Engineering at Google* ch. 12 ([S7](sources.md#s7)) |
| <a id="qb-3"></a>**QB-3** | No test was deleted, ignored or given a new expected value without the developer's explicit approval | Hook, CI flag, human | [Observed] ImpossibleBench, arXiv 2510.20270v1 (Oct 2025): models edit or special-case tests to pass ([S3](sources.md#s3)) |
| <a id="qb-4"></a>**QB-4** | Unhappy paths are covered: invalid, empty, boundary and failing inputs | Skill checklist, human | [Standard] ISTQB CTFL v4.0.1 §4.2.1, §4.2.2 ([S15](sources.md#s15)). [Established] eng-practices, "Functionality" ([S5](sources.md#s5)) |
| <a id="qb-5"></a>**QB-5** | Errors are handled, not swallowed. No `unwrap` or `expect` on input or I/O in production paths | CI lint. A human approves any `#[allow]` | [Proposed]. Lint mechanics: Clippy docs, fetched 2026-11-13, read by a research worker only ([S18](sources.md#s18)) |
| <a id="qb-6"></a>**QB-6** | It is the simplest design that works. No speculative features | Human, with AI review as a prompt | [Established] eng-practices, "Complexity" ([S5](sources.md#s5)) |
| <a id="qb-7"></a>**QB-7** | One concern per change. Refactors are separate. Tests are in the same change. No dead code, leftover debugging or unrelated edits | Skill checklist, human. Optional CI size warning | [Established] eng-practices, "Small CLs" ([S6](sources.md#s6)). The dead-code item is [Proposed] |
| <a id="qb-8"></a>**QB-8** | Every new or upgraded dependency exists, is the one intended, is justified in the PR, and passes policy | Hook, CI (`cargo deny`), human | [Observed] Spracklen et al., USENIX Security 2025: invented package names, measured for Python and JavaScript, unmeasured for Rust ([S2](sources.md#s2)). [Standard] NIST SP 800-218 v1.1 (2022) PW.4.4 ([S13](sources.md#s13)) |
| <a id="qb-9"></a>**QB-9** | It is formatted, compiles, is lint-clean, and the tests pass | CI | [Standard] NIST SP 800-218 v1.1 (2022) PW.7, PW.8 ([S13](sources.md#s13)) |
| <a id="qb-10"></a>**QB-10** | Names, comments and docs say what the code actually does. Comments explain why | AI review, human | [Established] eng-practices, "Naming", "Comments", "Documentation" ([S5](sources.md#s5)) |
| <a id="qb-11"></a>**QB-11** | Any `unsafe` is minimal, justified in a comment, and checked | Human. Miri in CI where it applies | [Proposed]. The Miri documentation was not opened in the research run |
| <a id="qb-12"></a>**QB-12** | Agent-written code meets the same bar as hand-written code. "An agent wrote it" and "CI is green" are not reasons to approve | Human | [Standard] NIST SP 800-218A (2024), Scope ([S14](sources.md#s14)). [Established] eng-practices, "Tests" ([S5](sources.md#s5)) |
| <a id="qb-13"></a>**QB-13** | What was not verified is written down | Skill output, PR description | [Standard] ISTQB CTFL v4.0.1 §1.3, principle 1 ([S15](sources.md#s15)) |

Rules that must always hold have a mechanical layer: QB-3, QB-5, QB-8 and QB-9. The rest need judgment, so a person owns them. [Working with agents](working-with-agents.md) explains why each rule sits at its layer.

## Defects to look for in agent-written code

| Defect | Evidence status |
|---|---|
| Invented or near-miss package names | **Measured** for Python and JavaScript, not for Rust ([S2](sources.md#s2)) |
| Tests edited, special-cased or weakened so that they pass | **Measured** on benchmark tasks built to be impossible. The everyday rate is unknown ([S3](sources.md#s3)) |
| Tests that restate the implementation; happy-path-only logic; duplicated code; scope creep; plausible but wrong API use; comments that do not match the code | **Not measured in the research run.** Use these as review prompts, not as known rates |

**CSE 370:** [Unit 04](../cse370-prepare/unit04-testing.md) covers code review, static analysis and why coverage can be gamed. It also says the developer owns the correctness of their own code.

## Related

[Verify a change](verify-a-change.md) · [Plan tests](plan-tests.md) · [Risk tiers](risk-tiers.md) · [Working with agents](working-with-agents.md) · [Standards crosswalk](standards-crosswalk.md) · [Sources](sources.md)
