# W1: SDLC frameworks and artifacts

Model: Haiku 5.5 (fallback Sonnet 5.5). Brief written 2026-11-13 00:40 MST.

## Brief sent

```
ROLE: Research assistant for a software-engineering handbook. Return evidence notes, not prose.
QUESTION:
  1. What is the smallest useful written artifact for requirements on a single change (problem statement, Given/When/Then acceptance criteria, specification by example), and what does the original guidance say it must contain?
  2. What do the original sources say an Architecture Decision Record must contain, when to write one, and how design docs/RFCs differ from ADRs? When is each worth the effort?
  3. What does spec-driven development for AI coding agents (for example GitHub Spec Kit) actually prescribe as artifacts and steps? Record the method only, not benefit claims.
  4. Is there original guidance on change planning as ordered steps with a "done when" check, dependencies and risk per step (for example Shape Up scopes, appetite and hill charts)?
  Key question: which artifacts are worth the effort at which risk tier (small bug fix / normal feature / high-risk change)?
READER: A junior developer, solo or on a small team, maintaining real software (mainly Rust, some TypeScript or Python). Uses AI coding agents interactively and for bounded autonomous tasks.
SCOPE: lightweight requirements, ADRs, design docs/RFCs, spec-driven development, change planning. OUT OF SCOPE: testing practice, CI/CD, AI productivity studies, formal ISO/IEEE standards (other workers cover those).
SOURCE RULES: Open every source you report (use WebSearch to find it and WebFetch to read it; if you have no such tools, say so at the top and stop). Prefer official handbooks, original engineering guidance, peer-reviewed or clearly labeled preprints, and public repos with real artifacts. Vendor docs only for how features work. Treat productivity claims by their evidence. Never report a detail (number, section, date) you did not see on the page.
LEADS (unverified; use, replace, or reject):
  - Michael Nygard, "Documenting Architecture Decisions" (2011), cognitect.com blog
  - MADR (Markdown Architectural Decision Records), adr.github.io/madr
  - "Design Docs at Google" (industrialempathy.com) or another first-hand description of Google-style design docs
  - Shape Up (Basecamp), basecamp.com/shapeup
  - GitHub Spec Kit, github.com/github/spec-kit
  - Cucumber docs on Gherkin Given/When/Then, or Gojko Adzic on Specification by Example
BUDGET: Up to 4 opened sources. Stop when your sub-questions are answered. Pick the 4 that best cover sub-questions 1 to 4.
PRIVACY: Use no project names, private details or personal information in any search. You have no local file access and need none. Do not write any files.
OUTPUT, one block per source:
  - Title, author/org, date/version, direct link
  - Tier (T1 official standard, original practitioner guidance or peer-reviewed / T2 labeled preprint or public repo artifact / T3 vendor docs / T4 vendor claims, blogs, surveys) and claim type (Established / Observed)
  - Finding: the specific claim it supports, with the section/page where you saw it
  - Study card (empirical sources only): task · participants · tools/models+date · measure · result
  - Limitation: why this may not apply to the reader
  - Handbook implication: one sentence
THEN: list any gaps or conflicts you noticed and any leads you rejected and why. Keep the whole reply under 900 words.
```

## Notes returned

Returned 2026-11-13 about 03:10 MST (Haiku 5.5, resumed once after a usage-limit stop). 4 of 4 sources opened. Saved as returned, lightly reformatted.

### Source 1: "Documenting Architecture Decisions", Michael Nygard, Cognitect blog, 15 Nov 2011
- Link: https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions
- Tier: T1 (original practitioner guidance). Claim type: Established.
- Finding: Required sections are Title (short noun phrase), Context (forces, stated as facts), Decision (full sentences, active voice, "We will..."), Status (proposed, accepted, later deprecated or superseded), Consequences (all of them, not just the positive ones). Write one for "architecturally significant" decisions (structure, non-functional characteristics, dependencies, interfaces, construction techniques), one decision per record. The post criticizes large specification documents as never kept up to date and prefers small modular documents. It does not compare ADRs with design docs.
- Limitation: A single short blog post from 2011. Nothing on templates, review process, or design docs/RFCs.
- Handbook implication: A one-page ADR (title, context, decision, status, consequences) is the minimum record for a significant decision. Keep it short so it stays current.

