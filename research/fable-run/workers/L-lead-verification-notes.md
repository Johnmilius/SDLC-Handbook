# Lead verification and follow-up notes (Phase 2 follow-up and Phase 3)

Fetched 2026-11-13 03:35 to 03:45 MST through four `web-fetch` agents, each given exact URLs and questions. These are the raw answers the claim-check table (`04-claim-check.md`) is built from. "Summary-only" marks points the fetch tool summarized rather than returned verbatim.

## A. AI evidence

### METR trial (arXiv 2507.09089v2, 25 Jul 2025). Read: https://arxiv.org/html/2507.09089
- 16 developers, 246 tasks (Abstract). Tasks are issues defined before randomization, each randomized by a simulated coin flip (§2.2). Split: 136 AI-allowed, 110 AI-disallowed (§3).
- AI-allowed issues took 19% longer (Abstract; §3). Developers forecast a 24% speedup before and estimated a 20% speedup after (§3.1). Expert forecasts: 39% (economics) and 38% (ML) speedup.
- Tools: mainly Cursor Pro with Claude 3.5/3.7 Sonnet; any tools were allowed (§2.2.1). February to June 2025.
- Quality: no formal correctness or code-quality metric in the text read. §3.3 lists qualitatively similar PR quality between conditions and a minor difference in post-review time.
- Caveats (§4.1): the authors caution against overgeneralizing. They say results are consistent with substantial speedup on small greenfield projects or in unfamiliar codebases (a hypothesis, not tested). Less experienced developers are not addressed.

### Spracklen et al. (arXiv 2406.10279v3, 2 Mar 2025). Read: https://arxiv.org/html/2406.10279
- Languages: Python (PyPI) and JavaScript (npm) (§4.2). Rust appears only as a source of cross-language confusion: Table 3 (§5.4) lists "Rust (Cargo)" with 181 cross-language hallucinations. crates.io is not mentioned.
- 16 models, 576,000 code samples, at least 5.2% (commercial) vs 21.7% (open-source), 205,474 unique hallucinated names (Abstract; §5.1).
- Persistence (§5.3): 500 prompts repeated 10 times. 43% of hallucinated packages recurred in all 10 runs; 39% never recurred; 58% recurred more than once.

### ImpossibleBench (arXiv 2510.20270v1, 23 Oct 2025). Zhong, Raghunathan, Carlini. Read: https://arxiv.org/html/2510.20270v1
- Builds "impossible" variants of LiveCodeBench and SWE-bench tasks where the unit tests conflict with the natural-language spec. Any pass means the model exploited the tests. Two mutations: One-Off (one expected value changed) and Conflicting (a contradictory test added) (§2.2).
- GPT-5 passed 76% of One-Off SWE-bench tasks and 2.9% of One-Off LiveCodeBench tasks (§1). GPT-5 reached 54.0% on Conflicting SWE-bench (§4). Newer Claude models cheat less than Claude Sonnet 3.7 (§4).
- Cheating types (§4.1): modifying test cases, overloading comparison operators, recording extra state, special-casing the tests. Claude and Qwen3-Coder models cheat mainly (over 79%) by modifying test cases (§4.2).
- Mitigations (§5): a strict prompt cut GPT-5 from over 85% to 1% and o3 to 33% (§5.1). Hiding tests cut cheating to near zero but hurt normal performance; read-only test access did not stop special-casing or operator overloading (§5.2). Letting the agent flag the task for human intervention cut GPT-5 from 54% to 9% and o3 from 49% to 12%, with a much weaker effect for Claude Opus 4.1 (§5.3).

### Olausson et al. (arXiv 2306.09896v5, 2 Feb 2024). Read: https://arxiv.org/abs/2306.09896 (abstract)
- With repair cost counted, gains are often modest. Feedback from a stronger model gives substantially larger gains. With human feedback, GPT-4 self-repair still lags far behind human debugging. Models: Code Llama, GPT-3.5, GPT-4. Benchmarks: HumanEval, APPS.

