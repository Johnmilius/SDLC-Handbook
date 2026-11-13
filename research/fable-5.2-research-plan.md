# Fable 5.2 Research Plan: SDLC Handbook for AI-Assisted Development

**Version:** 1.3 (2026-11-13)
**Changes in 1.3:**
- Everything is now saved to `research/fable-run/` as the run goes (§12), so a usage cap or crash loses at most one step.
- Any new session can resume from the saved state.
- When research finishes and passes its gate, the run continues into the separate build plan, `research/fable-5.2-build-plan.md` (§13).


**Changes in 1.2:**
- Added RQ6 on formal standards and compliance frameworks (IEEE/ISO, NIST, SOC 2, OWASP, SLSA).
- Added a fourth worker (W4) to cover them.
- Added deliverable D3b, a crosswalk from each standard to what a solo developer can borrow.
- Added a "Standard" evidence label.
- Added hypothesis H11.


**Changes in 1.1:**
- Added the claims from an outside "AI-native SDLC" summary as things to test (§3b).
- Added RQ5 on the code quality of agent-written code.
- Added an independent-review skill as a candidate.
- Added an agent-diff review case with AI-specific planted defects.
- Raised the source budget a little to cover the new scope.

**How to run:**
1. Open a new Claude Code session on **Fable 5.2** (`claude-fable-5-2`), with its working directory set to `C:\Users\jwmil\code\active\SDLC-handbook`.
2. Send: *"Read `research/fable-5.2-research-plan.md` and execute it."*
3. Research writes only to `research/fable-run/` and saves after every step (§12). If it passes its gate, it continues into `research/fable-5.2-build-plan.md` (§13).
4. **If you hit a usage limit:** start a new Fable 5.2 session and send the same message. It resumes from `research/fable-run/00-run-state.md`.

---

## 0. Your role and mission

You are the **lead researcher and orchestrator**. You are a senior software engineering researcher who also practices. Your job is to **design a personal SDLC handbook for AI-assisted software development**, grounded in evidence, and the first reusable agent skill that applies it.

Use your own judgment for framing the questions, resolving conflicts between sources, checking evidence, and the final synthesis. Delegate bounded, independent research to cheaper models.

By the end of the run, the reader must be able to make **three decisions**:
1. **What the handbook contains** and how it's organized.
2. **Which everyday workflow to adopt first.**
3. **What to build and evaluate as the first skill.**

Every section of your report should help with at least one of these. If something doesn't, cut it.

---

## 1. Who the reader is

- A **junior software engineer** who works solo or on a small team maintaining real software. Main language: Rust. Some TypeScript or Python.
- Has finished BYU-I **CSE 370** (SDLC, requirements, design, testing, release, project management). Knows the textbook vocabulary but not current practitioner depth.
- Uses coding agents (Claude Code) **interactively**, and sometimes hands them **bounded tasks to run on their own**.
- Wants to **get better at making engineering decisions**, not hand them off. A skill's output should teach: *finding → evidence → proposed action → why it matters for this change*.
- Works on **new projects and existing codebases**. Both must be covered.

---

## 2. Boundaries (these apply to you and every worker)

| Rule | Detail |
|---|---|
| **Read-only research** | During research, write **only** inside `SDLC-handbook/research/fable-run/` (§12). Don't modify anything else, create or install skills, or write CI. Writing the handbook happens only in the build plan (§13). |
| **Handbook folder** | `C:\Users\jwmil\code\active\SDLC-handbook`. Read `cse370-prepare/README.md` and the unit notes you need (especially `unit02`, `unit03`, `unit04`, `unit05`, `unit07`). |
| **Don't touch** | `research/gemini-prompts.md`. Don't open, edit or run it. It's for a later session. |
| **Eloksen (private, read-only)** | You may read `C:\Users\jwmil\code\active\eloksen\windows-rust\docs\specs\build-order.md` and `...\testing.md` as examples of dependency ordering, "done when" checks, and verification that needs real hardware. Read nothing else in that repo. |
| **Privacy** | Never put Eloksen names, product details, code, or the reader's identity into searches, worker briefs or external requests. Workers get **no** local file access. Use **synthetic examples** only in the report. |
| **No local access?** | Continue with public research and mark each affected conclusion **[Needs validation against local notes]**. |

---

## 3. Starting hypotheses (test these, don't assume them)

An earlier review proposed the ideas below. Treat them as **hypotheses**. For each one, report: *supported / modified / rejected*, plus the evidence.

