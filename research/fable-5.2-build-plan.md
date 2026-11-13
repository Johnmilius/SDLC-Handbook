# Fable 5.2 Build Plan: Handbook v1 and First Skill

**Version:** 1.0 (2026-11-13). This plan pairs with `research/fable-5.2-research-plan.md` v1.3.

**How it starts:**
- **Automatically**, in the same session, right after the research run passes its gate (research plan §13).
- **Manually:** open a new Fable 5.2 session in `C:\Users\jwmil\code\active\SDLC-handbook` and send *"Read `research/fable-5.2-build-plan.md` and execute it."*
- **After a usage limit:** send the same message in a new session. It resumes from `build/00-build-state.md`.

---

## 0. Role and goal

You are the **lead builder**. Turn the research blueprint into a **working v1**: the handbook pages, the first skill, worked examples, and an evaluation kit. Then run a small pilot evaluation.

You are **implementing a design that has already been decided**, not reopening it. Use the saved research files as the specification. When the blueprint is unclear or wrong, make the smallest reasonable decision and log it (§5). Don't redesign.

**Quality standard:** this handbook teaches production-quality engineering, so its own files must meet the bar they describe:
- short, accurate and cited
- every rule enforceable or explained
- no filler
- nothing invented

---

## 1. Inputs (read in this order)

1. `research/fable-run/00-run-state.md`: confirm `Current phase: BUILD`, or a gate PASS.
2. `research/fable-run/HANDOFF.md`
3. `research/fable-run/deliverables/D6-first-milestone.md`: **the file list is the build scope.**
4. `D4-handbook-blueprint.md`: table of contents, folder tree, linking pattern, quality-bar page.
5. `D5-first-skill.md`: skill spec, demos, evaluation protocol.
6. `D2-lifecycle-workflow.md` and `D3-source-comparison.md` (including the D3b standards crosswalk): content and citations for the pages.
7. `04-claim-check.md`: **use only claims marked *yes***. Anything marked *partly* has to carry that caveat. Claims marked *no* stay out.
8. `02-synthetic-cases.md`: the cases and answer keys.
9. The relevant `cse370-prepare/` unit notes, for the CSE 370 cross-references.

**If an input is missing**, or the gate didn't pass:
- Write `build/BUILD-BLOCKED.md` saying what's missing.
- Stop.

---

## 2. Boundaries

| Rule | Detail |
|---|---|
| **Where you may write** | Only inside `SDLC-handbook/`, in the folders the D4 blueprint defines (e.g., `README.md`, `handbook/`, `skills/`, `examples/`, `templates/`, `eval/`), plus `build/` for state and reports. |
| **Read-only** | `cse370-prepare/`, everything in `research/` (including `fable-run/`), and the Eloksen repo. |
| **Don't touch** | `research/gemini-prompts.md`. Don't open, edit or run it. |
| **No installs** | Don't copy skills into `~/.claude/skills` or any other repo. Don't install tools, add CI to real projects, or push anywhere. The developer installs things after reviewing. |
| **Privacy** | No Eloksen details or personal information anywhere in the handbook. Examples stay synthetic. |
| **Copyright** | Paraphrase standards and sources, and cite the clause or section. Never copy a standard's text, especially a paywalled one. Quotes stay under 15 words. |
| **Git (local only)** | If `SDLC-handbook` isn't a git repo, run `git init`. Then make a **baseline commit** of the existing files (`cse370-prepare/`, `research/`) before writing anything. Commit after every build step (§4). **No remote, no push.** End commit messages with:<br>`Co-Authored-By: Claude Fable 5.2 <noreply@anthropic.com>` |

---

## 3. Checkpoints and saving

Assume the session can stop at any moment. Each step ends with a **commit plus a state update**, so a cap or crash loses at most one step.

**`build/00-build-state.md`** (update after every step):
```
# Build state
Build plan version: 1.0
Last updated: <date time>
Current step: <B0–B8>
Done: <checklist with file links and commit hashes>
Next step: <exact next action, specific enough that a fresh session can do it cold>
Deviations logged: <count> (see build/DEVIATIONS.md)
Open issues: <anything a resuming session must know>
```

**Resuming:**
1. If `build/00-build-state.md` exists, run `git status` and `git log --oneline -5`.
2. If there are uncommitted changes from an interrupted step, **check them against that step's done-when**. Finish them or discard them, then continue from "Next step".
3. Never redo committed steps.

---

## 4. Build steps

Each step has a **done-when** check. Don't move on until it holds. Then commit and update the state file.