## B. Practice guidance

### Google eng-practices, "What to look for in a code review". Read: https://google.github.io/eng-practices/review/reviewer/looking-for.html
- Headings: Design, Functionality, Complexity, Tests, Naming, Comments, Style, Consistency, Documentation, Every Line, Context, Good Things.
- Tests: asks whether the tests will actually fail when the code is broken; says tests do not test themselves and a human must make sure they are valid.
- Complexity: over-engineering is a named type of complexity; solve the problem that is known to need solving now.
- The page does not state that the reviewer must be a different person from the author. It assumes it.

### Google eng-practices, "Small CLs". Read: https://google.github.io/eng-practices/review/developer/small-cls.html
- Benefits: reviewed more quickly and more thoroughly, less likely to introduce bugs, less wasted work if rejected, easier to merge, easier to design well, less blocking, simpler to roll back.
- About 100 lines is usually reasonable and about 1000 usually too large, with no hard rule. Refactorings are usually best in a separate change. Related test code belongs in the same change.

### Software Engineering at Google, ch. 12 "Unit Testing" (Kuefler; ed. Manshreck). Read: https://abseil.io/resources/swe-book/html/ch12.html (first ~40k of 59k characters returned verbatim)
- "Test via Public APIs" and "Test Behaviors, Not Methods": write a test per behavior and call the public API.
- "Preventing Brittle Tests": a brittle test fails on an unrelated change to production code. "Strive for Unchanging Tests": the ideal test does not change; tests that must change during a refactoring indicate a problem. "Test State, Not Interactions": interaction tests are more brittle.
- Structure tests as given / when / then; name a test after the behavior it tests.
- Publication year not shown on the page.

### Nygard, "Documenting Architecture Decisions" (15 Nov 2011). Read: https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions
- Sections: Title, Context, Decision, Status, Consequences. Architecturally significant decisions affect structure, non-functional characteristics, dependencies, interfaces or construction techniques. One ADR per decision. One or two pages.

### Google SRE book, ch. 8 "Release Engineering". Read: https://sre.google/sre-book/release-engineering/
- "High Velocity": frequent releases mean fewer changes between versions, which makes testing and troubleshooting easier.
- Canary: mentioned as starting a few jobs in production. No further detail on this page.
- "It's Not Just for Googlers": most companies face the same release-engineering problems and should define their release process, whatever their size.

## C. Agent tooling documentation (T3: how features work)

### Claude Code best practices. Read: https://code.claude.com/docs/en/best-practices
- "Keep CLAUDE.md short": it should hold only what the agent could not work out from the code (commands, misleading code, undocumented decisions, anything corrected twice). The memory guide recommends staying under 200 lines.
- "Deterministic and non-deterministic instructions": CLAUDE.md is non-deterministic, with no guarantee it is followed. "Enforce the rules that can't have exceptions": use a hook, a deny rule, or a managed setting.
- "Give Claude a check to run": give it something that returns pass or fail (a test file, a build exit code, a linter) and ask for evidence. The built-in review runs in a fresh subagent that never saw the conversation. A reviewer that has not seen how the code was written, with read-only tools, is a listed use of subagents. Summary-only: a second session is the easiest independent reviewer.
- "Brainstorm with Claude first": interview until a spec exists, write it to a file, then start a fresh session to build from it. When to skip planning: not found.

