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