| # | Step | Done when |
|---|---|---|
| **B0** | **Set up.** Read the inputs (§1). Confirm the gate passed. `git init` if needed, then the baseline commit. Create `build/00-build-state.md` and `build/DEVIATIONS.md`. Turn the D6 file list into a checklist in the state file. | The baseline commit exists. The state file lists every D6 file. |
| **B1** | **Scaffold.** Create the D4 folder tree. Write the root `README.md`: what the handbook is, **"pick a tool by task"** routing, the folder map, how to use a skill in another repo, and current status. | Every D4 folder exists. The README routes each common task (bug fix, feature, high-risk change, review, release) to a page or skill. |
| **B2** | **Quality-bar page first.** Write the RQ5 quality-bar page from D4. Everything else links to it. | ≤1 page. Each item names its enforcement layer (hook / CI / skill / human). Each item is cited. |
| **B3** | **Handbook pages.** Write the remaining v1 pages from D6, following the D4 table of contents. One page per sitting, committed separately. | For each page: it answers a decision (stated at the top). Every claim is labeled [Standard] / [Established] / [Observed] / [Proposed] and cited **with a link plus a date or edition**. It includes the CSE 370 cross-reference where D4 checked one. It uses stable section anchors for skills to link to. It's within the D4 length target. |
| **B4** | **Templates.** Write the minimum artifacts D2 names for each tier (e.g., problem statement, acceptance criteria, ADR, test plan, PR description, rollback note), but only those D6 lists. | Each template is ≤1 screen, has a filled-in synthetic example, and links back to its handbook section. |
| **B5** | **First skill.** Write `skills/<name>/SKILL.md` to the D5 spec. **Check the current Claude Code Agent Skills format** (frontmatter fields, how the description triggers it, where supporting files go) against the official docs before writing. Read the docs only. Don't install anything. | The frontmatter is valid. The description says when to use it *and when not to*. Each step matches D5. The output format is *finding · evidence · proposed check + layer · why it matters · handbook link*. Stop-and-ask points are explicit. The guardrails from RQ2 are included. The handbook links follow the D4 linking pattern and work when the skill is copied into another repo. |
| **B6** | **Worked examples.** Put the two D5 demo cases in `examples/` as runnable synthetic Rust snippets (a tiny `cargo` crate or self-contained files). Each comes with an expected-output walkthrough labeled **[Illustrative]**. | If the snippets are a crate, `cargo check` passes, so the examples compile and the planted bugs are real bugs, not typos. Each example links to the skill and the handbook sections it exercises. |
| **B7** | **Eval kit.** Build `eval/` from the D5 protocol: `cases/` (prompts plus any code), `keys/` (answer keys, **stored separately from the cases**), `rubric.md` (0–2 on: catches the risk / grounded / clear reasoning / proportional; plus false positives, invented references, review time), and `RUN.md` (how to run it: skill vs plain request, same model, 3 runs per case, blind scoring). Include at least one agent-written-diff case (research case 4 style). | Someone could rerun the eval from `RUN.md` alone. Keys aren't referenced from any case file. |
| **B8** | **Pilot eval and build report.** See §6. | The pilot results and `build/BUILD-REPORT.md` are committed. |

**Splitting work:** pages, templates and examples that don't depend on each other may go to worker agents (Sonnet 5.5) running in parallel. Each worker gets the D4 page spec, the relevant deliverable excerpts and the claim-check rows, and **writes only its own file**. The lead reviews every worker file against the step's done-when before committing it. The lead writes the quality-bar page (B2) and the skill (B5) itself.

---

## 5. When the blueprint is wrong or unclear

- Make the **smallest decision** that keeps the build moving and fits the research.
- Log it in `build/DEVIATIONS.md`: *what the blueprint said · what you did · why · whether the developer should review it*.
- If a gap would change **the first skill** or **the handbook's structure**:
  - Don't improvise.
  - Write it under "Needs developer decision" in the build report.
  - Build everything that doesn't depend on that decision.
- Never add content that isn't backed by the research files. If a page clearly needs a source that research didn't cover, mark it `TODO(source)` and list it in the report.

---

## 6. Pilot evaluation (B8)

This is a small smoke test to show the skill does something useful. **It isn't the real evaluation.**

1. **Cases:** demo case 1 (new feature), demo case 2 (change to existing code), and the agent-written-diff case. Run each **once** per condition.
2. **Conditions:**
   - **Skill:** a fresh subagent with the skill loaded and the case.
   - **Plain:** a fresh subagent given the case and a one-line request (e.g., "write tests for this" or "review this diff").

   Use the same model for both. **Neither runner may read `eval/keys/`.**
3. **Scoring:** a separate fresh subagent scores both outputs against the key and the rubric, without knowing which output is which (shuffle them, label them A/B). Then the lead spot-checks the scores by hand and notes any disagreement.
4. **Record** to `eval/results/pilot-<date>.md`: the raw outputs, the scores, any invented references, and the lead's notes.
5. **Label it a pilot:** n=1 per case, not statistically meaningful. The full protocol in `RUN.md` is for the developer to run.

---

## 7. Build report (`build/BUILD-REPORT.md`)

Keep it short. The developer reads this first.

- **What was built:** the file tree with one line per file.
- **Review first:** the 3–5 files that most need the developer's judgment, and why.
- **Pilot results:** a score table and an honest read. If the skill didn't beat the plain request, say so and give likely reasons.
- **Deviations** from the blueprint, plus anything under "Needs developer decision".
- **`TODO(source)`** items and known gaps.
- **How to try v1** on a real change: a short walkthrough.
- **Next:** what results from the developer's own use would justify expanding (from D6), and the next skill candidate from the D5 ranking.

---

## 8. Final self-check (before the last commit)

- [ ] Every D6 file exists, or is listed in the report with a reason.
- [ ] All relative links resolve. Check them with a script or a grep over `](`. Skill-to-handbook links follow the D4 pattern.
- [ ] Every claim carries its label and citation. No *no* claim from the claim check was used, and every *partly* claim keeps its caveat.
- [ ] Standards are cited by edition and clause, paraphrased, never copied.
- [ ] The quality-bar page is the single source of truth. No other page restates it differently.
- [ ] No Eloksen or personal details anywhere: grep for the project name and the developer's name and email.
- [ ] The example code compiles (`cargo check`), and the planted defects are deliberate and documented.
- [ ] Answer keys stay separate from the cases, and pilot runners never had access to them.
- [ ] Nothing was installed outside the repo, nothing was pushed, and `research/` and `cse370-prepare/` are unchanged (`git diff` against the baseline commit shows no changes there).
- [ ] The state file shows the build as complete, and the final commit is made.