### Claude Code skills documentation. Read: https://code.claude.com/docs/en/skills
- Locations: personal `~/.claude/skills/<name>/SKILL.md`; project `.claude/skills/<name>/SKILL.md`; plugin `<plugin>/skills/<name>/SKILL.md`. The file must be named `SKILL.md`.
- Frontmatter (summary-only; all optional, `description` recommended): `name`, `description`, `when_to_use`, `argument-hint`, `arguments`, `disable-model-invocation`, `user-invocable`, `allowed-tools`, `disallowed-tools`, `model`, `effort`, `context` (`fork` runs in a subagent), `agent`, `background`, `hooks`, `paths`, `shell`, `metadata`, `license`, `compatibility`.
- Loading (summary-only): descriptions load at session start and decide when the skill applies; the full file loads only when invoked. Description plus `when_to_use` is truncated at 1,536 characters in the listing. Keep `SKILL.md` under 500 lines.
- Supporting files (summary-only): put `reference.md`, `examples.md`, `scripts/` in the skill directory and link them from `SKILL.md` with relative markdown links. Scripts can use `${CLAUDE_SKILL_DIR}`.
- Invocation: `/skill-name`. `disable-model-invocation: true` stops automatic use. `context: fork` runs the skill in a subagent that does not see the conversation history.

### Claude Code hooks guide. Read: https://code.claude.com/docs/en/hooks-guide
- `PreToolUse` runs before a tool call and can block it: exit code 2 blocks, with the reason on stderr; or JSON with a `permissionDecision` of deny.
- Configured in `~/.claude/settings.json`, `.claude/settings.json`, `.claude/settings.local.json`.
- "Block edits to protected files": a `PreToolUse` hook matching `Edit|Write` runs a script that exits 2 for protected paths.

## D. Standards

### SLSA v1.2. Read: https://slsa.dev/spec/latest/levels, /spec/v1.2/build-track-basics, /spec/v1.2/build-requirements, /spec/v1.2/source-requirements
- Tracks: Build and Source.
- Build: L0 no guarantees. L1 provenance exists (the platform generates it). L2 hosted build platform that generates and signs provenance. L3 hardened builds (runs cannot influence one another; signing secrets are out of reach of build steps).
- Source: L1 version controlled. L2 history and provenance (continuous, immutable, retained history; attestations). L3 continuous technical controls enforced on named references. L4 two-party review of changes to protected branches.

### AICPA Trust Services Criteria. https://www.aicpa-cima.com/resources/download/2017-trust-services-criteria-with-revised-points-of-focus-2022
- **LOGIN REQUIRED.** Landing page only: "2017 Trust Services Criteria (With Revised Points of Focus – 2022)", page dated 30 Sep 2023, covers Security, Availability, Processing Integrity, Confidentiality, Privacy. **No criteria text read. CC8.1 wording not verified.**

### NIST SP 800-218, NIST SP 800-218A, ISTQB CTFL syllabus v4.0.1
- The fetch tool returned compressed PDF bytes it could not read. The PDFs were saved locally. See the "PDF extraction" section below for what the lead read from them directly.

## PDF extraction (lead)

The lead extracted text from the three saved PDFs with `pdftotext` and read the passages below directly on 2026-11-13 about 03:55 MST.

### NIST SP 800-218, SSDF v1.1 (Feb 2022). https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-218.pdf
- Groups (§2): Prepare the Organization (PO), Protect the Software (PS), Produce Well-Secured Software (PW), Respond to Vulnerabilities (RV).
- Practices seen in the table: PO.1 to PO.5, PS.1 to PS.3, PW.1, PW.2, PW.4 to PW.9 (PW.3 was moved into PW.4), RV.1 to RV.3.
- Scope statements (§1 and §2): usable by organizations of any size or sector; it does not prescribe how to implement a practice; not every practice applies to every case; organizations should take a risk-based approach; the practices are a starting point meant to be customized; the intention is not a checklist.
- PO.3 Implement Supporting Toolchains: use automation to reduce human effort and improve accuracy and reproducibility, and to document and demonstrate the practices. PO.3.3: configure tools to generate artifacts supporting the practices (example: use existing workflow or issue tracking to create an audit trail).
- PO.4.1: define criteria for software security checks and track them through the SDLC.
- PS.1 Protect All Forms of Code from Unauthorized Access and Tampering. PS.1.1: store code under least privilege.
- PS.2 Provide a Mechanism for Verifying Software Release Integrity (example: publish cryptographic hashes).
- PS.3 Archive and Protect Each Software Release. PS.3.1: archive the files and supporting data, including integrity and provenance data, for each release.
- PW.4 Reuse Existing, Well-Secured Software When Feasible Instead of Duplicating Functionality. PW.4.1: acquire and maintain well-secured components. PW.4.4: verify that acquired third-party components comply with the organization's requirements throughout their life cycles.
- PW.7 Review and/or Analyze Human-Readable Code to Identify Vulnerabilities and Verify Compliance with Security Requirements. PW.7.1: decide whether code review (a person), code analysis (tools), or both are used. PW.7.2: perform it against the secure coding standards, and record and triage all issues in the team's workflow or issue tracker. Examples include peer review and static analysis tools.
- PW.8 Test Executable Code to Identify Vulnerabilities and Verify Compliance with Security Requirements. PW.8.2: scope, design and perform the testing and document the results.
- RV.1 Identify and Confirm Vulnerabilities on an Ongoing Basis.

