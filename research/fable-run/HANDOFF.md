# Handoff to the build

Written 2026-11-13 05:40 MST, after the research gate passed. The build works from the saved files, not from conversation memory.

1. **Handbook model:** a short field guide (one decision per page, rules with stable IDs, every claim labeled and cited) plus self-contained agent skills. Source: D1, D4.
2. **First skill:** `test-plan`. For a spec or a diff it says what could break, which check would catch it and at which layer, and what stays unverified. Spec: D5 section 1.
3. **Workflow:** three risk tiers that scale depth and never skip a stage; mechanical escalation triggers; deterministic checks, then independent AI review, then human review. Source: D2.
4. **Quality bar:** 13 rules (QB-1 to QB-13), each with an enforcement layer and a source. Full text: D4 section 1. It is the single source of truth.
5. **Linking:** rule IDs, not paths. Each skill folder carries a `reference.md` pinned to a handbook version, so links survive copying. Source: D4 section 4.
6. **v1 scope (D6):** `README.md`; 8 handbook pages (quality-bar, workflow, risk-tiers, plan-tests, verify-a-change, working-with-agents, standards-crosswalk, sources); `skills/test-plan/SKILL.md` and `reference.md`; 4 templates (acceptance-criteria, test-plan, pr-description, adr); a dependency-free example crate with 2 walkthroughs; an eval kit with 8 cases, 8 separate keys, a rubric and `RUN.md`; 3 build records. 43 files.
7. **Left out of v1:** four later pages, the second skill (`review-change`), hook scripts and CI config, anything marked [Large-org only].
8. **Evidence rules for the build:** use claim-check rows marked *yes* as stated; *partly* rows keep their caveat (C11, C29, C30, C35, C36, C39, C43); *no* rows stay out or are labeled [Proposed] or "not read" (C16, C21, C26, C31, C34, C37, C42, C44 to C47).
9. **Do not claim:** any speed gain; any SOC 2 criterion text; any ISO/IEEE clause; a Rust package-hallucination rate; that an AI reviewer is a control.
10. **Re-check at B5:** the Agent Skills frontmatter and loading rules came back as a tool summary (C3). Read the docs again before writing `SKILL.md`.