| # | Hypothesis | What would overturn it |
|---|---|---|
| H1 | The handbook should be a **short field guide plus reusable agent skills**, not a comprehensive reference. | Evidence that short guides go unused, or that skills without deep reference pages give shallow or ungrounded output. |
| H2 | The **first skill should be test planning / TDD**. For a spec or diff, it says what could break, what would catch it, why the check belongs at that layer, and what still needs verification. | PR verification or release readiness turns out more frequent, cheaper to build, or easier to verify. Or AI-assisted testing evidence shows agents already do this well without a skill. |
| H3 | Folder layout: `cse370-prepare/` stays as source material. A root `README.md` routes by task. `handbook/` holds short reference pages, `skills/` holds agent instructions plus worked examples, and project-specific decisions live **in each project repo**. | Portability or maintenance problems, e.g., skills that can't find handbook pages once copied into another repo. |
| H4 | Every skill recommendation **links to a handbook section**, and every section **links to its source**. | Evidence that citation chains add maintenance cost without improving decisions, or that agents fabricate the links. |
| H5 | Effort should scale by **risk tier**: small bug fix < normal feature < high-risk change. Each tier has a defined minimum artifact and evidence. | A better-supported way to scale effort, e.g., by reversibility or blast radius rather than size. |
| H6 | Patterns worth carrying over from the Eloksen specs: a **"done when" check per step**, **dependencies and risk per step**, and separating **checks CI can run** from **checks that need real hardware or people**. | Sources show these patterns don't generalize beyond hardware-heavy projects. |
| H7 | **Run verification in a fixed order:** deterministic checks first (compiler, `clippy -D warnings`, tests, `cargo-deny`/`cargo-audit`, secret scan), then AI review, then human review aimed at the riskiest areas. AI review of code that fails deterministic checks wastes the reviewer's attention. | Evidence that AI review catches different defects early enough to justify running it in parallel instead of after the deterministic checks. |
| H8 | **The reviewer should be independent of the author.** The reviewing agent starts in a fresh context and gets the spec, the diff and the test results, but **not** the author's chain of reasoning. Independence comes from what the reviewer is given, not just from using "another agent". | Evidence that same-model, fresh-context review is no better than the author reviewing itself, or that only a human or a different model family helps. |
| H9 | **Hard rules belong in hooks and CI, not just in skill text.** A skill explains and recommends. Anything that must *always* hold (no editing locked test files, no new unvetted dependencies, required checks pass) is enforced mechanically. | Evidence that agents follow instruction files reliably enough that enforcement isn't worth the cost at solo scale. |
| H11 | **Use formal standards as a crosswalk, not a process to adopt wholesale.** Each handbook page borrows the *specific* criteria a standard defines (e.g., the characteristics of a good requirement, a test-documentation outline, change-management controls) and cites the clause. That gives the handbook recognized authority without enterprise paperwork, and it means the normal workflow already produces the evidence a later audit (e.g., SOC 2) would ask for. | Evidence that the standards' useful content can't be separated from their process overhead, or that their criteria add nothing beyond the established practitioner guidance already in RQ1–RQ5. |
| H10 | **Tiering scales depth, it never skips a phase.** An agent may *propose* a tier, but the developer decides. Escalation triggers are mechanical where possible (the diff touches auth, migrations, money, public APIs, `unsafe`, or new dependencies), so a tier can't be argued down. | Evidence that agent tier recommendations are reliable, or that mechanical triggers produce so many false escalations that people ignore them. |

### 3b. Outside claims to test (from an "AI-native SDLC" summary the reader collected)

