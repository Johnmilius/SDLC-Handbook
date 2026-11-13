# Build report: handbook v1 and the first skill

Built 2026-11-13 from the research run in `research/fable-run/`. All 43 files in the D6 scope exist. Everything is committed on the local branch `handbook-v1`. Nothing was installed, merged or pushed.

**The one result to read first:** in the pilot, the `test-plan` skill scored 8.0 out of 8 and a plain one-line request scored 7.0. That is a gain of 1.0, under the 1.5 the skill needs to pass. The pilot is three cases run once each, so it settles nothing, but it does not yet show that the skill earns its place. See "Pilot results" and "Needs developer decision".

## What was built

```
README.md                         What the handbook is, routing by task, how to use a skill elsewhere, maintenance rule
handbook/
  quality-bar.md                  QB-1 to QB-13: the single reference for "good enough to merge"
  workflow.md                     WF-1 to WF-8: one change through eight stages
  risk-tiers.md                   RT-1 to RT-4: three tiers and the escalation triggers
  plan-tests.md                   TP-1 to TP-13: the rules the skill applies
  verify-a-change.md              VC-1 to VC-9: the verification order and what each check proves
  working-with-agents.md          AG-1 to AG-8: delegation ladder and enforcement map
  standards-crosswalk.md          SC-1 to SC-3: clauses that were read, audit-evidence map, separation of duties
  sources.md                      S1 to S18 with links, study cards, and what was not read
skills/test-plan/
  SKILL.md                        The first skill
  reference.md                    Pinned copy of every rule the skill may cite
templates/
  acceptance-criteria.md          Given/When/Then, with a filled example
  test-plan.md                    The skill's output format, with a filled Tier 1 example
  pr-description.md               How the change was verified and what was not
  adr.md                          One decision per record, with a filled example
examples/
  README.md                       What the examples are
  handbook-examples/              A dependency-free Rust crate (Cargo.toml, src/lib.rs, src/config.rs, src/title.rs)
  case-1-max-retries.md           Walkthrough of case 1, labeled [Illustrative]
  case-2-truncate-title.md        Walkthrough of case 2, labeled [Illustrative]
eval/
  RUN.md                          How to rerun the evaluation from scratch
  rubric.md                       The 0 to 2 rubric and score sheet
  cases/case-01.md to case-08.md  Case material, no answer keys
  keys/key-01.md to key-08.md     Answer keys, kept separate
  results/pilot-2026-11-13.md     Pilot summary, with raw outputs and scorer notes in a folder beside it
build/
  00-build-state.md               Step-by-step state
  DEVIATIONS.md                   Four logged deviations
  BUILD-REPORT.md                 This file
```

Six of the eight handbook pages were drafted by Sonnet 5.5 workers from the research files and then reviewed and corrected by the lead. The lead wrote the quality bar, `plan-tests.md`, the skill, the templates, the examples and the evaluation kit.

## Review first

| File | Why it needs your judgment |
|---|---|
| [handbook/risk-tiers.md](../handbook/risk-tiers.md) | The tier names and every escalation trigger are [Proposed]. No source lists them. You will apply them on every change, so they should match how your projects actually fail |
| [skills/test-plan/SKILL.md](../skills/test-plan/SKILL.md) | The pilot shows it runs long on small fixes and asks a lot of questions. Decide whether that is what you want to read every day |
| [handbook/quality-bar.md](../handbook/quality-bar.md) | Everything else points here. QB-5, QB-11 and part of QB-7 are [Proposed] |
| [handbook/working-with-agents.md](../handbook/working-with-agents.md) | The delegation ladder and its counts (5 and 10) are starting values with no source |
| [handbook/standards-crosswalk.md](../handbook/standards-crosswalk.md) | Coverage is narrower than you asked for. Four standards were read. SOC 2 and the ISO/IEEE standards were not, and the page says so |

## Pilot results

Three cases, one run per condition, Sonnet 5.5 as the runner, blind scoring by a separate subagent, then a hand check by the lead. Full detail: [eval/results/pilot-2026-11-13.md](../eval/results/pilot-2026-11-13.md).

