# Gemini Research Prompts — SDLC Handbook

Goal: gather sources from outside CSE 370 for building Claude Code skills and tools that produce safe, production-grade code. These are not for vibe coding.

**How to use**
- Run each prompt in its own Gemini **Deep Research** session.
- Save each result as `research/results/NN-topic.md`.
- Run prompt 9 last, in a session where you paste results 1–8.

The context block below is already built into every prompt, so each one can be pasted on its own.

> **Context:** I'm a software engineer on a small team building a production product. We use AI coding agents (Claude Code) heavily, and I'm writing "skills": instruction files the agent follows, plus CI/CD tooling, so that agent-written code meets real production standards. Our main language is Rust; some TypeScript/Python may appear. I already know textbook SDLC (Waterfall/Agile/Scrum, test pyramid, V-model, user stories, UML). I need **current (2023–2026), practitioner-level, actionable** guidance with sources, not textbook definitions.

---

## SPECS

### 1. Spec-driven development with AI agents
```
Context: I'm a software engineer on a small team building a production product. We use AI coding agents (Claude Code) heavily, and I'm writing "skills" (instruction files the agent follows) plus CI/CD tooling so agent-written code meets real production standards. Main language is Rust. I already know textbook SDLC. I need current (2023–2026), practitioner-level, actionable guidance with sources.

Research "spec-driven development" and how specs are written for AI coding agents:
1. What spec-driven development means in 2025–2026: GitHub Spec Kit, Kiro (AWS), Tessl, the "requirements.md / design.md / tasks.md" pattern, and similar approaches. Compare them.
2. What a good spec for an AI agent contains: scope, non-goals, acceptance criteria, constraints, interfaces, error cases, and the open questions the agent must ask before coding.
3. How acceptance criteria (Given/When/Then, BDD/Gherkin, specification by example) turn mechanically into test cases, and whether that's worth formalizing.
4. Lightweight requirement-to-test traceability for a small team (IDs in test names, tags, a generated matrix) that doesn't require heavyweight tools.
5. Failure modes: specs that drift from code, over-specification, agents ignoring specs, ambiguous specs ("fast", "user-friendly").

Give concrete templates and examples. Cite and link sources.
```

### 2. Requirements quality and change management in small teams
```
Context: small product team, AI coding agents (Claude Code), Rust. I want practical, current practice, not textbook theory.

Research how high-performing small teams (startups, open-source projects) capture and manage requirements in practice:
1. Formats actually used: PRDs, one-pagers, RFCs, GitHub issues with acceptance criteria, user story maps, Shape Up "pitches". Strengths and weaknesses of each.
2. How to judge requirement quality (complete, unambiguous, feasible, testable), with checklists or lint rules that could be automated or run by an AI reviewer.
3. How non-functional requirements (performance budgets, security, reliability SLOs, accessibility) are written so they become measurable and testable in CI.
4. How requirement changes are tracked and flow through to design, tests and docs without things going stale.

Output concrete templates and a requirement-quality checklist. Cite sources.
```

---

## DOCS

### 3. Docs-as-code and design documentation
```
Context: small product team, AI coding agents (Claude Code), Rust, GitHub. I need practical, current guidance with sources.

Research the best current practice for engineering documentation that stays accurate:
1. Docs-as-code: keeping docs in the repo, reviewing them in PRs, and checking them in CI (link checking, doc tests like Rust's doctests, markdown lint, docs-drift detection).
2. Architecture Decision Records (ADRs): the formats (Nygard, MADR, Y-statements), when to write one, how to supersede one. Include real examples.
3. Design docs / RFCs: Google-style design docs, Rust RFC process, Oxide RFDs. What sections matter and how long they should be.
4. The Diátaxis framework (tutorials, how-to guides, reference, explanation) and how to apply it to a small project.
5. API documentation: OpenAPI/schema-first, rustdoc conventions, documenting error cases and invariants.
6. Lightweight diagrams as code: Mermaid, C4 model, PlantUML, D2.

Give concrete templates, file layouts, and CI checks. Cite and link sources.
```

### 4. Documentation for AI agents
```
Context: we use AI coding agents (Claude Code) heavily on a production Rust codebase.

Research how teams write and maintain documentation that AI coding agents consume:
1. Agent instruction files: CLAUDE.md, AGENTS.md, .cursorrules, copilot-instructions.md. What belongs in them, recommended structure and length, and real examples from well-known open-source repos.
2. Claude Code "skills" and similar reusable instruction packs: how they're structured and what makes one effective vs ignored.
3. Keeping agent context accurate: preventing stale instructions, layering (global vs repo vs directory), and "progressive disclosure" of docs.
4. Having agents write and update docs (changelogs, ADRs, API docs) as part of each change, and how to review that output.
5. Known failure modes: bloated instruction files, contradictory rules, agents ignoring instructions, prompt injection via docs.

Cite Anthropic's docs, other vendors' docs, and credible practitioner write-ups. Include links and concrete examples.
```

---

## TESTING

