# Phase 3 claim-check table

Written 2026-11-13 04:05 MST. Every row below was checked by the lead against the source itself (through a `web-fetch` agent with exact questions, or by reading extracted PDF text). Raw answers are in [workers/L-lead-verification-notes.md](workers/L-lead-verification-notes.md).

**How to read "Supports it?"**
- **yes**: the source says this, at the location given.
- **partly**: the source supports a narrower claim. The note says what the report may say instead.
- **no**: the source does not support it, or was not read. The claim is dropped or labeled as unverified.

Build rule (from the build plan): use only *yes* rows as stated; a *partly* row must carry its caveat; *no* rows stay out.

## A. Claims behind the major recommendations

| # | Claim in report | Source | Location | Supports it? | Note |
|---|---|---|---|---|---|
| **Handbook model** | | | | | |
| C1 | Short decision records stay current; large documents do not get kept up to date. An ADR is one or two pages with Title, Context, Decision, Status, Consequences | Nygard, "Documenting Architecture Decisions" (2011) | "Decision" section | yes | One practitioner's guidance, not a study |
| C2 | Agent instruction files should be short and hold only what the agent cannot infer; they are not guaranteed to be followed | Claude Code best practices (fetched 2026-11-13) | "Keep CLAUDE.md short"; "Deterministic and non-deterministic instructions" | yes | T3: describes how the product behaves |
| C3 | A skill's description is loaded at session start and its full file only when invoked; supporting files live in the skill directory and are linked relatively | Claude Code skills docs (fetched 2026-11-13) | "Choose where skills load"; "Frontmatter reference"; "Add supporting files" | yes | Frontmatter and loading details came back as a tool summary, not verbatim. Re-check at build step B5 |
| **First skill (test planning)** | | | | | |
| C4 | Given tests that conflict with the spec, current models often make the tests pass anyway, including by editing the tests or special-casing them | Zhong, Raghunathan, Carlini, "ImpossibleBench" (arXiv 2510.20270v1, Oct 2025) | §1, §4, §4.1, §4.2 | yes | Benchmark tasks built to be impossible; rates vary widely by model and prompt (GPT-5: 76% on one variant, 2.9% on another) |
| C5 | A strict prompt, hiding the tests, and letting the agent flag a conflict for a human each reduce test exploitation; read-only tests alone do not stop special-casing | ImpossibleBench | §5.1, §5.2, §5.3 | yes | Effect sizes are model-specific |
| C6 | A reviewer should ask whether the tests would actually fail if the code were broken; tests do not validate themselves and a human must check them | Google eng-practices, "What to look for in a code review" | "Tests" | yes | |
| C7 | Test behaviors through the public API; a test that must change when the implementation is refactored signals a problem | *Software Engineering at Google*, ch. 12 | "Test via Public APIs"; "Test Behaviors, Not Methods"; "Strive for Unchanging Tests" | yes | About 40k of 59k characters were returned verbatim |
| C8 | Acceptance criteria can be treated as the test conditions and should cover negative as well as positive scenarios | ISTQB CTFL syllabus v4.0.1 | §4.5.2 | yes | Consensus syllabus, not outcome evidence |
| C9 | Equivalence partitioning and boundary value analysis are standard black-box test techniques | ISTQB CTFL v4.0.1 | §4.2.1, §4.2.2 | yes | Section titles and body confirmed present; technique detail matches the CSE 370 Unit 04 notes |
| C10 | Model self-repair of code gives modest gains; the quality of feedback is the limit, and feedback from a stronger model helps much more | Olausson et al. (ICLR 2024; arXiv 2306.09896v5) | Abstract | yes | [Older-generation tools]. Abstract only |
| C11 | Mutation testing leads developers to write more tests, and mutants are coupled with real faults | Petrovic et al. (ICSE 2021; arXiv 2103.07189) | Abstract | partly | Abstract only, read by W2, not re-opened by the lead. Report may say "a Google study reports", with no effect size |
| **Tiers** | | | | | |
| C12 | Product risk (likelihood and impact) should determine test scope, levels, techniques, coverage and effort | ISTQB CTFL v4.0.1 | §5.2.3 | yes | Supports scaling by risk, not by change size |
| C13 | Secure-development practices should be applied with a risk-based approach and customized; SSDF is not a checklist and applies at any organization size | NIST SP 800-218 v1.1 | §1, §2 | yes | |
| C14 | A methodology may change the timing or style of a phase, never whether it happens | CSE 370 Unit 07 notes (local) | "Methodology vs SDLC" | yes | Course notes, not an external source |
| C15 | Exit criteria ("Definition of Done") define what must be achieved to call an activity complete | ISTQB CTFL v4.0.1 | §5.1.3 | yes | Backs the "done when" pattern |
| C16 | Specific escalation triggers (auth, migrations, money, public API, `unsafe`, new dependencies) | none | n/a | no | No source lists these. They are **[Proposed]** and must be labeled so |
| **Delegation** | | | | | |
| C17 | Experienced developers were 19% slower with AI tools on their own repos while believing they were about 20% faster | METR (arXiv 2507.09089v2, Jul 2025) | Abstract; §3; §3.1 | yes | Speed only. 16 developers. Authors caution against generalizing; they say unfamiliar codebases may differ (untested) |
| C18 | Rules with no exceptions belong in a hook, a deny rule or a managed setting, because instruction files are not deterministic | Claude Code best practices | "Enforce the rules that can't have exceptions" | yes | T3 |
| C19 | A `PreToolUse` hook can block a tool call (exit code 2), and the docs show blocking edits to protected files | Claude Code hooks guide | "How hooks work"; "Block edits to protected files" | yes | T3 |
| C20 | Give the agent a pass/fail check it can run, and ask for evidence | Claude Code best practices | "Give Claude a check to run" | yes | T3 |
| C21 | A delegation ladder (levels of autonomy earned by evidence) | none | n/a | no | No source defines one. **[Proposed]**, built from C4, C5, C17 to C20 |
| **Quality bar** | | | | | |
| C22 | A reviewer looks at design, functionality, complexity, tests, naming, comments, style, consistency, documentation, every line, and context | Google eng-practices, "What to look for" | Section headings | yes | |
| C23 | Over-engineering is a named review concern; solve the problem known to exist now | Google eng-practices | "Complexity" | yes | |
| C24 | Keep changes small and single-purpose; put refactors in a separate change; include tests in the same change | Google eng-practices, "Small CLs" | "Why Write Small CLs?"; "What is Small?"; "Separate Out Refactorings" | yes | Line counts are rules of thumb |
| C25 | Code-generating models invent package names at measurable rates (at least 5.2% commercial, 21.7% open-source models), and 43% of invented names recur on every repeat of the same prompt | Spracklen et al. (USENIX Security 2025; arXiv 2406.10279v3) | Abstract; §5.1; §5.3 | yes | [Older-generation tools]. **Python and JavaScript only.** No rate for Rust |
| C26 | Package hallucination is measured for Rust/crates.io | Spracklen et al. | §5.4, Table 3 | no | Rust appears only as a source of cross-language name confusion. The report must say the Rust rate is unmeasured |
| C27 | All source code, human-written or AI-generated, should be evaluated for vulnerabilities and other issues before use | NIST SP 800-218A | Scope | yes | The profile targets AI model and system producers, not users of coding tools. This one sentence is the usable part |
| C28 | Review and/or analyze code (PW.7), test executable code (PW.8), and verify third-party components (PW.4.4) are named practices | NIST SP 800-218 v1.1 | Table, PW.4, PW.7, PW.8 | yes | |
| C29 | Clippy's `restriction` group should not be enabled wholesale; pick individual lints such as `unwrap_used` | Clippy docs | "Lint groups" | partly | Read by W2 only, not re-opened by the lead. T3. Acceptable for how the tool works |
| C30 | `cargo deny check` covers advisories, bans, licenses and sources | cargo-deny docs | "Checks" | partly | Read by W2 only. T3 |
| C31 | Agent-written code more often has duplicated code, mirrored tests, scope creep, or wrong comments | none opened | n/a | no | **Anecdote in this run.** No measured source was opened. Label as unmeasured |

