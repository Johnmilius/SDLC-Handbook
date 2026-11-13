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