### Source 2: Shape Up, Ryan Singer / Basecamp, chapter 3 "Set Boundaries"
- Link: https://basecamp.com/shapeup/1.2-chapter-03 (date/version not shown on the fetched page)
- Tier: T1 (original practitioner guidance). Claim type: Established.
- Finding: Appetite is a time budget, not an estimate: "Small Batch" (1 to 2 weeks) or "Big Batch" (six-week cycle). Fixed time, variable scope. Shaping order: raw idea, appetite, narrowed problem, solution elements. Chapter 3 has no dependencies, risk, or "done when" checklist. The table of contents points to Chapter 5, "Risks and Rabbit Holes", which was not opened.
- Limitation: Designed for a product team on six-week cycles. A solo developer can borrow the appetite idea, but the cycle structure does not transfer directly. Risk and done-criteria content is unverified.
- Handbook implication: Before planning, write down a time budget and a narrowed problem statement. Keep the solution flexible within that budget.

### Source 3: GitHub Spec Kit README
- Link: https://github.com/github/spec-kit (no version or release date shown)
- Tier: T2 (public repo artifact). Claim type: method as described in the README.
- Finding: The spec-driven flow runs as agent chat commands, in order: `/speckit-constitution` (project principles, once per project), `/speckit-specify` (feature spec), `/speckit-plan`, `/speckit-tasks`, `/speckit-implement`, `/speckit-converge`. Implement and converge repeat until convergence is reported. Clarification, checklists and consistency analysis are covered in a linked walkthrough that was not opened. The README does not name `spec.md`, `plan.md` or `tasks.md`. The paths it does name are `.specify/bugs/<slug>/` and `.specify/assessments/<slug>/`.
- Limitation: Tooling is tied to specific agent CLIs and the commands may change. Expected artifact file names were not confirmed on this page.
- Handbook implication: The method can be reproduced as constitution, spec, plan, tasks, implement, check, with files kept in the repo, without adopting the tool.

### Source 4: Cucumber Gherkin Reference
- Link: https://cucumber.io/docs/gherkin/reference/ (page says last updated Nov 11, 2026)
- Tier: T1/T2 (official tool-maintainer documentation). Claim type: Established.
- Finding: Given sets the initial context. When describes an event or action, with implementation details kept in step definitions. Then describes an observable expected outcome. And/But replace repeated keywords. Recommends 3 to 5 steps per example. A Rule holds one business rule with scenarios illustrating it. Examples are concrete and double as executable tests.
- Limitation: Describes syntax, not what a requirement must contain. Specification by Example (Adzic) was not opened.
- Handbook implication: Write each acceptance criterion as one Given/When/Then example, 3 to 5 steps, one observable outcome.

### Answers to sub-questions
1. Requirements artifact: Given/When/Then examples, one Rule per business rule. Minimum content beyond that is not sourced.
2. ADR: Nygard's five sections. When: architecturally significant decisions only. Design docs/RFCs vs ADRs: **no source found.**
3. Spec-driven method: the six-step Spec Kit command flow. Artifact file names not confirmed.
4. Change planning: Shape Up chapter 3 gives appetite and boundary-setting only. **No source found** for ordered steps with done-when, dependencies, or risk per step.

Key question (which artifact at which tier): **not answered by any source here.** The worker's tier mapping is its own inference: small fix = one example or none; normal feature = examples plus a short plan; high-risk change = an ADR plus a written plan with per-step done-when.

### Gaps and rejected leads
- No source on design docs/RFCs vs ADRs (Google design-doc leads not opened: budget).
- Spec Kit artifact names unverified.
- Shape Up chapter 5 (risks, rabbit holes) not opened.
- MADR and Adzic not opened (budget).