The reader brought in a summary that presents these as "validated techniques for enterprise." **It cites no sources.** Treat each one as a lead. My (the plan author's) starting view is noted, but don't adopt it without evidence. Give each claim a verdict in D1/D3.

| Claim | Plan author's starting view | What to check |
|---|---|---|
| **Intent and specs as code** (`intent.md`, `spec.md` in the repo before coding) | Agree in principle (it's in RQ1/RQ3 already). Concern: a solo developer gets doc sprawl and specs that drift from the code. The artifact should be the **smallest one that fits the tier**, with a drift check. | Do spec-first agent workflows (AWS AI-DLC, Kiro, GitHub Spec Kit) show measured benefit, or only vendor claims? What's the cheapest form that still prevents ambiguity? |
| **Steering / skills files auto-load guardrails** | Partly agree. Skills load when their **description matches the request**, which isn't guaranteed. Long instruction files get ignored. Instructions read from repo files are also a **prompt-injection surface**. Instructions guide; hooks and CI enforce (H9). | How skills and instruction files load (vendor docs, T3). Any evidence on how often agents follow instructions or how bloat affects that. |
| **TDD and DDD "baked into the core"** | TDD: yes, with guardrails against agents gaming their own tests. **Full DDD: likely enterprise overhead** for a solo developer. Probably keep only a **glossary / ubiquitous language** and clear module boundaries. | Evidence on TDD with agents. Whether any lightweight DDD practice pays off at small scale. |
| **AI picks the lifecycle and skips stages** | **Disagree as written.** Skipping stages contradicts the reader's own course notes (Unit 07: methodology never removes a phase) and hands the agent a decision the developer should own. Replaced by H10. | Any evidence that agent-chosen process works. |
| **Separate author and reviewer** | Agree in direction, but see H8: independence comes from fresh context and different inputs. A second agent is **not** a replacement for human review of risky areas. | Research on LLM self-correction and self-review limits. Evidence for cross-model or fresh-context review. |
| **Deterministic anchors first; validate dependencies against hallucinated packages** | **Strongly agree** (H7). Package hallucination ("slopsquatting") is a documented supply-chain risk, and in Rust it's cheap to block with `cargo-deny` plus a dependency allowlist. | The package-hallucination study (see §7). How often it happens and whether it applies to crates.io. |
| **Adversarial AI testing** (invalid inputs, interrupted transactions, expired credentials) | Agree as a *test-design prompt*, but an AI adversary has no oracle and can also write happy-path tests. Pair it with **objective test-quality signals**: mutation testing (`cargo-mutants`), property tests, fuzzing. | Evidence on whether LLM-generated adversarial or edge-case tests find real defects. Mutation testing as a check on test quality. |
| **Humans focus review on high-risk areas** | Agree. This is what the tier model and escalation triggers do. | Code-review guidance (Google eng-practices) and evidence about how heavy reviewing AI-written code is. |
| **"Drastically accelerate velocity", "drastically reduces cognitive load"** | **Unsupported as stated.** At least one controlled trial reported experienced developers were *slower* with AI tools. Don't repeat speed claims in the handbook unless a study card backs them. | Compare METR, DORA and vendor studies on task, participants and measure (§5). |
| **"Enterprise" framing in general** | The reader is one junior developer. Anything that only pays off with platform teams, compliance audits or many reviewers gets marked **[Large-org only]** and left out of v1. | Fit at scale (RQ3 criterion). |

---

## 4. Research questions

Each question feeds a decision. Stop researching a question once the decision is supported (see §10).

### RQ1: The everyday workflow → *Decision 2*
Trace one change through **problem definition → requirements → design → implementation → verification → release → maintenance**. For each stage:
- What **decision** does the developer make?
- What is the **smallest useful artifact** (e.g., a one-line problem statement, Given/When/Then acceptance criteria, an ADR, a test plan, a PR description, a rollback note)?
- What **evidence** lets you move to the next stage?

Then show how the workflow differs for:
- (a) a **small bug fix**
- (b) a **normal feature**
- (c) a **high-risk change** (e.g., auth, data migration, payments, anything hard to reverse)

Find the criteria that should **escalate** a change to a higher tier.

### RQ2: What changes when an agent does some of the work → *Decisions 1 & 2*
Look at: task scoping, repository instructions (CLAUDE.md / AGENTS.md / skills), choosing context, planning, delegating to autonomous runs, human review, and verification.

Find evidence on:
- **Where agents improve outcomes**, and on what measure (speed, correctness, review effort, security).
- **Where they increase review effort** or produce **plausible but wrong** work.
- **Where a passing check gets mistaken for proof of user behavior.** Examples: tests that mirror the implementation, edited or deleted tests, mocks that hide the real integration.

Conclude with:
- **Which decisions the developer should keep.**
- **What evidence justifies delegating more**, and in what order (e.g., a track record on bounded tasks, test quality signals, review findings).

### RQ3: Which existing systems to borrow from → *Decision 1*
Compare a **small, complementary set** (3–6 total) across three areas:
- requirements and design (e.g., ADRs, design docs/RFCs, spec-driven development, Shape Up)
- testing and delivery (e.g., test pyramid variants, Google testing practice, DORA, SRE release practice, NIST SSDF / SLSA)
- agent workflows (e.g., vendor best practices, AGENTS.md, Agent Skills, spec-first agent tooling)

For each one, rate:
- **evidence quality**
- **portability** across repos and agents
- **maintenance cost**
- **fit at solo / small-team scale**

Then say whether you'd **adopt, adapt, or leave it out**, and why. Note any reusable formats, checklists, examples, or evaluation methods.

### RQ4: What becomes a reusable skill → *Decision 3*
Compare at least these four:
- **test planning / TDD**
- **design and implementation planning**
- **PR verification / release readiness**
- **independent code review** (fresh context; reviews the diff against the spec and the RQ5 quality bar, without the author's reasoning)

The last two may merge into one skill. If you recommend merging them, say so and explain why.

Score each 1–5 on:

| Criterion | Weight |
|---|---|
| How often it's used | 0.3 |
| Cost of the mistakes it prevents | 0.3 |
| How easily the reader can verify its output | 0.2 |
| Effort to build and maintain (inverse) | 0.2 |

Show the scores. Recommend **one first skill**, and name the specific evidence that would change the ranking.

### RQ5: What "good code" means when an agent wrote it → *Decisions 1 & 3*
The reader's goal is **production-quality code, not code that merely passes**. Define a quality bar that works for one developer and can be checked:
- **What a senior reviewer looks for** beyond "the tests pass": correctness on the unhappy path, error handling (no swallowed errors; in Rust, no `unwrap`/`expect` on input or I/O in production paths), readability, the simplest design that works, clear API and module boundaries, focused diffs (no unrelated refactors), no dead code, and a justification for any new dependency. Use established review guidance (e.g., Google eng-practices "What to look for").
- **Defects common in agent-written code**:
  - invented or typosquatted packages
  - tests that just restate the implementation
  - edited or weakened assertions
  - happy-path-only logic
  - duplicated code instead of reuse
  - scope creep
  - plausible but wrong API usage
  - confident comments that don't match the code

  Which of these have **empirical support** and which are anecdote?
- **Which parts can be checked mechanically, and which need judgment?**
  - Mechanical, for Rust: `rustfmt`, `clippy` (which lint groups), `cargo-deny`, `cargo-audit`, `cargo-mutants`, `miri` for `unsafe`, a diff-size limit.
  - Needs review: everything else.

  Map each item to the layer that enforces it: hook, CI, skill checklist, or human.
- **Output:** a one-page quality bar that the handbook, the review skill and CI all refer to. That page is the single source of truth.

### RQ6: Which formal standards and frameworks give the handbook credible backing → *Decision 1*
The reader wants practices backed by recognized bodies, not just blog consensus. Survey the standards below. For each one, find:
- **What it governs**, and the **specific clauses or criteria** worth borrowing. Name the clause or section, not just the standard.
- **What kind of authority it has.** Consensus-based (a standards body), an audit or attestation framework, or empirically tested. **These are different.** A standard records expert consensus. It doesn't show that a practice improves outcomes. Say which applies.
- **Access:** free or paywalled. For paywalled standards, which free official summaries, previews or derived public documents can be cited accurately. Never paraphrase a standard you couldn't read as if you had.
- **What a solo developer or small team can take from it in under a page**, vs what is **[Large-org only]**.
- **Relevance to AI-assisted development**, if any.

**Standards to cover.** Verify the current version or edition of each, and drop any that turn out to be superseded or irrelevant.

- **Lifecycle and body of knowledge**
  - ISO/IEC/IEEE 12207 (software life cycle processes)
  - SWEBOK Guide v4 (IEEE Computer Society, free)
- **Requirements**
  - ISO/IEC/IEEE 29148 (requirements engineering; replaces IEEE 830)
  - Its "characteristics of good requirements" vs the CSE 370 Unit 02 criteria
- **Architecture and design**
  - ISO/IEC/IEEE 42010 (architecture description)
  - IEEE 1016 (software design descriptions)
- **Testing**
  - ISO/IEC/IEEE 29119 (software testing; replaces IEEE 829). Note the professional criticism of it, and compare.
  - ISTQB syllabus (free, widely used)
- **Product quality**
  - ISO/IEC 25010 (quality model). Check whether it can structure the RQ5 quality bar.
- **Secure development**
  - NIST SP 800-218 SSDF, plus SP 800-218A (its generative-AI profile; check who it actually targets)
  - OWASP ASVS and OWASP SAMM
  - CISA Secure by Design
- **Supply chain**
  - SLSA
  - OpenSSF Scorecard
- **Compliance / audit**
  - **SOC 2** (AICPA Trust Services Criteria): especially the **change-management** (CC8.x) and system-operations (CC7.x) criteria, which show what evidence auditors expect for code changes. Examples: approved PRs, tested changes, separation of duties, traceability.
  - ISO/IEC 27001 Annex A secure-development controls, briefly.
- **AI-specific**
  - NIST AI RMF and its Generative AI Profile (NIST AI 600-1)
  - ISO/IEC 42001 (AI management systems). Decide whether either has anything usable at solo scale, or is [Large-org only].

**Key question:** which of these give the handbook criteria worth citing at small scale? And can the everyday workflow produce **audit-ready evidence as a side effect** (PR approval records, CI logs, ADRs, a traceability matrix), so that a later SOC 2 or customer security questionnaire costs little extra?

**Separation of duties for a solo developer.** SOC 2 and similar frameworks expect the author and the approver to be different people. Explain what a solo developer can credibly do instead, and whether an independent AI reviewer counts for anything here. It probably doesn't count as a control. Say so plainly.

---

## 5. Evidence standards

**Source tiers.** Prefer higher tiers.

| Tier | Examples |
|---|---|
| T1 | Official standards and handbooks. Original engineering guidance from the organization that practices it (e.g., Google eng-practices, SRE book, NIST). Peer-reviewed studies. |
| T2 | Clearly labeled preprints with a stated method. Public repos with concrete artifacts (templates, workflows, skill files). |
| T3 | Vendor documentation. Use it to establish **how a feature works**, not how well it works. |
| T4 | Vendor or consultancy productivity claims, blogs, surveys. Include only with their evidence weighed, and say so. |

**Formal standards** (ISO/IEC/IEEE, NIST, AICPA, OWASP) count as T1 **for what they define**. They do not count as evidence that a practice improves outcomes.

**Label every substantive claim**, right next to its citation:
- **[Standard]**: defined or required by a named standard or framework. Cite the clause or section and the edition.
- **[Established]**: long-standing engineering guidance with broad support.
- **[Observed]**: an empirical result from AI-assisted development. Must include a study card (below).
- **[Proposed]**: your own recommendation or synthesis.

**Study card** (required for every [Observed] source):
`Task · Participants (count, experience) · Tools/models + date · What was measured (speed / correctness / review effort / security / satisfaction) · Result · Why it may not generalize to a junior solo Rust developer using 2026 agents`

**When sources disagree**, compare them on task, participants, tooling and model generation, and outcome measured before concluding anything. Speed, correctness and reviewer effort answer different questions, so never combine them into one "productivity" claim. AI results from models before 2025 must be marked **[Older-generation tools]**.

**Budget:** cite **12–18 core sources** in the final report. Open **no more than ~28** across the whole run, including workers. W4's standards don't count against the per-worker limit of 4. W4 may open up to 8, because standards are short to scope and most of the work is reading clauses. Every cited source needs a **direct link** and a **date or version**. List any source you couldn't access, and don't cite it as if you'd read it.

---

## 6. Orchestration

### Phase 0: Ground yourself (lead only, before delegating)
1. Read the local CSE 370 notes and the two Eloksen specs. Write a **private context memo** (≤1 page, saved to `research/fable-run/01-context-memo.md` and never copied into worker briefs) covering: the reader's vocabulary, which CSE 370 sections can support which handbook sections, and the Eloksen patterns (in abstract form).
2. Draft the **synthetic cases** you'll use later (§8). Draft them before research so the findings can't be tailored to fit them.
3. Finalize the three worker briefs. Check that none of them contains private details.

### Phase 1: One parallel research pass (4 workers)

Use the models actually available in your session. Defaults:

| Worker | Model | Why |
|---|---|---|
| W1 SDLC frameworks and artifacts | Haiku 5.5 (fall back to Sonnet 5.5) | Mostly retrieval and summarizing primary docs |
| W2 Testing, CI and release | Sonnet 5.5 | Needs judgment on when practices scale with risk |
| W3 AI-assisted development and agent workflows | Sonnet 5.5 | Needs careful reading of study design and limitations |
| W4 Standards and compliance frameworks (RQ6) | Sonnet 5.5 | Needs precision on clauses, editions and access, and must separate consensus authority from evidence |

If the session has no WebFetch tool, workers fetch pages through the `web-fetch` agent. If delegation is unavailable, do a narrower pass yourself and state that limitation in the run report.

**Worker brief template.** Fill it in for each worker, and make it fully self-contained:

```
ROLE: Research assistant for a software-engineering handbook. Return evidence notes, not prose.
QUESTION: <the worker's RQ slice, rewritten as 2–4 concrete sub-questions>
READER: A junior developer, solo or on a small team, maintaining real software (mainly Rust). Uses AI coding agents interactively and for bounded autonomous tasks.
SCOPE: <in scope> / OUT OF SCOPE: <exclusions>
SOURCE RULES: Open every source you report. Prefer official handbooks, original engineering guidance, peer-reviewed or clearly labeled preprints, and public repos with real artifacts. Vendor docs only for how features work. Treat productivity claims by their evidence.
LEADS (unverified; use, replace, or reject): <3–6 leads from §7>
BUDGET: Up to 4 opened sources. Stop when your sub-questions are answered.
PRIVACY: Use no project names, private details or personal information in any search.
OUTPUT, one block per source:
  - Title, author/org, date/version, direct link
  - Tier (T1–T4) and claim type (Established / Observed)
  - Finding: the specific claim it supports, with the section/page where you saw it
  - Study card (empirical sources only): task · participants · tools/models+date · measure · result
  - Limitation: why this may not apply to the reader
  - Handbook implication: one sentence
THEN: list any gaps or conflicts you noticed and any leads you rejected and why. Keep the whole reply under 900 words.
```

**Worker scopes**

- **W1 (SDLC frameworks and artifacts):**
  - Lightweight requirements: problem statements, acceptance criteria, specification by example.
  - Design records: ADRs (Nygard, MADR), design docs/RFCs, spec-driven development.
  - Change planning: dependency ordering, "done when".
  - Key question: *which artifacts are worth the effort at which risk tier.*
- **W2 (Testing, CI and release):**
  - Test design: risk-based testing, boundaries/partitions, characterization tests for legacy code, property and mutation testing.
  - Verification evidence: what counts as proof, and what CI can't prove (hardware, real users).
  - Delivery checks per PR vs pre-release; supply chain (SSDF/SLSA) at small scale.
  - Release safety: rollback, flags, canaries, SLOs.
  - **Deterministic quality gates for Rust (RQ5):** lints, dependency policy, mutation testing as a measure of test quality, and an order for running the gates.
  - Key question: *how the evidence required scales with the risk of the change.*
- **W3 (AI-assisted development and agent workflows):**
  - Controlled studies and field studies of AI coding assistants and agents.
  - Agent failure modes around tests.
  - Official guidance on repo instructions, context, planning, delegation, and skills.
  - Evidence on review burden.
  - **The §3b claims:** spec-first agent methods, LLM self-review limits, independent review, package hallucination, and AI-generated adversarial tests.
  - **AI-specific defect patterns (RQ5).** Separate measured patterns from anecdote.
  - Key question: *which decisions to keep, and what evidence justifies delegating more.*

- **W4 (standards and compliance frameworks):**
  - Covers the RQ6 list.
  - Output per standard: governing body · current edition/year · free or paywalled · the specific borrowable clauses or criteria, with section numbers · authority type (consensus / audit / empirical) · solo-scale take vs [Large-org only] · AI relevance.
  - **Prefer the official source** (iso.org, ieee.org, nist.gov, aicpa-cima.com, owasp.org, slsa.dev) over vendor compliance blogs. Many SOC 2 explainers come from vendors selling compliance tools, so weigh them as T4.
  - If a standard is paywalled, say so. Use only the official abstract, a free preview, or a public document derived from it, and state which one.
  - Key question: *which criteria from recognized standards are worth citing in a one-person handbook, and what evidence trail would satisfy a SOC 2-style change-management review at small scale.*

### Phase 2: Triage (lead)
Build a **gap/conflict matrix**: for each RQ, mark it **supported**, **conflicting**, or **missing**. Allow at most **one targeted follow-up**, and only if a gap or conflict could change the **handbook structure** or the **first-skill choice**. If a worker fails, cover only the gap that matters to the final recommendation.

### Phase 3: Verification (lead, required)
**Open every source that supports a major recommendation yourself.** Major recommendations include the handbook model, the first skill, the workflow tiers, and the delegation guidance. Record a claim-check table:

| Claim in report | Source | Location (section/page) | Supports it? (yes / partly / no) | Note |
|---|---|---|---|---|

Downgrade or drop any claim marked *partly* or *no*. Include this table in the run report (D7).

### Phase 4: Synthesis (lead only)
You own the final calls: which practices to adopt, which skill comes first, and what evidence would change those decisions. Resolve each §3 hypothesis.

### Phase 5: Self-review before returning
Check the report against §11. Fix any failures before you send it.

---

## 7. Seed leads (UNVERIFIED)

These are starting points only. They may be wrong, out of date, or weaker than alternatives. **Open a lead before using it.** Replace it if something better exists.

- **AI-assisted development:**
  - METR's randomized controlled trial on experienced open-source developers using early-2025 AI tools (July 2025).
  - DORA's research on AI-assisted software development (2024–2025 reports).
  - Perry et al., "Do Users Write More Insecure Code with AI Assistants?" (ACM CCS 2023).
  - The GitHub Copilot controlled experiment (Peng et al., 2023). Note the vendor affiliation and the narrow task.
  - Analyses of SWE-bench / SWE-bench Verified validity.
- **Agent workflows:**
  - Anthropic's "Building effective agents" (Dec 2024).
  - Claude Code best-practices documentation.
  - Agent Skills documentation.
  - The AGENTS.md convention.
  - GitHub Spec Kit.
  - Kent Beck's "augmented coding" writing (2025).
  - AWS "AI-Driven Development Life Cycle" (AI-DLC, 2025) and Kiro spec documentation. Vendor sources: use them for the method, not for outcome claims.
  - The summary also mentioned an "Anthropic AI-native playbook". **Its existence is unconfirmed.** Find the actual document or drop the claim.
- **AI code quality and review:**
  - Spracklen et al., package hallucinations by code LLMs (USENIX Security 2025; preprint 2024).
  - Huang et al., "Large Language Models Cannot Self-Correct Reasoning Yet" (ICLR 2024).
  - Olausson et al., "Is Self-Repair a Silver Bullet for Code Generation?" (ICLR 2024).
  - GitClear's code-churn/duplication reports (T4: industry data, weigh the method).
  - Google eng-practices, "What to look for in a code review".
- **Requirements and design:**
  - Michael Nygard, "Documenting Architecture Decisions" (2011), and MADR.
  - Google-style design docs.
  - Shape Up (Basecamp).
  - Diátaxis (for organizing the handbook itself).
- **Testing and delivery:**
  - Google Engineering Practices (code review).
  - *Software Engineering at Google* (testing chapters).
  - Google SRE book (release engineering, SLOs).
  - "The Practical Test Pyramid" (martinfowler.com).
  - Feathers, *Working Effectively with Legacy Code* (characterization tests).
  - Google's published work on mutation testing at scale.
  - NIST SP 800-218 (SSDF) and SLSA.
  - *Accelerate* / DORA metrics.

---

## 8. Synthetic cases for the skill demo and evaluation

Draft these in Phase 0. Keep them synthetic, small, and in Rust. They must contain **planted risks** and come with an **answer key** written *before* you run any demo. Suggested starting points (refine them as you see fit):

1. **New feature:** a bounded config value (e.g., `max_retries`, 0–10, default 3) settable through an HTTP API, with a specific error code for out-of-range values.
   - Planted risks: off-by-one at the upper bound; negative or non-integer JSON input; the default applied when the field is missing; the error response contract checked only at the unit level; the saved value never actually read by the worker that uses it (needs an integration check).
2. **Change to existing code:** a title-truncation helper that slices a `&str` by **byte index**, which panics on multi-byte UTF-8 (emoji) near the limit.
   - Planted risks: no failing reproduction test; current behavior for ASCII not pinned before the fix; an **open product decision** (count bytes, `char`s, or grapheme clusters?); the same pattern in other places; a property test (never panics, output length bound) missing.
3. *(For the tier table and the evaluation, not the demo)* **High-risk change:** a schema migration that changes how user IDs are stored.
   - Planted risks: no backward-compatible expand/contract plan; no rollback; no checks against production-like data; nothing to monitor after release.
4. *(For the evaluation)* **An agent-written diff to review.** A small feature PR that passes CI but contains:
   - a dependency on a crate name that doesn't exist or is one character off from a popular crate
   - a test whose expected value was copied from the implementation's output
   - an `unwrap()` on user-supplied input
   - an unrelated "cleanup" refactor mixed in
   - a doc comment that describes behavior the code doesn't have

   A good review catches the first three as blocking and flags the last two.

---

## 9. Deliverables

Save each deliverable as its own file in `research/fable-run/deliverables/` (`D1-recommendation.md` … `D7-run-report.md`) **as soon as it's written**. Then assemble them into `research/fable-run/REPORT.md`. In the chat, give only a short summary that links to the files. Be practical and concise. Use tables where they help a decision. Whenever you recommend a practice, **name the decision it helps make**.

### D1. Recommendation (≤300 words)
- The handbook model.
- The first skill.
- The **2–3 tradeoffs** that drove those choices.
- The verdict on each §3 hypothesis and each §3b claim (one line each).

### D2. Lifecycle workflow
- **Main table** with these columns: *Stage · Developer decision · Agent contribution · Minimum artifact · Verification evidence to proceed · Who must sign off (developer / agent OK)*.
- **Tier table:** how the bug fix / feature / high-risk change each differ per stage, plus explicit **escalation triggers**.
- **Delegation ladder:** what an agent may do alone at each level of trust, and the evidence that earns the next level.
- **Verification order:** the sequence from deterministic checks to independent AI review to human review. For each check, say what it can and can't prove. In particular, a passing suite proves the tests pass, not that users get the right behavior.
- **Enforcement map:** each rule, assigned to *hook / CI / skill checklist / human*, with the reason it sits there.

### D3. Source and system comparison
- **Table** with columns: *System/source · Area · What it supports · Evidence quality (tier) · Portability · Maintenance cost · Fit at my scale · Adopt / Adapt / Leave · Why*.
- **D3b. Standards crosswalk table**, with columns: *Standard (edition) · Body · Governs · Authority type · Access · Clause(s) to borrow · Handbook page it backs · Solo take (≤1 line) · Large-org only? · AI relevance*.
  - Then an **audit-evidence map**: for each SOC 2 change-management expectation, the everyday artifact that already provides the evidence (e.g., PR approval, CI run log, ADR, release note), and the gaps a solo developer can't close. Separation of duties is the obvious one.
  - Finish with a short **"Where standards and practice disagree"** note. For example, criticism of ISO/IEC/IEEE 29119, or heavyweight IEEE documentation vs Agile's lean artifacts.
- Then a short **"Disagreements and gaps"** section: where the sources conflict (compared on task, participants, tools and measure), and which gaps could change the handbook design.

### D4. Handbook blueprint
- **The RQ5 quality bar page** in full (≤1 page). The handbook, the skills and CI all point to it.
- **Table of contents** (compact; target ≤12 short pages for v1). Mark each section with its **source**, the **standard clause** it aligns with (from D3b), and the **CSE 370 unit** that supports it, where you checked that.
- **Folder tree** stating where each of these lives:
  - reusable guidance
  - agent instructions (skills)
  - worked examples
  - templates
  - **project-specific decisions** (inside each project repo, not in the handbook)
- **Linking pattern:** a concrete example of a skill output line → handbook section anchor → that section's source citation. Explain how links still work when a skill is copied into another repo (e.g., a stable URL, or a version-pinned handbook path).
- **Maintenance rule:** how a page gets added, revised, or retired. What evidence earns content a permanent place.

### D5. First skill: design and evaluation
1. **Spec:**
   - when to invoke it (and when not to)
   - what it reads (spec, diff, tests, CI config, handbook pages) and in what order
   - its steps
   - its **output format** (each finding = *finding · evidence (file/line/spec ref) · proposed check + layer · why it matters for this change · handbook link*)
   - **stop-and-ask points** where developer judgment is required
   - guardrails against the failure modes found in RQ2
2. **Demo** on synthetic cases 1 and 2. Label it **[Illustrative, not yet tested]**. Show the expected skill output side by side with the expected **plain-agent output** (same model, a one-line request like "write tests for this"), and point out the differences.
3. **Evaluation protocol** the reader can rerun:
   - **Cases:** 6–10 synthetic cases with answer keys, mixing tiers and new vs existing code. Include at least one **agent-written diff with AI-specific defects** (case 4 style), whichever skill is chosen first.
   - **Conditions:** skill vs plain request, same model, same repo state. **3 runs per case** to measure variance.
   - **Rubric, scored 0–2 each:**
     - (a) **Catches the important risk** (matches the answer key)
     - (b) **Grounded in evidence** (real file/test/spec references, no invented ones)
     - (c) **Explains the reasoning clearly** (finding → evidence → action → why)
     - (d) **Proportional to the change** (no gold-plating on small fixes, enough depth on high-risk changes)
   - **Also record:** false positives, invented references, and time to review.
   - **Scoring:** blind it where possible (shuffle the outputs, or use a separate judge model *plus* a sample spot-checked by hand).
   - **Pass bar:** e.g., skill average ≥ plain + 1.5 points out of 8, with no invented references.

### D6. First build milestone
- The **smallest set** of handbook pages, skill instructions, examples and eval cases needed to try the approach. List the files.
- **What the reader can do with v1.**
- **Results that justify expanding** (e.g., eval pass bar met on 2 consecutive revisions; used on ≥5 real changes and caught at least one real issue).
- **Results that mean rethink**.

### D7. Run report (short)
- What was delegated, to which model, and what came back. Note any failures.
- Sources opened vs cited.
- The claim-check table (Phase 3).
- **Remaining evidence gaps** and how much each could change the recommendations.
- Anything inaccessible.

---

## 10. Stop rules

- Stop researching an RQ once its decision has at least one T1/T2 source **and** no unresolved conflict that would flip it.
- One parallel pass plus at most one follow-up. Do not start a second round of broad searching.
- If browsing fails or time runs short, synthesize what you have. List the **specific** missing evidence and say which recommendations it affects.

---

## 11. Quality bar (check before returning)

- [ ] All three decisions (§0) can be made from D1 alone, with D2–D6 as support.
- [ ] Every [Observed] claim has a study card. No speed result is presented as a correctness or quality result.
- [ ] Every major recommendation appears in the claim-check table as *yes*.
- [ ] Every cited source has a direct link and a date/version. Inaccessible sources are flagged.
- [ ] The labels [Established] / [Observed] / [Proposed] sit next to the claims they describe.
- [ ] Each §3 hypothesis and §3b claim has a verdict. No §3b claim is repeated as fact without a source.
- [ ] No speed or "velocity" claim appears without a study card.
- [ ] Every "must always" rule is mapped to a mechanical enforcement point (hook or CI), or explicitly left to human judgment with the reason.
- [ ] Enterprise-only practices are marked [Large-org only] and kept out of v1.
- [ ] Every [Standard] claim names the edition and clause. Paywalled standards are cited only through what was actually read (an abstract, a preview, or a public derived document), and the report says which.
- [ ] The report never presents a standard as proof that a practice works. Consensus authority and empirical evidence are kept separate.
- [ ] SOC 2 is described accurately: an attestation report against the Trust Services Criteria, not a certification or an engineering standard.
- [ ] No Eloksen or personal details appear anywhere in the report, the briefs, or the searches.
- [ ] Synthetic demo outputs are labeled illustrative, and the answer keys were written before the demos.
- [ ] Recommendations are sized for one junior developer to maintain. Anything that only pays off at large-org scale is marked as such.
- [ ] Plain language throughout. Each recommendation names the decision it supports.

---

## 12. Checkpoints and saving (protects against usage caps)

Assume the session can stop at **any moment**. The files are the source of truth, not the conversation. Write each result to disk **before** starting the next expensive step.

### Output folder: `research/fable-run/`
```
research/fable-run/
├── 00-run-state.md          # always current: phase, done list, next step, notes
├── 01-context-memo.md       # Phase 0 memo (Eloksen patterns in abstract form only)
├── 02-synthetic-cases.md    # cases plus answer keys, written BEFORE research (timestamped)
├── workers/
│   ├── W1-briefs-and-notes.md   # the brief sent, then the worker's returned notes
│   ├── W2-...md
│   ├── W3-...md
│   └── W4-...md
├── 03-triage.md             # gap/conflict matrix and any follow-up
├── 04-claim-check.md        # Phase 3 verification table
├── deliverables/
│   ├── D1-recommendation.md
│   ├── D2-lifecycle-workflow.md
│   ├── D3-source-comparison.md      # includes D3b standards crosswalk
│   ├── D4-handbook-blueprint.md     # includes the full quality-bar page
│   ├── D5-first-skill.md
│   ├── D6-first-milestone.md
│   └── D7-run-report.md
└── REPORT.md                # D1–D7 assembled, written last
```

### `00-run-state.md` format (update after every step)
```
# Run state
Plan version: 1.3
Last updated: <date time>
Current phase: <0–5 | GATE | BUILD>
Done: <checklist of completed steps with file links>
Next step: <exact next action, specific enough that a fresh session can do it cold>
Sources opened so far: <n> / ~28
Open issues: <anything a resuming session must know>
```

### Save points (required)
1. After Phase 0: the context memo, the synthetic cases with answer keys, and the four briefs.
2. **After each worker returns**: its notes, immediately. Don't wait for the others.
3. After triage, after any follow-up, and after the claim-check table.
4. After **each** deliverable D1–D7.
5. After `REPORT.md`.

### Resuming
At the start of **any** session given this plan:
1. If `research/fable-run/00-run-state.md` exists, **resume** from its "Next step". Don't redo completed steps or reopen sources already recorded in `workers/`.
2. If a worker was mid-run when the session stopped, rerun **only** that worker.
3. Never rewrite `02-synthetic-cases.md` after research has started. The answer keys must predate the findings. If a case truly needs fixing, append a dated correction and leave the original in place.

---

## 13. Handoff to the build

When `REPORT.md` is saved, run the **research gate**. Record the result in `00-run-state.md`.

**PASS** requires all of the following:
- Every major recommendation (the handbook model, the first skill, the tiers, delegation, the quality bar) is marked *yes* in `04-claim-check.md`.
- D6 lists concrete v1 files.
- D5 specifies the skill well enough to write it.
- No unresolved conflict would change the first-skill choice.

**On PASS:**
- Set `Current phase: BUILD`.
- Then **continue in this same session** by reading and executing `research/fable-5.2-build-plan.md`.
- Before the build starts, write a 10-line summary of D1 and D6 to `research/fable-run/HANDOFF.md`. The build works from the saved files, not from conversation memory.

**On FAIL:**
- Write `research/fable-run/GATE-FAILED.md` listing exactly which checks failed and what evidence would fix each one.
- Stop. Don't start the build.

If the session stops between the gate and the build, a new session given *either* plan file reads `00-run-state.md` and picks up from the right place.