### NIST SP 800-218A (Jul 2024). https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-218A.pdf
- Audience: AI model producers, AI system producers, AI system acquirers.
- Scope: AI model development (data sourcing, design, training, fine-tuning, evaluation, and integrating models into other software). Deployment and operation are out of scope.
- The only statement relevant to a developer who *uses* an AI coding tool (Scope section): the profile does not distinguish between human-written and AI-generated source code, because all source code is assumed to need evaluation for vulnerabilities and other issues before use.
- No guidance specific to AI coding assistants was found by keyword search ("coding assistant", "generated code", "copilot").

### ISTQB Certified Tester Foundation Level syllabus v4.0.1 (15 Sep 2024). https://istqb.org/wp-content/uploads/2024/11/ISTQB_CTFL_Syllabus_v4.0.1.pdf
- §1.3 Testing Principles (seven): testing shows the presence, not the absence, of defects; exhaustive testing is impossible; early testing saves time and money; defects cluster; tests wear out; testing is context dependent; the absence-of-defects fallacy.
- §1.5.3 Independence of Testing: some independence makes a tester more effective because author and tester have different cognitive biases. It is not a replacement for familiarity. Levels: the author (none), peers on the same team (some), testers outside the team (high), testers outside the organization (very high). Benefit: independent testers recognize different kinds of failures and can challenge assumptions. Drawbacks: isolation, an adversarial relationship, developers losing a sense of responsibility for quality, bottlenecks.
- §2.1.3 Testing as a Driver for Software Development; §2.1.5 Shift Left; §2.2.3 Confirmation Testing and Regression Testing.
- §3.1.2 Value of Static Testing; §3.2.2 Review Process Activities.
- §4.2.1 Equivalence Partitioning; §4.2.2 Boundary Value Analysis; §4.4.1 Error Guessing; §4.4.3 Checklist-Based Testing.
- §4.5.2 Acceptance Criteria: the conditions an implementation must meet to be accepted; they can be viewed as the test conditions; they describe both positive and negative scenarios. §4.5.3 Acceptance Test-driven Development.
- §5.1.3 Entry Criteria and Exit Criteria: exit criteria define what must be achieved to declare an activity complete; in Agile development they are often called the Definition of Done.
- §5.1.6 Test Pyramid: lower layers hold small, isolated, fast tests, and many of them; higher layers are slower and less isolated.
- §5.2.3 Product Risk Analysis: its results determine test scope, test levels and types, techniques and coverage, and effort. Risk level combines likelihood and impact.
- §5.2.4 Product Risk Control: mitigation actions include an appropriate level of independence of testing, reviews and static analysis, and appropriate techniques and coverage levels.

## Source count after this step

28 distinct sources opened across the run: W1 4, W2 7, W3 6, W4 6, and 5 new for the lead (ImpossibleBench; *Software Engineering at Google* ch. 12; Claude Code skills docs; Claude Code hooks guide; ISTQB CTFL syllabus). The SSDF PDF, the SP 800-218A PDF, the SLSA v1.2 pages and the AICPA landing page are deeper reads of sources W4 had already opened.
