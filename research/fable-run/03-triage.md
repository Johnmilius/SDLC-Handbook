# Phase 2 triage: gap and conflict matrix

Written 2026-11-13 03:30 MST, from the four worker files. Sources opened so far: 23 of about 28.

## Matrix

| RQ | Status | What is supported | What is missing or conflicting | Could it change the handbook structure or the first skill? |
|---|---|---|---|---|
| RQ1 Everyday workflow | **Partly supported** | ADR contents (Nygard). Given/When/Then form (Gherkin reference). Small, single-concern changes with tests in the same change (Google "Small CLs"). Rollout matched to risk (SRE book). | No source ties artifacts to risk tiers. No source for "done when" per step or for design docs vs ADRs. Tiering stays [Proposed], backed only by "match depth to risk" guidance. | No. Tiers are a design choice the plan already treats as a hypothesis (H5, H10). |
| RQ2 What changes with an agent | **Partly supported** | Speed: METR trial (slower, and perception was wrong). Self-repair is limited without outside feedback (Olausson, Huang). Vendor guidance: short instruction files, deterministic hooks for hard rules, reviewer in a separate context. | **No measured evidence that agents edit or weaken tests or write mirrored tests.** No review-effort measurement. No evidence that a fresh context with the *same* model helps (only vendor guidance says so). | **Yes, for the first skill.** If agents are measured to game tests, a skill that plans tests independently of the implementation gains value. This is the follow-up target. |
| RQ3 Systems to borrow from | **Supported for a small set** | Nygard ADR, Gherkin, Google eng-practices, SRE release chapter, Spec Kit method (commands only), Claude Code best practices. | Spec Kit artifact names unverified. Shape Up gave only "appetite". Test-layer and behavior-vs-implementation guidance not sourced. | No for structure. The missing test-design source affects the first skill's content, so it joins the follow-up. |
| RQ4 First skill | **Open until follow-up** | Reviewer checklist (eng-practices) supports a review skill. Test-design techniques are in the reader's course notes. | No direct evidence on which skill prevents more costly mistakes. The ranking will rest on [Proposed] scoring. | Yes. See follow-up. |
| RQ5 Quality bar | **Supported** | eng-practices "what to look for" (12 items). Clippy groups and restriction lints, cargo-deny's four checks, cargo-mutants purpose. Package hallucination measured (not for Rust). | cargo-audit and Miri docs not opened. No official guidance on gate order (cheap-first is inference). Agent-specific defects other than package hallucination are unmeasured in what was read. | No. Unmeasured items get labeled as anecdote or [Proposed]. |
| RQ6 Standards | **Mostly missing** | Editions and access confirmed for SSDF v1.1, SP 800-218A, SLSA (v1.2 current), ASVS 5.0.0. ISO/IEC 25010:2023 changes via a third-party page. | **SOC 2 criteria text not read (login wall).** SSDF practice IDs not seen. ISO/IEEE standards not opened (paywall or 403). ISTQB, SWEBOK, SAMM, CISA, Scorecard, 27001, AI RMF, 42001 not opened. 29119 criticism not sourced. | **Yes, for H11.** A crosswalk that cannot name clauses it has read is not a crosswalk. Follow-up must get the free standards' clause IDs. Paywalled ones get marked "not read". |

## Conflicts

1. **Speed.** The plan's seed list includes vendor studies that report faster work. The one trial opened (METR) reports slower work for experienced developers on their own repos. These differ on participants (experts vs general), task (real issues in mature repos vs bounded tasks), and tool generation. Only METR was opened, so the report will state the METR result with its study card and make **no** speed claim in either direction for the reader.
2. **Self-review.** Vendor guidance recommends a fresh-context reviewer. The research opened shows that self-repair is weak and that feedback from a *stronger* model or a human helps. Nothing tests "same model, fresh context". H8 can be at most "modified": independence of inputs is [Proposed], backed by vendor guidance (T3), not by a study.

## The one targeted follow-up (lead, with Phase 3)

Scope is limited to gaps that could change the first-skill choice or the viability of the standards crosswalk. The lead opens these new sources (budget: 5, which brings the run to about 28):

| # | Source | Gap it closes |
|---|---|---|
| F1 | A measured study of models exploiting or modifying tests (lead candidate: "ImpossibleBench", arXiv 2025) | RQ2/RQ5: is test-gaming measured or anecdote? Affects the first-skill ranking. |
| F2 | *Software Engineering at Google*, ch. 12 "Unit Testing" (free online) | RQ3/RQ4: test behavior, not implementation. Content for the test skill. |
| F3 | ISTQB CTFL syllabus v4.0 (free PDF) | RQ6: citable sections for test techniques, risk-based testing, independence of testing. |
| F4 | Claude Code Agent Skills documentation | RQ2/H1/H3: how skills load and where supporting files go. Also needed by the build. |
| F5 | AICPA Trust Services Criteria PDF (one attempt) | RQ6: CC8.1 wording. If the login wall blocks it, SOC 2 content is marked "criteria text not read". |

Re-opened for clause detail (same sources the workers opened, so they do not add to the count): the SP 800-218 PDF (practice IDs), the SLSA v1.2 levels page, the SP 800-218A PDF.

## Dropped, with reason

- Perry et al., DORA 2025, GitClear, Peng et al.: not opened. The report makes no security-rate, DORA or duplication claims from them. Listed in D7 as "not opened".
- ISO/IEC/IEEE 12207, 29148, 42010, 29119, IEEE 1016, ISO/IEC 25010, 27001, 42001: paywalled or blocked. They appear in the D3b crosswalk as "not read", with no clause paraphrase.
- Shape Up beyond "appetite", MADR, Adzic, Google design docs: not opened. Not cited.
- The "Anthropic AI-native playbook": not found by any worker. Reported as unconfirmed and not used.