| Case | Skill | Plain | Difference |
|---|---|---|---|
| 01 New feature | 8 | 8 | 0 |
| 02 Bug fix in existing code | 8 | 6 | +2 |
| 04 Agent-written diff | 8 | 7 | +1 |
| **Average (scorer)** | **8.0** | **7.0** | **+1.0** |
| Average (lead's hand scores) | 7.3 | 6.3 | +1.0 |

- **The skill did not beat the plain request by the 1.5 points its own pass bar requires.**
- On cases 01 and 04, the plain request found every planted defect the skill found. The model already derives tests from a spec and reviews a diff well without help.
- On case 02 the skill did what it was built for: it stopped to ask what the length limit counts and whether the second function was in scope. The plain run decided both silently.
- The skill had no invented references. One plain output had one.
- The skill ran long: 57 lines for a Tier 1 fix against its own target of about 15, and eight blocking questions on case 04. The scorer did not penalize this. The lead did.

Likely reasons the gap is small: the cases are small and self-contained, so a capable model sees the defects either way; the rubric rewards catching defects more than it rewards asking first; and one run per cell is too few to see a one-point effect.

## Deviations and decisions

Four deviations are logged in [DEVIATIONS.md](DEVIATIONS.md):

1. `research/gemini-prompts.md` was left untracked and unopened.
2. The baseline commit is on `main`; the build is on the branch `handbook-v1`, unmerged.
3. Vendor-documentation claims carry the marker `[T3 docs]` instead of one of the four labels.
4. One extra file, a `.gitignore` in the example crate.

Two smaller choices are not deviations but are worth knowing: the pilot's raw outputs are stored as files in a folder next to the pilot summary, not inside it; and the skill was given to the pilot runners by having them read its two files, not through the real skill loader.

### Needs developer decision

1. **Whether to keep `test-plan` as the first skill.** The research ranking was close (4.3 against 4.1 for a review skill), and the pilot leans toward the "rethink" row in the research plan: a plain request already plans tests well. Do not decide on the pilot. Run the full protocol in [eval/RUN.md](../eval/RUN.md) first.
2. **Whether to shorten the skill before that run.** The Tier 1 path is too long. A revision would cap Tier 1 output and limit blocking questions to the ones that change an expected value. It was left as specified, because changing the skill after seeing pilot results and before the real evaluation is your call.
3. **Whether to merge `handbook-v1` into `main`.**

## TODO(source) and known gaps

No page contains a `TODO(source)` marker. Every unsourced rule is labeled [Proposed] instead. The gaps:

| Gap | Where it shows |
|---|---|
| SOC 2 criteria text was not read (login wall) | Crosswalk has no SOC 2 clauses; the audit-evidence map is built on SSDF and SLSA |
| ISO/IEC/IEEE 29148, 25010, 12207, 42010, 29119, IEEE 1016, ISO/IEC 27001 and 42001 were not read (paywalled) | Listed as "not read" with no clause |
| No source opened for property testing, characterization tests, or fail-first bug reproduction | TP-4, TP-5, TP-13 are [Proposed] |
| Miri, cargo-audit and secret-scanner documentation not opened | QB-11 and two enforcement-map rows are [Proposed] |
| Clippy, cargo-deny, cargo-mutants and the mutation-testing study were read by a research worker only | Cited with that caveat (VC-2, VC-5, VC-6, TP-8) |
| No measured evidence for most agent-specific defects | The quality bar lists them as review prompts, not rates |
| Whether a same-model reviewer in a fresh context beats self-review is untested | AG-6 and VC-7 call it a filter, not a control |
| Four handbook pages are deferred to v2 | `define-the-change`, `design-and-decisions`, `release-and-maintain`, `maintaining-this-handbook` |
| No hook scripts or CI configuration | The enforcement map says what to add per project |

## How to try v1 on a real change

1. Pick a small, real change in one of your projects.
2. Copy `skills/test-plan/` into that project at `.claude/skills/test-plan/`. Read both files first.
3. Open [risk-tiers.md](../handbook/risk-tiers.md) and decide the tier yourself before asking the agent.
4. Write the acceptance criteria with the [template](../templates/acceptance-criteria.md).
5. In a fresh session, run `/test-plan` with the criteria and the files involved.
6. Read the plan. Answer its blocking questions. Note how long that took and whether it told you anything you did not already know.
7. Implement, then check the change against the [quality bar](../handbook/quality-bar.md) and fill in the [PR description](../templates/pr-description.md).
8. Write one line in a log: date, change, tier, what the plan caught, minutes spent reading it.

## Next

**Expand only when both hold** (from D6):

- the full evaluation meets the pass bar on two consecutive revisions of the skill;
- the skill has been used on at least 5 real changes and surfaced at least one issue you agree you would have missed, with plan review staying under about 10 minutes for a Tier 2 change.

**If they hold**, write the four v2 pages and build the second skill, `review-change`: one skill that merges independent review and PR verification, run on a finished diff against the quality bar.

**If the full evaluation confirms the pilot** (gain under 1.5), shrink `test-plan` to the behaviors that did help (stop and ask, order the work, say what was not verified) and build `review-change` first.

## Final self-check (build plan section 8)

| Check | Result |
|---|---|
| Every D6 file exists | Pass: 43 of 43 |
| All relative links and anchors resolve | Pass: 281 links in 40 files, 0 broken (checked by script) |
| Every rule ID that is used is defined, and the skill cites only IDs in its `reference.md` | Pass |
| Claims carry a label and citation; no *no* claim used; *partly* claims keep their caveat | Pass on review. No speed claim, no SOC 2 criterion text, no ISO/IEEE clause |
| Standards cited by edition and clause, paraphrased | Pass |
| The quality bar is the single source of truth | Pass. `reference.md` carries a pinned copy of four of its rules, as the blueprint specifies |
| No private project or personal details | Pass: searched for the project name and the developer's name and email |
| Example code compiles; planted defects are deliberate and documented | Pass: `cargo check`, `cargo clippy` and `cargo test` are clean, and each defect was confirmed with a throwaway test |
| Answer keys separate; pilot runners had no access | Pass, with the limit stated in the pilot file: isolation was by a separate folder and instruction, not a sandbox |
| Nothing installed, nothing pushed; `research/` and `cse370-prepare/` unchanged | Pass: no git remote exists, and the diff against the baseline is empty for both folders |
| State file shows the build complete | Pass |
