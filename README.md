# SDLC handbook for AI-assisted development

A short field guide for one developer who ships real software with the help of coding agents, plus agent skills that apply it. Each page answers one decision. Each rule has a stable ID, a label that says how strong its evidence is, and a source.

**Version:** v1.0 (2026-11-13). **Status:** first build. The first skill has had a pilot run only, not a full evaluation. See [build/BUILD-REPORT.md](build/BUILD-REPORT.md).

## Pick a tool by task

| I am about to... | Start here | Then |
|---|---|---|
| Fix a small bug | [Risk tiers](handbook/risk-tiers.md): confirm it is Tier 1 | Run the [`test-plan` skill](skills/test-plan/SKILL.md), write the failing test first, check the [quality bar](handbook/quality-bar.md) |
| Build a feature | [Workflow](handbook/workflow.md) | [Acceptance criteria](templates/acceptance-criteria.md), then the [`test-plan` skill](skills/test-plan/SKILL.md), then [verify](handbook/verify-a-change.md) |
| Make a high-risk change (auth, data migration, money, hard to undo) | [Risk tiers](handbook/risk-tiers.md): the escalation triggers | An [ADR](templates/adr.md), a Tier 3 [test plan](templates/test-plan.md), a line-by-line human review |
| Review a change, mine or an agent's | [Review a PR](handbook/review-a-pr.md): pick the tier, then review to that depth | [Quality bar](handbook/quality-bar.md), then [verify a change](handbook/verify-a-change.md): what each check proves and does not prove |
| Release | [Verify a change](handbook/verify-a-change.md), then the [PR description](templates/pr-description.md) | A release page is planned for v2. Until then, tag the release and keep a last-good version you can redeploy |
| Hand a task to an agent | [Working with agents](handbook/working-with-agents.md) | The delegation ladder and the enforcement map |
| Answer "which standard says so?" | [Standards crosswalk](handbook/standards-crosswalk.md) | [Sources](handbook/sources.md) |

## Folder map

| Folder | Holds |
|---|---|
| [handbook/](handbook/) | Reusable guidance. Eight short pages in v1, plus [Review a PR](handbook/review-a-pr.md) (added 2026-11-16, all [Proposed]) |
| [skills/](skills/) | Agent instructions. One self-contained folder per skill |
| [templates/](templates/) | One-screen templates, each with a filled-in synthetic example |
| [examples/](examples/) | A small synthetic Rust crate with planted defects, and walkthroughs |
| [eval/](eval/) | The evaluation kit: cases, separate answer keys, rubric, how to run it, results |
| [build/](build/) | Build state, deviations from the blueprint, and the build report |
| [cse370-prepare/](cse370-prepare/) | Course study notes the handbook cross-references |
| [research/](research/) | The research run that this handbook is built from |

Project-specific decisions do not live here. ADRs, acceptance criteria, test plans, hook settings and CI configuration live in each project's own repository.

## How to read a rule

```
TP-3  Test each boundary and its neighbors.  [Standard] ISTQB CTFL v4.0.1 (2024) §4.2.2 (S15)
```

- **TP-3** is a stable ID. It is never renumbered or reused.
- The label says what kind of support the rule has:
  - **[Standard]**: defined by a named standard. That is expert consensus, not proof that the practice works.
  - **[Established]**: long-standing practitioner guidance.
  - **[Observed]**: an empirical result. Its study card is in [sources](handbook/sources.md).
  - **[Proposed]**: this handbook's own synthesis. Treat it as provisional.
  - **[T3 docs]** marks vendor documentation: it says how a tool works, not how well. **[Older-generation tools]** marks a result from AI models released before 2025.
- **S15** is an entry in [handbook/sources.md](handbook/sources.md), which has the link and the date.

## Use a skill in another repository

1. Copy the whole skill folder, for example `skills/test-plan/`, into the other repository at `.claude/skills/test-plan/`. For use in every project, copy it to `~/.claude/skills/test-plan/` instead.
2. Keep `SKILL.md` and `reference.md` together. The skill cites rules by ID, and `reference.md` carries a pinned copy of each rule it may cite, so the links work without this handbook being present.
3. Invoke it with `/test-plan`, or let the agent pick it up when a request matches its description.

Nothing in this repository installs itself. Copying a skill is your decision after you have read it.

## Keeping this handbook honest

| Action | When | How |
|---|---|---|
| Add a rule or page | You hit the same decision twice and the handbook did not answer it | It needs a source, or the label [Proposed] and a review date. One decision per page |
| Revise | A source changes edition, or your evaluation or real use contradicts the rule | Change the page and every `reference.md` that carries the ID in the same commit. Bump the version above and the pinned line in `reference.md` |
| Retire | A [Proposed] rule is unused or unconfirmed after two quarterly reviews | Mark it retired with the date. Keep the ID |
| Promote from [Proposed] | A primary source is found, or your own records show the rule caught a real problem twice | Change the label and record the evidence in [sources](handbook/sources.md) |
| Review | Once a quarter | Re-fetch the vendor documentation pages (they change) and check each [Proposed] rule |
