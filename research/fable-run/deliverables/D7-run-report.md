# D7. Run report

## 1. How the run went

- **Lead model.** The run started on Opus 5.5 and the developer switched it to Fable 5.2 during Phase 0. The Phase 0 reading happened on Opus. Every file was written, and every later phase run, on Fable 5.2.
- **Interruption.** All four workers stopped on the 5-hour usage limit seconds after launch (about 00:46 MST) and returned nothing. After the limit reset (03:00) each was resumed from its own transcript. Nothing saved was lost, which is what plan §12 is for.
- **Tools.** The lead had no direct page-fetch tool. Verification went through `web-fetch` agents given exact URLs and questions. Three standards came back as PDF bytes the fetch tool could not read; the lead extracted their text locally with `pdftotext` and read the passages directly.

## 2. What was delegated

| Worker | Model | Scope | Opened | What came back | Problems |
|---|---|---|---|---|---|
| W1 | Haiku 5.5 | Requirements and design artifacts, spec-driven tooling, change planning | 4 | Nygard's ADR sections; Gherkin's Given/When/Then; Spec Kit's command flow; Shape Up's "appetite" | No source for design docs vs ADRs, or for "done when" per step. The tier mapping was the worker's inference |
| W2 | Sonnet 5.5 | Testing, review, release, Rust tooling | 7 (4 + 3 tool pages) | Google review checklist and small-changes guidance; SRE release chapter; mutation-testing abstract; Clippy, cargo-deny, cargo-mutants | Test-layer and behavior-vs-implementation guidance not sourced; cargo-audit and Miri not opened; no official gate-order guidance |
| W3 | Sonnet 5.5 | AI studies, self-correction, package hallucination, agent guidance | 6 | METR, Spracklen, Huang, Olausson (abstract pages), Claude Code best practices, one PR-shape study | Papers read as abstracts only. No measured evidence on agents and tests. One search the brief asked for was not spent |
| W4 | Sonnet 5.5 | Standards (RQ6) | 6 | Editions and access for SSDF, SP 800-218A, SLSA, ASVS; a third-party summary of ISO/IEC 25010:2023 | **The weakest return.** No clause text read. SOC 2 behind a login; ISO pages returned HTTP 403; 13 standards not opened |
| Lead follow-up and verification | Fable 5.2 | Gaps that could change the first skill or the crosswalk; re-reading sources behind major recommendations | 5 new, plus deeper reads of 4 already opened | ImpossibleBench; *SWE at Google* ch. 12; skills and hooks docs; ISTQB syllabus; full text of METR and Spracklen; SSDF and SP 800-218A clause text; SLSA v1.2 levels | AICPA criteria still behind the login |

The workers were honest about what they did not read, which made triage straightforward. One worker correction mattered: W3 reported that Rust was absent from the package-hallucination paper. The full text shows Rust appears as a source of cross-language name confusion, with no rate of its own (C26).

## 3. Sources opened and cited

- **Opened: 28 distinct sources** (budget about 28): W1 4, W2 7, W3 6, W4 6, lead 5 new.
- **Cited as core: 18** (S1 to S18 in D3), within the 12 to 18 target. S18 groups three Rust tool pages.
- **Cited as supporting, with a caveat: 6** (read by a worker only).
- **Opened and not cited:** the ISO/IEC 25010 third-party summary (not an official source).

## 4. Claim-check table

The full table is [04-claim-check.md](../04-claim-check.md) and is included as an appendix in `REPORT.md`. Summary:

| Result | Count | Examples |
|---|---|---|
| yes | 29 | METR result (C17); test exploitation (C4, C5); ISTQB risk-based depth (C12); SSDF practices (C28, C41) |
| partly | 7 | Mutation-testing study, abstract only (C11); Clippy and cargo-deny docs, worker only (C29, C30); SRE "matched to risk" (C39) |
| no | 11 | Escalation triggers have no source (C16); the delegation ladder has no source (C21); same-model review untested (C34); SOC 2 CC8.1 not read (C42); ISO/IEEE clauses not read (C44) |

