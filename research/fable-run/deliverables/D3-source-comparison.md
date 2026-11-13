# D3. Source and system comparison

Supports **Decision 1** (what the handbook contains). Claim numbers refer to [04-claim-check.md](../04-claim-check.md).

## 1. Core sources (18)

Tiers: T1 official standard, original practitioner guidance, or peer-reviewed. T2 labeled preprint or public repository artifact. T3 vendor documentation (how a feature works, not how well). "Read by" says who opened it: **L** the lead, **W** a worker only.

| # | Source | Date / version | Tier | Read by | Link |
|---|---|---|---|---|---|
| S1 | Becker et al. (METR), "Measuring the Impact of Early-2025 AI on Experienced Open-Source Developer Productivity" | arXiv 2507.09089v2, 25 Jul 2025 | T2 | L | https://arxiv.org/abs/2507.09089 |
| S2 | Spracklen et al., "We Have a Package for You! A Comprehensive Analysis of Package Hallucinations by Code Generating LLMs" | USENIX Security 2025; arXiv 2406.10279v3, 2 Mar 2025 | T1 | L | https://arxiv.org/abs/2406.10279 |
| S3 | Zhong, Raghunathan, Carlini, "ImpossibleBench: Measuring LLMs' Propensity of Exploiting Test Cases" | arXiv 2510.20270v1, 23 Oct 2025 | T2 | L | https://arxiv.org/abs/2510.20270 |
| S4 | Olausson et al., "Is Self-Repair a Silver Bullet for Code Generation?" | ICLR 2024; arXiv 2306.09896v5, 2 Feb 2024 | T1 | L (abstract) | https://arxiv.org/abs/2306.09896 |
| S5 | Google Engineering Practices, "What to look for in a code review" | Undated; fetched 2026-11-13 | T1 | L | https://google.github.io/eng-practices/review/reviewer/looking-for.html |
| S6 | Google Engineering Practices, "Small CLs" | Undated; fetched 2026-11-13 | T1 | L | https://google.github.io/eng-practices/review/developer/small-cls.html |
| S7 | *Software Engineering at Google*, ch. 12 "Unit Testing" (Kuefler; ed. Manshreck) | Online edition; fetched 2026-11-13 | T1 | L | https://abseil.io/resources/swe-book/html/ch12.html |
| S8 | Nygard, "Documenting Architecture Decisions" | 15 Nov 2011 | T1 | L | https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions |
| S9 | Google SRE book, ch. 8 "Release Engineering" | Online edition; fetched 2026-11-13 | T1 | L | https://sre.google/sre-book/release-engineering/ |
| S10 | Anthropic, "Best practices for Claude Code" | Fetched 2026-11-13 | T3 | L | https://code.claude.com/docs/en/best-practices |
| S11 | Anthropic, Claude Code skills documentation | Fetched 2026-11-13 | T3 | L | https://code.claude.com/docs/en/skills |
| S12 | Anthropic, Claude Code hooks guide | Fetched 2026-11-13 | T3 | L | https://code.claude.com/docs/en/hooks-guide |
| S13 | NIST SP 800-218, Secure Software Development Framework (SSDF) | v1.1, Feb 2022 | T1 (standard) | L | https://csrc.nist.gov/pubs/sp/800/218/final |
| S14 | NIST SP 800-218A, SSDF Community Profile for Generative AI and Dual-Use Foundation Models | Final, 26 Jul 2024 | T1 (standard) | L | https://csrc.nist.gov/pubs/sp/800/218/a/final |
| S15 | ISTQB Certified Tester Foundation Level syllabus | v4.0.1, 15 Sep 2024 | T1 (standard) | L | https://www.istqb.org/certifications/certified-tester-foundation-level |
| S16 | SLSA specification | v1.2 | T1 (standard) | L | https://slsa.dev/spec/v1.2/ |
| S17 | Petrovic et al., "Does mutation testing improve testing practices?" | ICSE 2021; arXiv 2103.07189, 12 Mar 2021 | T1 | W (abstract) | https://arxiv.org/abs/2103.07189 |
| S18 | Rust tool documentation: Clippy lint groups; cargo-deny checks; cargo-mutants | Fetched 2026-11-13 | T3 | W | https://doc.rust-lang.org/clippy/lints.html · https://embarkstudios.github.io/cargo-deny/checks/index.html · https://mutants.rs/welcome.html |

