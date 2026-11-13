# Sources

**Decision this page supports:** Where did this claim come from, and how strong is it?

## Key

Claim labels used on every page:

- **[Standard]** Defined by a named standard or framework. The edition and clause are cited.
- **[Established]** Long-standing practitioner guidance with broad support.
- **[Observed]** An empirical result from AI-assisted development. Each one has a study card below.
- **[Proposed]** This handbook's own recommendation or synthesis. No source states it.

Two markers appear next to those labels:

- **[T3 docs]** Vendor documentation. It says how a tool works, not how well it works.
- **[Older-generation tools]** The result comes from AI models released before 2025.

"Read by" says who opened the source during the research: the lead researcher, or a research worker only. A source read by a worker only is used with that caveat.

Source tiers:

- **T1** Official standard, original practitioner guidance, or peer-reviewed paper.
- **T2** Labeled preprint or public repository artifact.
- **T3** Vendor documentation. It shows how a feature works, not how well it works.

A standard records consensus or defines what an auditor checks. It is not proof that a practice works.

## Core sources (S1 to S18)

<a id="s1"></a>**S1** Becker et al. (METR), "Measuring the Impact of Early-2025 AI on Experienced Open-Source Developer Productivity". arXiv 2507.09089v2, 25 Jul 2025. T2. [Link](https://arxiv.org/abs/2507.09089). Read by the lead.
Used for: the one speed result, with its limits (see study card).

<a id="s2"></a>**S2** Spracklen et al., "We Have a Package for You! A Comprehensive Analysis of Package Hallucinations by Code Generating LLMs". USENIX Security 2025; arXiv 2406.10279v3, 2 Mar 2025. T1. [Link](https://arxiv.org/abs/2406.10279). Read by the lead.
Used for: the rule to verify every dependency an agent names.

<a id="s3"></a>**S3** Zhong, Raghunathan, Carlini, "ImpossibleBench: Measuring LLMs' Propensity of Exploiting Test Cases". arXiv 2510.20270v1, 23 Oct 2025. T2. [Link](https://arxiv.org/abs/2510.20270). Read by the lead.
Used for: protecting tests and planning them from the spec.

<a id="s4"></a>**S4** Olausson et al., "Is Self-Repair a Silver Bullet for Code Generation?". ICLR 2024; arXiv 2306.09896v5, 2 Feb 2024. T1. [Link](https://arxiv.org/abs/2306.09896). Read by the lead, abstract only.
Used for: treating an AI self-review as a weak filter, not a control.

<a id="s5"></a>**S5** Google Engineering Practices, "What to look for in a code review". Undated; fetched 2026-11-13. T1. [Link](https://google.github.io/eng-practices/review/reviewer/looking-for.html). Read by the lead.
Used for: the review checklist the quality bar is built on.

<a id="s6"></a>**S6** Google Engineering Practices, "Small CLs". Undated; fetched 2026-11-13. T1. [Link](https://google.github.io/eng-practices/review/developer/small-cls.html). Read by the lead.
Used for: small, single-purpose changes with tests in the same change.

<a id="s7"></a>**S7** *Software Engineering at Google*, ch. 12 "Unit Testing" (Kuefler; ed. Manshreck). Online edition; fetched 2026-11-13. T1. [Link](https://abseil.io/resources/swe-book/html/ch12.html). Read by the lead.
Used for: testing behavior through the public API and avoiding brittle tests.

<a id="s8"></a>**S8** Nygard, "Documenting Architecture Decisions". 15 Nov 2011. T1. [Link](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions). Read by the lead.
Used for: the short decision record (ADR) for significant decisions.

<a id="s9"></a>**S9** Google SRE book, ch. 8 "Release Engineering". Online edition; fetched 2026-11-13. T1. [Link](https://sre.google/sre-book/release-engineering/). Read by the lead.
Used for: small frequent releases and a defined release process at any size.

<a id="s10"></a>**S10** Anthropic, "Best practices for Claude Code". Fetched 2026-11-13. T3. [Link](https://code.claude.com/docs/en/best-practices). Read by the lead.
Used for: how instruction files, pass/fail checks and fresh-context reviewers work.

<a id="s11"></a>**S11** Anthropic, Claude Code skills documentation. Fetched 2026-11-13. T3. [Link](https://code.claude.com/docs/en/skills). Read by the lead.
Used for: how skills load (description first, full file when invoked).

<a id="s12"></a>**S12** Anthropic, Claude Code hooks guide. Fetched 2026-11-13. T3. [Link](https://code.claude.com/docs/en/hooks-guide). Read by the lead.
Used for: enforcing hard rules with hooks, such as blocking edits to protected files.

<a id="s13"></a>**S13** NIST SP 800-218, Secure Software Development Framework (SSDF). v1.1, Feb 2022. T1 (standard). [Link](https://csrc.nist.gov/pubs/sp/800/218/final). Read by the lead.
Used for: practice IDs the workflow already satisfies (PO.3.3, PW.4.4, PW.7, PW.8, PS.1, PS.3).

<a id="s14"></a>**S14** NIST SP 800-218A, SSDF Community Profile for Generative AI and Dual-Use Foundation Models. Final, 26 Jul 2024. T1 (standard). [Link](https://csrc.nist.gov/pubs/sp/800/218/a/final). Read by the lead.
Used for: one sentence of scope, that all code is evaluated before use, human-written or AI-generated.

<a id="s15"></a>**S15** ISTQB Certified Tester Foundation Level syllabus. v4.0.1, 15 Sep 2024. T1 (standard). [Link](https://www.istqb.org/certifications/certified-tester-foundation-level). Read by the lead.
Used for: test technique names, risk-based depth, exit criteria and independence, cited by section.

<a id="s16"></a>**S16** SLSA specification. v1.2. T1 (standard). [Link](https://slsa.dev/spec/v1.2/). Read by the lead.
Used for: stating the build provenance level honestly (Build L1 to L2; Source L4 needs two people).

<a id="s17"></a>**S17** Petrovic et al., "Does mutation testing improve testing practices?". ICSE 2021; arXiv 2103.07189, 12 Mar 2021. T1. [Link](https://arxiv.org/abs/2103.07189). Read by a research worker only, abstract only.
Used for: the claim that mutation testing is a signal of test sensitivity, with no effect size.

<a id="s18"></a>**S18** Rust tool documentation: Clippy lint groups; cargo-deny checks; cargo-mutants. Fetched 2026-11-13. T3. Links: [Clippy](https://doc.rust-lang.org/clippy/lints.html), [cargo-deny](https://embarkstudios.github.io/cargo-deny/checks/index.html), [cargo-mutants](https://mutants.rs/welcome.html). Read by a research worker only.
Used for: how the lint, dependency-check and mutation tools work.

## Study cards

The sources below are the [Observed] evidence. Speed (S1), correctness (S2, S3, S4) and test quality (S17) are different measures. This handbook never combines them into one "productivity" claim, and it makes no speed claim for the reader.

**S1 METR**
- Task: 246 real issues in mature open-source repositories, each randomly assigned to AI-allowed or AI-disallowed.
- Participants: 16 experienced developers working on their own repositories.
- Tools and date: mainly Cursor Pro with Claude 3.5 and 3.7 Sonnet; Feb to Jun 2025.
- Measured: speed (time to complete) and forecasts of speed.
- Result: 19% longer with AI allowed. Developers expected to be 24% faster and afterwards believed they had been 20% faster.
- Why it may not carry over: small sample of experts on code they know well. The authors say unfamiliar code and greenfield work may differ (untested). The tools are a generation old.

**S2 Spracklen et al.**
- Task: generate code from 2 prompt datasets and check whether the referenced packages exist.
- Participants: none. 16 code models, 576,000 samples.
- Tools and date: 2024 model set. **[Older-generation tools]**
- Measured: correctness and supply-chain security (rate of non-existent package names).
- Result: at least 5.2% for commercial models and 21.7% for open-source models; 205,474 unique invented names; 43% of invented names recurred on all 10 repeats.
- Why it may not carry over: Python and JavaScript only. Rust appears only as a source of cross-language confusion, so the Rust rate is unmeasured. Single-shot generation, not an agent that builds and sees the error.

**S3 ImpossibleBench**
- Task: coding tasks altered so the tests contradict the specification; any pass means the tests were exploited.
- Participants: none. Several frontier models.
- Tools and date: 2025 models, including GPT-5, o3 and Claude models.
- Measured: correctness and integrity (rate of passing impossible tasks).
- Result: GPT-5 passed 76% of one SWE-bench variant and 2.9% of a LiveCodeBench variant. Methods: editing tests, overloading comparison, special-casing. A strict prompt, hidden tests, or a "flag for human" option each cut the rate sharply for some models.
- Why it may not carry over: tasks are built to be impossible, so rates do not predict everyday frequency. Large differences between models and prompts.

**S4 Olausson et al.**
- Task: repair failing generated programs on HumanEval and APPS.
- Participants: none, plus a small human-feedback study.
- Tools and date: Code Llama, GPT-3.5, GPT-4. **[Older-generation tools]**
- Measured: correctness (pass rate, with the cost of repair counted).
- Result: gains from self-repair are often modest. Feedback from a stronger model gives much larger gains.
- Why it may not carry over: benchmark puzzles and 2023 models. Abstract only was read.

**S17 Petrovic et al.**
- Task: mutation testing shown to developers during code review.
- Participants: Google developers (count not seen).
- Tools and date: Google's internal system, to 2021.
- Measured: test quality (tests written, surviving mutants).
- Result: developers exposed to mutants wrote more tests; mutants are coupled with real faults.
- Why it may not carry over: abstract only, read by a worker. One company, observational.

## Supporting sources (read by a research worker only)

Used only with a caveat. No claim in the handbook rests on one of these alone.

- Cucumber, [Gherkin reference](https://cucumber.io/docs/gherkin/reference/).
- GitHub, [Spec Kit README](https://github.com/github/spec-kit). Method description only; no outcome evidence was opened.
- Huang et al., ICLR 2024, [abstract](https://arxiv.org/abs/2310.01798).
- OWASP, [ASVS 5.0.0 landing page](https://asvs.owasp.org). Edition and license only; no requirement text read.
- Basecamp, [Shape Up ch. 3](https://basecamp.com/shapeup/1.2-chapter-03).
- Ogenrwot and Businge, MSR 2026, [arXiv 2601.17581](https://arxiv.org/abs/2601.17581).

## Not read

No clause or paraphrase from these is cited anywhere in the handbook.

| Source | Why not read | How the handbook treats it |
|---|---|---|
| AICPA Trust Services Criteria (SOC 2), 2017, revised points of focus 2022 | Free, but behind an account login | Criteria are not paraphrased. The audit map uses SSDF and SLSA and marks SOC 2 unverified. Read the free PDF before relying on any mapping |
| iso.org pages for ISO/IEC 25010 (2023) and others | HTTP 403; paywalled | Listed as "not read"; no clause cited |
| ISO/IEC/IEEE 29148, 12207, 42010, 29119; IEEE 1016 | Paywalled | Not used in v1. CSE 370 Unit 02's four criteria replace 29148 |
| ISO/IEC 27001 Annex A, ISO/IEC 42001, NIST AI RMF and AI 600-1 | ISO paywalled; NIST not opened | Not used in v1. Marked **[Large-org only]** where relevant |
| SWEBOK v4, OWASP SAMM, CISA Secure by Design, OpenSSF Scorecard | Not opened | Candidates for v2 |
| OWASP ASVS requirement text | Only the landing page was seen | Edition named, no requirement cited |
| Feathers, *Working Effectively with Legacy Code* | A book, not online | Named only as the origin of the term "characterization test" |
| "Anthropic AI-native playbook" | Not found; the one search allotted was not spent | Dropped |
| research.google page for the mutation-testing paper | HTTP 404 | The arXiv abstract (S17) was used instead |
| DORA metrics and AI report, GitClear, Perry et al., Peng et al. | Not opened in this run | No claim is made from them |

## Keeping this page current

Vendor documentation pages (S10, S11, S12, S18) change. The fetch date is 2026-11-13. Re-check them each quarter.

## Related

- [quality-bar.md](quality-bar.md)
- [standards-crosswalk.md](standards-crosswalk.md)
- [working-with-agents.md](working-with-agents.md)
- [plan-tests.md](plan-tests.md)