### 5. TDD with AI coding agents
```
Context: small team, AI coding agents (Claude Code), Rust. I'm building a skill that makes the agent do disciplined test-driven development on production code.

Research TDD as practiced with AI coding agents in 2025–2026:
1. Known failure modes when LLMs write tests: tests mirroring the implementation, editing or deleting tests to make them pass, assertion-free tests, over-mocking, hardcoded expected values, "reward hacking" the test suite.
2. Guardrails that work: tests written and confirmed failing before implementation; separate test-writer and implementer agents; locking test files; hooks that block test edits; mutation testing to check test quality.
3. Kent Beck's writing on "augmented coding" / TDD with AI, plus other credible practitioner write-ups.
4. Existing open-source agent TDD instructions (Claude Code skills, CLAUDE.md examples, the "superpowers" skill set, Cursor rules) and what they get right or wrong.
5. Research papers or benchmarks on the quality of LLM-generated tests.
6. Core TDD discipline as experts describe it: red-green-refactor step size, Chicago vs London school, what makes a good unit test.

Focus on actionable rules. Cite and link sources.
```

### 6. Test strategy and test design for production Rust
```
Context: production Rust codebase, small team, AI coding agents.

Research a complete, current testing strategy for production Rust:
1. Test layers: unit (#[cfg(test)]), integration (/tests), doc tests, end-to-end. Test pyramid vs testing trophy vs honeycomb, and current consensus.
2. Beyond example tests, and when each is worth the cost: property-based (proptest), mutation testing (cargo-mutants), fuzzing (cargo-fuzz), snapshot testing (insta), concurrency (loom), UB detection (miri), and contract tests.
3. Tooling: cargo-nextest, cargo-llvm-cov, rstest, mockall vs trait-based fakes, wiremock, testcontainers-rs, async test patterns with tokio.
4. Designing code for testability in Rust: trait-based dependency injection, avoiding global state, testing error paths.
5. A practical "what to test" checklist: boundary values, equivalence partitions, edge cases by data type (empty, unicode, overflow, time zones), failure and partial-failure paths.
6. Coverage: line vs branch vs diff/patch coverage, sensible thresholds, why coverage gets gamed, and mutation score as the better quality signal.

Flag deprecated or unmaintained crates. Cite and link docs.
```

---

## CI/CD

### 7. The CI/CD pipeline and quality gates
```
Context: small team, production Rust product, GitHub Actions, AI coding agents writing much of the code. I want the pipeline to be the safety net that stops bad agent-written code from shipping.

Research best-practice CI/CD design for this setup:
1. Pipeline stages and which gates run per-PR vs on merge vs nightly vs pre-release: fmt, clippy, build, unit/integration tests, coverage, mutation tests, fuzzing, cargo-deny/cargo-audit, SAST, secret scanning, license checks, benchmarks.
2. Branch protection, required checks, merge queues, trunk-based development vs GitFlow for a small team.
3. Keeping CI fast: caching (sccache, rust-cache), test sharding, running only affected tests, cost control.
4. Flaky tests: detection, quarantine, and why automatic retries are dangerous.
5. Supply-chain security: SLSA levels, artifact signing (Sigstore/cosign), SBOMs, pinning actions by SHA, least-privilege GITHUB_TOKEN, OIDC for cloud deploys.
6. Example GitHub Actions workflows from well-regarded Rust projects (ripgrep, tokio, rust-analyzer, etc.).

Give concrete numbers, YAML snippets, and rules. Cite and link sources.
```

### 8. Release, deployment and operating in production
```
Context: small team shipping a production Rust service/product with CI/CD on GitHub Actions.

Research current best practice for the release and operate half of the cycle:
1. Versioning and release automation: SemVer, Conventional Commits, automated changelogs (release-please, cargo-release, git-cliff, semantic-release).
2. Deployment strategies: continuous delivery vs continuous deployment, canary, blue/green, progressive rollout, automated rollback criteria.
3. Feature flags: when to use them, flag lifecycle and cleanup (flag debt), tools.
4. Database/schema migrations safely in CI/CD (expand-contract pattern).
5. Observability as part of "done": structured logging, metrics, tracing (OpenTelemetry, tracing crate), SLOs/error budgets, alerting.
6. Measuring the whole delivery system: DORA metrics (deployment frequency, lead time, change failure rate, time to restore) and the latest DORA report findings, including on AI-assisted development.
7. Incident response and blameless postmortems for small teams.

Concrete, actionable, with sources and links.
```

---

## SYNTHESIS (run last)

### 9. A full SDLC loop for an AI-assisted small team
```
I'm pasting research on specs, documentation, testing, and CI/CD below. Using it, design a concrete end-to-end development loop for a small team where AI coding agents write much of the code and production quality is mandatory.

Output:
1. The loop, step by step, from idea → spec → design/ADR → tests (failing) → implementation → review → CI gates → release → monitoring → feedback. For each step, state its inputs and outputs (artifacts), who or what does it (human, agent, CI), and the exit criteria to move on.
2. A traceability chain: how a requirement ID flows through spec, tests, code, PR, release notes.
3. A "Definition of Done" checklist for a change.
4. Hard rules for the AI agent (never/always), split into those the agent must self-enforce and those CI/hooks should enforce automatically.
5. A recommended repo layout for specs, ADRs, docs, and tests.
6. Which parts should become separate agent skills (e.g., spec-writing skill, TDD skill, review skill, release skill) and what each one owns.

Be concise and enforceable. No vague advice.

[PASTE RESEARCH RESULTS HERE]
```
