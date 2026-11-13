# Context memo (Phase 0, lead only)

Written 2026-11-13 00:25 MST. Private to the lead. Never copied into worker briefs.
Local access: the CSE 370 notes and the two permitted example spec files were both readable, so no conclusion needs the "[Needs validation against local notes]" mark for lack of access.

## 1. The reader's vocabulary (from the CSE 370 notes)

The reader already knows these terms, so the handbook can use them without defining them:

- Lifecycle: SDLC phases, Waterfall, V-Model, Agile, Scrum, Kanban, DevOps, Shift Left.
- Requirements: user story, Given/When/Then acceptance criteria, use case, functional vs non-functional, the four quality criteria (completeness, ambiguity/precision, feasibility, timeline/scope), documented assumptions.
- Design: levels of design, interface as contract, UML class and sequence diagrams, data dictionary, design patterns, "a stale design can be worse than none".
- Testing: test pyramid, unit / integration / E2E / UAT, boundary value analysis, equivalence partitioning, white-box vs black-box, coverage (and gaming it), traceability matrix, regression testing, static analysis, tester mindset.
- Release: CI/CD, continuous delivery vs deployment, feature flags, canary, blue/green, SemVer, rollback, monitoring, the four maintenance types, technical debt (including testing debt), change control.

Terms the notes do not cover, which the handbook must introduce briefly: ADR, characterization test, property test, mutation testing, fuzzing, expand/contract migration, SLO, supply-chain provenance, test oracle, risk tier, hook.

## 2. Which CSE 370 unit can support which handbook section

| Handbook topic | Unit | What the unit already gives |
|---|---|---|
| Problem and requirements | 02 | Given/When/Then, the four quality criteria, ambiguity red flags, documented assumptions, "formality depends on the cost of failure" |
| Design and decisions | 03 | Design as communication, interfaces as contracts, levels of design, stale docs are harmful, "AI works best when the problem is well defined" |
| Test planning | 04 | Risk-based "how much is enough", pyramid layers, boundary values, partitions, coverage is gameable, traceability matrix, where tests run (local / CI / staging / production), simulation is not the real thing |
| Release and maintenance | 05 | Small frequent releases, regression on every commit, flags, canary, rollback approval, dependency updates as preventive maintenance, testing debt |
| Risk tiers | 04, 07 | "Every phase manages risk"; depth depends on the cost of failure; a methodology changes timing and style of a phase, never whether it happens (supports H10 directly) |
| Verification order | 04, 07 | Static analysis and automation as early gates (Shift Left) |
| Review and ownership | 04, 07 | Code review, "you own the correctness of your code", no "throwing it over the fence" |

Unit 06 (project management) and Unit 01 (methodologies) are background only. Nothing in v1 depends on them.

## 3. Patterns from the two private example specs (abstract form)

These came from a build-order plan and a testing spec for a desktop application port. Only the shapes are recorded here.

1. **"Done when" per step.** Each build step has one observable completion check, and the plan carries no time estimates.
2. **Dependencies and main risk per step.** A dependency graph shows which steps can run in parallel. Each step names its biggest risk, and the riskiest step is flagged.
3. **Milestones tied to steps**, each defined by what works end to end.
4. **Long-lead items started early in parallel** (work that is mostly waiting, such as approvals and recruiting testers).
5. **An explicit "not included" list** to bound scope.
6. **Test layers table: layer, tool, what it covers.** The layers run from unit, to parity against golden files from a reference implementation, to integration with real devices, to visual side-by-side comparison, to manual checklists.
7. **A release gate:** the release script refuses to run unless the automatic tests pass.
8. **CI vs real environment split.** CI runs unit and golden-file tests on every push. Tests that need real hardware or real models are kept for a physical machine, with a list of the hardware needed and why.
9. **Injected dependencies** (clock, secret store, devices, clipboard) so logic can be tested with fakes.
10. **Reference outputs as the contract.** A difference from the reference is a bug unless a spec lists it as intended.
11. **"How we'll know" checklist and open questions** at the end of the spec.

How these map to hypotheses: patterns 1 to 5 bear on H6 (done-when, dependencies and risk per step). Patterns 6 to 8 bear on H6 (CI checks vs real-environment checks) and H7. Pattern 10 is a form of characterization or golden-master testing.

## 4. What the reader needs from the handbook (from the plan, restated)

Decisions stay with the developer. Skill output teaches by the chain *finding, evidence, proposed action, why it matters for this change*. Both new and existing code must be covered. Rust first.