Every *no* row is either dropped from the report or presented with the label [Proposed] or "not read". Each of the five major recommendations rests on *yes* rows (table at the end of the claim-check file).

## 5. Remaining evidence gaps

| Gap | Affects | How much it could change the recommendations |
|---|---|---|
| Does the `test-plan` skill beat a plain request? | The first-skill choice | **High.** This is what the D5 evaluation measures. The build's pilot is a smoke test only |
| Does a same-model fresh-context review catch more than self-review? | H8; the value of the second skill | Medium |
| How often do agents weaken tests in everyday work (not on impossible tasks)? | The strength of the test-integrity rules | Low to medium: the rules are cheap either way |
| SOC 2 criteria text (CC8.1, CC7.x, separation of duties) | D3b audit-evidence map | Medium for RQ6. The map is built on SSDF and SLSA instead and says so |
| ISO/IEC/IEEE 29148, 25010, 12207, 42010, 29119; IEEE 1016; ISO/IEC 27001, 42001 | Crosswalk coverage; whether 25010 could structure the quality bar | Medium for RQ6, low for the workflow |
| Measured rates for agent-specific defects other than invented packages and test edits | Which quality-bar prompts are agent-specific | Low: they stay as review prompts labeled unmeasured |
| Outcome evidence for spec-first agent workflows | How much specification to require | Low |
| Security and delivery studies in the seed list (Perry et al., DORA, GitClear, Peng et al.) | Nothing in the report relies on them | Low |
| Official docs for cargo-audit, Miri and secret scanners | Three rows of the verification order and enforcement map, labeled [Proposed] or "not opened" | Low |
| Lightweight DDD, and LLM-generated adversarial tests | Two §3b claims have the verdict "not researched" | Low |

## 6. Inaccessible or unread

| Source | Why | How the report treats it |
|---|---|---|
| AICPA Trust Services Criteria PDF | Free, but behind an account login | SOC 2 criteria are not paraphrased anywhere |
| iso.org pages for ISO/IEC 25010 and others | HTTP 403; standards are paywalled | Listed as "not read"; no clause cited |
| Feathers, *Working Effectively with Legacy Code* | A book, not online | Named only as the origin of the term "characterization test" |
| "Anthropic AI-native playbook" | Not found; the one search allotted was not spent | Dropped |
| research.google page for the mutation-testing paper | HTTP 404 | The arXiv abstract was used instead |

## 7. Self-check against plan §11

| Check | Result |
|---|---|
| All three decisions can be made from D1 | Pass |
| Every [Observed] claim has a study card; no speed result presented as quality | Pass (D3 §2) |
| Every major recommendation is *yes* in the claim check | Pass, with the [Proposed] parts labeled |
| Every cited source has a link and a date or version; inaccessible ones flagged | Pass. Undated pages carry a fetch date |
| Labels sit next to the claims | Pass |
| Each hypothesis and §3b claim has a verdict; none repeated as fact | Pass. Two §3b claims have the verdict "not researched" |
| No speed claim without a study card | Pass |
| Every "must always" rule has a mechanical layer or an explicit human reason | Pass (D2 §5) |
| Enterprise-only practices marked and kept out of v1 | Pass |
| Every [Standard] claim names edition and clause; paywalled standards cited only through what was read | Pass. Unread standards have no clause cited |
| No standard presented as proof that a practice works | Pass |
| SOC 2 described accurately | Pass, as an attestation against the Trust Services Criteria. This description comes from the plan and the AICPA landing page; the criteria were not read |
| No private project or personal details | Pass (checked by search before the gate) |
| Demo outputs labeled illustrative; answer keys predate the demos | Pass. Keys were written at 00:30, before any worker ran |
| Sized for one junior developer | Pass |
| Plain language; each recommendation names its decision | Pass |

One plan target was not met in full: **D1 is over 300 words** once the 21 one-line verdicts are included. The recommendation itself (the three decisions and the tradeoffs) is about 250 words.