**Supporting sources, read by a worker only and used with a caveat:** Cucumber Gherkin reference (https://cucumber.io/docs/gherkin/reference/); GitHub Spec Kit README (https://github.com/github/spec-kit); Huang et al., ICLR 2024, abstract (https://arxiv.org/abs/2310.01798); OWASP ASVS 5.0.0 landing page (https://asvs.owasp.org); Shape Up ch. 3 (https://basecamp.com/shapeup/1.2-chapter-03); Ogenrwot and Businge, MSR 2026 (https://arxiv.org/abs/2601.17581).

## 2. Study cards for every [Observed] source

| Source | Task | Participants | Tools / models, date | Measured | Result | Why it may not carry over to a junior solo Rust developer using 2026 agents |
|---|---|---|---|---|---|---|
| S1 METR | 246 real issues in mature open-source repositories, each randomly assigned to AI-allowed or AI-disallowed | 16 experienced developers working on their own repositories | Mainly Cursor Pro with Claude 3.5 and 3.7 Sonnet; Feb to Jun 2025 | **Speed** (time to complete) and forecasts of speed | 19% longer with AI allowed. Developers expected to be 24% faster and afterwards believed they had been 20% faster | Small sample of experts on code they know well. The authors say unfamiliar code and greenfield work may differ (untested). Tools are a generation old |
| S2 Spracklen et al. | Generate code from 2 prompt datasets and check whether referenced packages exist | No human participants. 16 code models, 576,000 samples | 2024 model set **[Older-generation tools]** | **Correctness / supply-chain security**: rate of non-existent package names | At least 5.2% for commercial models and 21.7% for open-source models; 205,474 unique invented names; 43% of invented names recurred on all 10 repeats | **Python and JavaScript only.** Rust appears only as a source of cross-language confusion. Single-shot generation, not an agent that builds and sees the error |
| S3 ImpossibleBench | Coding tasks altered so the tests contradict the specification; any pass means the tests were exploited | No human participants. Several frontier models | 2025 models, including GPT-5, o3, Claude models | **Correctness / integrity**: rate of passing impossible tasks | GPT-5 passed 76% of one SWE-bench variant and 2.9% of a LiveCodeBench variant. Methods: editing tests, overloading comparison, special-casing. A strict prompt, hidden tests, or a "flag for human" option each cut the rate sharply for some models | Tasks are built to be impossible, so rates do not predict everyday frequency. Large differences between models and prompts |
| S4 Olausson et al. | Repair failing generated programs on HumanEval and APPS | No human participants, plus a small human-feedback study | Code Llama, GPT-3.5, GPT-4 **[Older-generation tools]** | **Correctness** (pass rate, with the cost of repair counted) | Gains from self-repair are often modest. Feedback from a stronger model gives much larger gains | Benchmark puzzles and 2023 models. Abstract only was read |
| S17 Petrovic et al. | Mutation testing shown to developers during code review | Google developers (count not seen) | Google's internal system, to 2021 | **Test quality**: tests written, surviving mutants | Developers exposed to mutants wrote more tests; mutants are coupled with real faults | Abstract only, read by a worker. One company, observational |

Speed (S1), correctness (S2, S3, S4) and test quality (S17) are different measures. This report never combines them into one "productivity" claim, and it makes no speed claim for the reader.

## 3. System comparison

| System / source | Area | What it supports | Evidence (tier) | Portability | Maintenance cost | Fit at my scale | Verdict | Why |
|---|---|---|---|---|---|---|---|---|
| Given/When/Then acceptance criteria (ISTQB §4.5.2; Gherkin) | Requirements | Criteria as test conditions, with negative cases | T1 standard; consensus | High: plain text | Low | High | **Adopt** | Already in the reader's course notes; feeds the test plan directly |
| ADR (Nygard, S8) | Design | A one-to-two-page record per significant decision | T1 guidance; no outcome study | High: one markdown file | Low if only for significant decisions | High for Tier 3, wasteful for Tier 1 | **Adapt** | Use for Tier 3 and hard-to-reverse decisions only |
| Spec-driven tooling (Spec Kit) | Requirements, agents | A fixed sequence: principles, spec, plan, tasks, implement, check | T2 method only; **no outcome evidence opened** (C37) | Tied to specific agent tooling | Medium to high: several files per feature can drift | Low for a solo developer as packaged | **Leave** the tool; **adapt** the idea | Keep "write the criteria before the code, in the repo", at the smallest size the tier needs |
| Google code-review guidance (S5, S6) | Verification | What a reviewer looks for; small single-purpose changes | T1 guidance from one large organization | High | Low | High, except that it assumes a second person | **Adopt** | The quality bar is built on it |
| *Software Engineering at Google* ch. 12 (S7) | Testing | Test behaviors through the public API; avoid brittle tests | T1 guidance | High | Low | High | **Adopt** | Directly counters tests that restate the implementation |
| ISTQB CTFL (S15) | Testing | Named techniques, risk-based depth, exit criteria, independence levels | T1 consensus syllabus; free | High | Low | High as vocabulary; the certification itself is not needed | **Adopt** (as citable criteria) | Free, sectioned, and matches CSE 370 Unit 04 |
| Mutation testing (S17; cargo-mutants) | Testing | An objective signal of test sensitivity | T1 abstract (partly, C11); T3 tool docs | Rust tool exists | Medium: slow on large code | Good on changed files only | **Adapt** | Run on changed code for Tier 3; optional for Tier 2 |
| Google SRE release chapter (S9) | Release | Small frequent releases; define a release process at any size | T1 guidance | Ideas are portable; the tooling is not | Low for the ideas | Canary and progressive rollout need several instances | **Adapt** | Keep tagged releases and a last-good version. Mark fleet rollout **[Large-org only]** |
| NIST SSDF (S13) | Secure development | Named practices for review, testing, component vetting, protecting code, keeping records | T1 standard; consensus | High | Low as a crosswalk | High if used by risk, as §2 intends | **Adapt** | Cite the practice IDs the workflow already satisfies |
| SLSA (S16) | Supply chain | Levels for build provenance and source control | T1 specification | Depends on the CI host | Low at Build L1 to L2 | Build L1 to L2 is reachable. Source L4 (two-party review) is not, for one person | **Adapt** | State the level honestly; do not claim Source L4 |
| Claude Code best practices, skills, hooks (S10 to S12) | Agent workflow | Short instruction files; skills loaded by description; hooks for hard rules; reviewer in a fresh context | T3: how the product works | Skills use an open format; hooks are tool-specific | Medium: vendor docs change | High | **Adopt** for mechanics | Not evidence that any of it improves outcomes |
| "AI picks the lifecycle and skips stages" (outside summary) | Agent workflow | Nothing found | None | n/a | n/a | n/a | **Leave** | Contradicts Unit 07, and S3 shows models taking shortcuts when goals conflict |
| DORA metrics, DORA AI report, GitClear, Perry et al., Peng et al. | AI and delivery | Not opened in this run | n/a | n/a | n/a | n/a | **Leave for now** | No claim is made from them. See gaps |

## 4. D3b. Standards crosswalk

A standard records expert consensus or defines what an auditor checks. **It is not evidence that a practice improves outcomes.** "Read" means the lead read the clause text in this run. Standards that were not read have no clause cited.

| Standard (edition) | Body | Governs | Authority type | Access | Read? | Clause(s) to borrow | Handbook page it backs | Solo take | Large-org only? | AI relevance |
|---|---|---|---|---|---|---|---|---|---|---|
| NIST SP 800-218 SSDF (v1.1, 2022) | NIST | Secure development practices | Consensus guidance; not an audit | Free | **Yes** | §2 (risk-based, not a checklist); PO.3.3 (tools generate an audit trail); PW.4.4 (verify third-party components); PW.7 (review and/or analyze code); PW.8 (test executable code); PS.1 (protect code); PS.3 (archive releases); RV.1 (find vulnerabilities continuously) | Quality bar; Verify a change; Release | Your CI log and PR history already are the PO.3.3 audit trail | No. Role and training tasks in PO.2 are organizational | Indirect, through SP 800-218A |
| NIST SP 800-218A (2024) | NIST | SSDF profile for building AI models and AI systems | Consensus guidance | Free | **Yes** (audience, scope) | Scope statement: no distinction between human-written and AI-generated code; all code is evaluated before use | Quality bar; Working with an agent | Review agent-written code to the same bar as your own | The practices target AI model and system producers | Direct, but not for users of coding tools |
| ISTQB CTFL syllabus (v4.0.1, 2024) | ISTQB | Testing vocabulary and techniques | Consensus syllabus (certification body) | Free | **Yes** | §1.3 (principles); §1.5.3 (independence of testing); §4.2.1 and §4.2.2 (partitions, boundaries); §4.5.2 (acceptance criteria); §5.1.3 (exit criteria); §5.1.6 (test pyramid); §5.2.3 and §5.2.4 (product risk analysis and control) | Plan tests; Risk tiers; Verify a change | Names and section numbers for techniques you already learned in Unit 04 | No | None |
| SLSA (v1.2) | OpenSSF | Supply-chain integrity levels | Specification, self-assessed | Free | **Yes** | Build L1 (provenance exists), L2 (hosted build signs provenance), L3 (hardened); Source L1 to L4 (L4 is two-party review) | Release | Build on hosted CI and keep provenance: Build L1 to L2 | Build L3 and Source L4 need more than one person | None seen |
| OWASP ASVS (5.0.0, May 2025) | OWASP | Application security requirements | Community consensus | Free (CC BY-SA 4.0) | Edition only (worker) | **None cited.** Requirement text not read | (v2: security review checklist) | Use chapters as a review checklist for web and API code | No | None seen |
| SOC 2 / AICPA Trust Services Criteria (2017, revised points of focus 2022) | AICPA | Criteria an auditor reports against | Audit / attestation framework | Free with a login | **No** (login wall) | **None cited.** CC8.1 and CC7.x were not read | (audit-evidence map below, marked unverified) | Read the free PDF before relying on any mapping | The audit itself is organizational | Unknown |
| ISO/IEC/IEEE 29148 (requirements) | ISO/IEC/IEEE | Requirements engineering | Consensus standard | Paywalled | **No** | None cited | Would back "Define the change" | CSE 370 Unit 02's four criteria are used instead | No | Unknown |
| ISO/IEC 25010 (2023) (product quality) | ISO/IEC | Quality model | Consensus standard | Paywalled (HTTP 403) | **No**; characteristic names seen only on a third-party page | None cited | Could structure the quality bar | Not used in v1 | No | Unknown |
| ISO/IEC/IEEE 12207, 42010, 29119; IEEE 1016 | ISO/IEC/IEEE | Life cycle, architecture description, testing, design descriptions | Consensus standards | Paywalled | **No** | None cited | n/a | Not used in v1 | Process standards lean large-org | Unknown |
| SWEBOK v4; OWASP SAMM; CISA Secure by Design; OpenSSF Scorecard | Various | Body of knowledge; maturity model; principles; automated repo checks | Consensus / automated checks | Believed free | **No** | None cited | n/a | Candidates for v2 | SAMM leans large-org | Unknown |
| ISO/IEC 27001 Annex A; ISO/IEC 42001; NIST AI RMF and AI 600-1 | ISO/IEC; NIST | Security management; AI management; AI risk | Certifiable management systems; risk framework | ISO paywalled; NIST free | **No** | None cited | n/a | Not used in v1 | **[Large-org only]** (management systems) | Organizational AI governance |

### Audit-evidence map

SOC 2 is, as the plan describes it, an attestation report by an auditor against the AICPA Trust Services Criteria, not a certification or an engineering standard. **The criteria text was not read in this run**, so this map does not state what CC8.1 requires. It maps the everyday workflow to the record-keeping practices that *were* read (SSDF and SLSA). The SOC 2 column stays empty until the developer reads the free PDF.

| Expectation that was read | Clause | Everyday artifact that already provides it | Gap for a solo developer |
|---|---|---|---|
| Tools produce an audit trail of the development practices | SSDF PO.3.3 | CI run logs; PR history; commit history | None, if CI logs are kept |
| Code is reviewed and/or analyzed, and issues are recorded and triaged | SSDF PW.7.1, PW.7.2 | Clippy and review findings in the PR; issues opened for deferred items | The reviewer is the author. See below |
| Testing is scoped, designed, performed and its results documented | SSDF PW.8.2 | The test plan; the CI test log attached to the PR | None |
| Third-party components are verified against your requirements | SSDF PW.4.4 | `cargo deny` log; the dependency-approval line in the PR | None |
| Each release is archived with integrity and provenance data | SSDF PS.3.1; SLSA Build L1 to L2 | Tagged release; build provenance from hosted CI | Build L3 is out of reach without a hardened platform |
| Code is protected from unauthorized change | SSDF PS.1.1; SLSA Source L1 to L3 | Version control with protected branches | None |
| Changes to protected branches are reviewed by two trusted people | SLSA Source L4 | **Nothing.** | **Cannot be closed by one person** |
| Significant decisions are recorded | (no clause read) | ADRs | n/a |

**Separation of duties.** One person cannot be both author and independent approver, and no workflow change fixes that. An AI reviewer is not a second trusted person: SLSA Source L4 asks for two trusted *persons*, and ISTQB §1.5.3 describes independence between people. This report found no standard that accepts an AI review as a control, and it does not claim one. What a solo developer can credibly do is **[Proposed]**:
- keep the evidence that a second person would look at (criteria, test plan, CI logs, review findings, the decision);
- let time pass and re-read Tier 3 diffs in a fresh session before merging;
- bring in a human reviewer for Tier 3 changes when a customer or auditor requires it, and say plainly in any questionnaire that routine changes are self-approved.

### Where standards and practice disagree

- **Read every line, or focus on risk?** Google's guidance says to look at every line of human-written code you are asked to review (S5, "Every Line"). ISTQB §5.2.3 says risk decides depth. These fit together: risk decides how deep you go, not which lines you skip. The outside summary's "humans focus review on high-risk areas" is safe only in that sense.
- **Standards assume more than one person.** SLSA Source L4 and ISTQB's higher independence levels cannot be met alone. The handbook states the gap instead of papering over it.
- **Heavy documentation versus lean artifacts.** Nygard argues that large documents are not kept current (S8). The formal documentation standards (IEEE 1016, ISO/IEC/IEEE 29119) were not read, so this report cannot compare them fairly. A known criticism of 29119 from the context-driven testing community was recalled by a worker but not sourced (C45). It is recorded as an unverified lead only.

## 5. Disagreements and gaps

| Topic | What the sources say | Compared on | Effect on the handbook |
|---|---|---|---|
| Speed | Only S1 was opened: experienced developers were slower. Vendor studies in the plan's seed list report gains but were not opened | Participants (experts on their own code vs others), task (real issues vs bounded tasks), tool generation | The handbook makes **no speed claim** either way and tells the reader to measure their own tasks |
| Self-review | Vendor guidance recommends a fresh-context reviewer (S10). Research shows self-repair is weak and stronger or human feedback helps (S4). Nothing tests a same-model fresh-context reviewer (C34) | Vendor guidance vs benchmark study; 2023 models | Independent AI review is an inexpensive extra filter, **not a control**. Human review stays for Tier 3 |
| Agent test-gaming | S3 measures it on impossible tasks. No field study of everyday frequency was opened | Benchmark vs real work | Enough to justify protecting tests and planning them from the spec. Not enough to state how often it happens |

**Gaps that could change the handbook design**

| Gap | Could change | How much |
|---|---|---|
| No outcome evidence for spec-first agent workflows (C37) | How much specification the workflow asks for | Low: the handbook already asks for the smallest artifact per tier |
| No measured evidence for most agent-specific defects (C31): duplication, scope creep, mirrored tests, wrong comments | Which items the quality bar treats as agent-specific | Medium: they stay in the bar as review items, labeled unmeasured |
| SOC 2 criteria and the ISO/IEEE standards were not read (C42, C44) | The crosswalk's coverage and the audit-evidence map | Medium for RQ6: the crosswalk is narrower than the plan intended. Low for the workflow |
| No evidence on whether a same-model fresh-context review beats self-review (C34) | Whether the second skill (review) is worth building | Medium: the reader's own evaluation has to answer it |
| No field data on how often the first skill beats a plain request | The first-skill choice | High, and it is exactly what the D5 evaluation measures |
| Rust-specific package hallucination rate (C26) | How strict the dependency rule is | Low: the rule is cheap either way |
