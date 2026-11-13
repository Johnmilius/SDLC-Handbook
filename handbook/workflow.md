# Workflow: one change, start to finish

**Decision this page supports:** What do I do next on this change, and what do I need before moving on?

**CSE 370:** [Unit 07, The SDLC Revisited](../cse370-prepare/unit07-sdlc-revisited.md) supports the idea that every phase gets attention and that "shift left" means thinking about later phases early.

Work through the eight stages in order. A stage is done when its "evidence to proceed" is true, not when time has passed. [Established] A methodology may change the timing, frequency or style of a phase, never whether it happens; no stage is skipped, only sized differently. CSE 370 Unit 07 ([unit file](../cse370-prepare/unit07-sdlc-revisited.md)).

[Standard] A passing stage is evidence, not proof. "Tests pass" shows only that the tests pass. ISTQB CTFL v4.0.1 (2024) §1.3, principles 1 and 7 ([S15](sources.md#s15)).

How deep each stage goes depends on risk, not on diff size. See [risk-tiers.md](risk-tiers.md). The tier is the developer's call; an agent may only propose one. [Proposed]

## The eight stages

| Stage | Developer decision | Agent contribution | Minimum artifact | Evidence to proceed | Sign-off |
|---|---|---|---|---|---|
| <a id="wf-1"></a>**WF-1** Problem | Is this worth doing, and what is out of scope? | Restate the problem; list what it could not find out | One-line problem statement plus a "not included" line | The developer can say who is affected and what "fixed" looks like | Developer |
| <a id="wf-2"></a>**WF-2** Requirements | What must be true when it is done? Which tier is this? | Draft criteria; list ambiguities as questions; propose a tier | Given/When/Then acceptance criteria with negative cases ([template](../templates/acceptance-criteria.md)). [Standard] ISTQB CTFL v4.0.1 (2024) §4.5.2 ([S15](sources.md#s15)) | Each criterion is testable and no open question changes behavior | Developer decides the tier |
| <a id="wf-3"></a>**WF-3** Design | What is the simplest design that meets the criteria? Is any decision hard to reverse? | Propose options and trade-offs; find code to reuse | Tier 1: none. Tier 2: a short plan, each step with a "done when". Tier 3: an ADR per significant decision ([template](../templates/adr.md)). [Established] Nygard (2011) ([S8](sources.md#s8)) | Each step has an exit check. [Standard] ISTQB CTFL v4.0.1 (2024) §5.1.3 ([S15](sources.md#s15)) | Developer |
| <a id="wf-4"></a>**WF-4** Test plan, written before the code changes | What could break, which check catches it, at which layer, and what stays unverified? | Run the [test-plan skill](../skills/test-plan/SKILL.md); derive checks from the criteria, not from the implementation [Proposed] | A test plan sized to the tier ([template](../templates/test-plan.md)); see [plan-tests.md](plan-tests.md) | Every acceptance criterion maps to at least one check; open product questions are answered | Developer |
| <a id="wf-5"></a>**WF-5** Implementation | Accept, change or reject each diff | Write code and tests in small, single-purpose changes; refactors go in a separate change. [Established] Google "Small CLs" ([S6](sources.md#s6)) | The diff, with tests in the same change | Deterministic checks pass (see [verify-a-change.md](verify-a-change.md)) | Agent edits are fine at Tier 1 and 2 [Proposed]; the developer approves any changed test expectation or new dependency |
| <a id="wf-6"></a>**WF-6** Verification | Is the evidence enough for this tier? | Run checks; run an independent review in a fresh context; report what was not verified | PR description: what changed, why, how it was verified, what was not ([template](../templates/pr-description.md)) | The verification order was completed to the depth the tier requires [Proposed] | Developer; Tier 3 needs a line-by-line human review [Proposed] |
| <a id="wf-7"></a>**WF-7** Release | Ship now? How do we undo it? | Draft the release note and rollback note | Tier 1 and 2: tagged release and a note. Tier 3: rollback note and a watch list [Proposed] | A known last-good version exists to redeploy [Proposed] | Developer |
| <a id="wf-8"></a>**WF-8** Maintenance | Did it work for users? What did we learn? | Summarize errors and issues; propose follow-up tasks | A change-log line; an issue for anything deferred [Proposed] | The watch list from WF-7 is quiet, or an issue is open [Proposed] | Developer |

## Notes on the rows

- WF-2 and WF-4: the tier-by-tier artifact sizes are in [risk-tiers.md](risk-tiers.md). The tier names and escalation triggers are [Proposed] because no source lists them.
- WF-6: vendor guidance recommends a reviewer in a fresh context ([S10](sources.md#s10)). Whether a same-model reviewer catches more than the author is untested, so treat the review as an extra filter, not a control. [Proposed]
- WF-6: the checklist for what a reviewer looks at is in [quality-bar.md](quality-bar.md).
- Agents and tests: [working-with-agents.md](working-with-agents.md) covers what an agent may change on its own.

## Related

- [quality-bar.md](quality-bar.md)
- [risk-tiers.md](risk-tiers.md)
- [plan-tests.md](plan-tests.md)
- [verify-a-change.md](verify-a-change.md)
- [working-with-agents.md](working-with-agents.md)
- [standards-crosswalk.md](standards-crosswalk.md)
- [sources.md](sources.md)