## B. Other claims used in the report

| # | Claim in report | Source | Location | Supports it? | Note |
|---|---|---|---|---|---|
| C32 | Some independence makes testing more effective; levels run from the author to outside the organization; it has drawbacks | ISTQB CTFL v4.0.1 | §1.5.3 | yes | About human testers. Says nothing about AI reviewers |
| C33 | A reviewer in a fresh context that has not seen how the code was written is a recommended setup | Claude Code best practices | "Give Claude a check to run"; "Know what subagents cost" | yes | Vendor guidance (T3), not a measured result |
| C34 | A fresh-context reviewer using the *same* model catches more than the author reviewing itself | none | n/a | no | Not tested by any source read. H8 can only be "modified" |
| C35 | Given/When/Then: context, action, observable outcome; 3 to 5 steps per example | Cucumber Gherkin reference | Keyword sections | partly | Read by W1 (Haiku) only, not re-opened by the lead |
| C36 | Spec-driven tooling prescribes constitution, specify, plan, tasks, implement, converge | GitHub Spec Kit README | README | partly | Read by W1 only. Method description, no outcome evidence |
| C37 | Spec-first agent workflows have measured benefit | none | n/a | no | No outcome study opened. Label as unmeasured |
| C38 | Frequent releases mean fewer changes per release and easier troubleshooting; release process matters at any company size | Google SRE book, ch. 8 | "High Velocity"; "It's Not Just for Googlers" | yes | |
| C39 | Rollout should be matched to risk; canary criteria | Google SRE book, ch. 8 | n/a | partly | W2 reported "matched to risk". The lead's check confirmed only a brief canary mention. Report may cite the canary mention, not criteria |
| C40 | SLSA v1.2 has a Build track (L0 to L3) and a Source track (L1 to L4); Source L4 requires two-party review | slsa.dev v1.2 | "Tracks"; build-track-basics; source-requirements | yes | |
| C41 | SSDF PO.3.3 asks for tools to generate artifacts (an audit trail); PW.7.2 and PW.8.2 ask for issues and results to be recorded; PS.3.1 asks for releases to be archived | NIST SP 800-218 v1.1 | Table | yes | Basis of the audit-evidence map |
| C42 | SOC 2 CC8.1 requires changes to be authorized, tested, approved and documented; separation of duties is expected | AICPA Trust Services Criteria | n/a | **no** | **Criteria text not read (login wall).** The report must not paraphrase CC8.1. The audit-evidence map is built on SSDF and SLSA text and marks SOC 2 as unverified |
| C43 | OWASP ASVS current version is 5.0.0 (May 2025), free | asvs.owasp.org | Landing page | partly | Read by W4 only. Edition and license only; no requirement text read |
| C44 | ISO/IEC 25010:2023 characteristics; ISO/IEC/IEEE 29148 requirement characteristics; 12207, 42010, 29119, IEEE 1016, ISO/IEC 27001, 42001 clauses | none | n/a | **no** | Paywalled or blocked (HTTP 403). No clause may be cited. Listed in D3b as "not read" |
| C45 | Professional criticism of ISO/IEC/IEEE 29119 | none | n/a | no | Not researched. Mentioned only as an unverified lead |
| C46 | Characterization tests pin existing behavior before changing legacy code | Feathers, *Working Effectively with Legacy Code* (2004) | n/a | no | Book not opened. May be named as the origin of the term, marked "not opened in this run" |
| C47 | An "Anthropic AI-native playbook" exists | none | n/a | no | Not found. Dropped |

## Result for the gate

All five major recommendations rest on at least one *yes* row, and the parts that have no source are labeled [Proposed] rather than presented as sourced:

| Major recommendation | *yes* rows it rests on | Parts that are [Proposed] |
|---|---|---|
| Handbook model (short guide plus skills) | C1, C2, C3 | The page count and layout |
| First skill (test planning) | C4, C5, C6, C7, C8, C9, C10 | The weighted scores |
| Tiers (depth scales with risk, no phase skipped) | C12, C13, C14, C15 | The three tier names and the escalation triggers (C16) |
| Delegation (earned by evidence; hard rules in hooks) | C4, C5, C17, C18, C19, C20 | The ladder itself (C21) |
| Quality bar | C6, C22, C23, C24, C25, C27, C28 | Which lints to pick; the agent-defect list beyond C4 and C25 (C31) |
