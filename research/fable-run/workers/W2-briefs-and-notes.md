# W2: Testing, CI and release

Model: Sonnet 5.5. Brief written 2026-11-13 00:40 MST.

## Brief sent

```
ROLE: Research assistant for a software-engineering handbook. Return evidence notes, not prose.
QUESTION:
  1. Test design and test quality: what does original guidance say about choosing test size/layer, testing behavior rather than implementation, characterization tests for legacy code, and mutation testing as a measure of test quality (including Google's published mutation-testing work)?
  2. What does a reviewer look for beyond "tests pass"? Extract the specific list from Google eng-practices "What to look for in a code review", and its guidance on small changes.
  3. Release safety at small scale: what do primary sources say about rollback, canaries/progressive rollout and release gating, and which of it only pays off at large scale?
  4. Deterministic quality gates for Rust: what do the official docs say these tools check and cannot check: Clippy lint groups (and restriction lints such as unwrap_used), cargo-deny (advisories, bans, licenses, sources), cargo-audit, cargo-mutants, Miri? Is there guidance on the order to run gates?
  Key question: how should the evidence required scale with the risk of the change?
READER: A junior developer, solo or on a small team, maintaining real software (mainly Rust, some TypeScript or Python). Uses AI coding agents interactively and for bounded autonomous tasks.
SCOPE: test design, verification evidence, per-PR vs pre-release checks, release safety, Rust quality tooling. OUT OF SCOPE: AI-assistant studies, requirements/design artifacts, formal ISO/IEEE/NIST standards text (other workers cover those).
SOURCE RULES: Open every source you report (use WebSearch to find it and WebFetch to read it; if you have no such tools, say so at the top and stop). Prefer official handbooks, original engineering guidance, peer-reviewed or clearly labeled preprints, and public repos with real artifacts. Vendor docs only for how features work. Treat productivity claims by their evidence. Never report a detail (number, section, date) you did not see on the page.
LEADS (unverified; use, replace, or reject):
  - Google eng-practices, "What to look for in a code review" and "Small CLs" (google.github.io/eng-practices)
  - Software Engineering at Google (abseil.io/resources/swe-book), testing chapters 11 to 14
  - Petrovic and Ivankovic, "State of Mutation Testing at Google" (ICSE-SEIP 2018), or Petrovic et al., "Does mutation testing improve testing practices?" (ICSE 2021)
  - Google SRE book, "Release Engineering" chapter (sre.google/sre-book)
  - Ham Vocke, "The Practical Test Pyramid" (martinfowler.com, 2018)
  - Official docs: cargo-deny (embarkstudios.github.io/cargo-deny), cargo-mutants (mutants.rs), Clippy lint groups (doc.rust-lang.org/clippy)
BUDGET: Up to 4 opened sources for sub-questions 1 to 3. For sub-question 4 you may additionally open up to 3 official tool documentation pages (T3, how the tool works only). Stop when your sub-questions are answered.
PRIVACY: Use no project names, private details or personal information in any search. You have no local file access and need none. Do not write any files.
OUTPUT, one block per source:
  - Title, author/org, date/version, direct link
  - Tier (T1 official standard, original practitioner guidance or peer-reviewed / T2 labeled preprint or public repo artifact / T3 vendor docs / T4 vendor claims, blogs, surveys) and claim type (Established / Observed)
  - Finding: the specific claim it supports, with the section/page where you saw it
  - Study card (empirical sources only): task · participants · tools/models+date · measure · result
  - Limitation: why this may not apply to the reader
  - Handbook implication: one sentence
THEN: list any gaps or conflicts you noticed and any leads you rejected and why. Keep the whole reply under 1000 words.
```

## Notes returned

Returned 2026-11-13 about 03:15 MST (Sonnet 5.5, resumed once after a usage-limit stop). 4 sources opened for sub-questions 1 to 3, plus 3 official tool pages. Saved as returned, lightly reformatted.

### Source 1: Google eng-practices, "What to look for in a code review"
- Link: https://google.github.io/eng-practices/review/reviewer/looking-for.html (date/version not seen)
- Tier: T1 (original practitioner guidance). Claim type: Established.
- Finding (the page's 12 items): Design; Functionality (edge cases, concurrency, bugs visible in the code); Complexity (over-engineering, speculative features); Tests (unit, integration or end-to-end tests "as appropriate for the change"; tests must be correct, useful, would fail if the code broke, and are maintained like other code); Naming; Comments (explain why, not what; stale TODOs); Style ("Nit:" prefix for non-mandatory points; big reformatting in separate changes); Consistency; Documentation (update if build, test, use or release changes); Every Line (read all human-written code; ask another reviewer if unsure); Context (look beyond the diff; does it improve code health); Good Things.
- Limitation: Assumes a human reviewer and a team. A solo developer has no second reviewer.
- Handbook implication: A self-review or agent-review checklist should include "would this test fail if the code broke?", not just "tests pass".

### Source 2: Google eng-practices, "Small CLs"
- Link: https://google.github.io/eng-practices/review/developer/small-cls.html (date not seen)
- Tier: T1. Claim type: Established.
- Finding: Small changes are reviewed faster and more thoroughly, are less likely to introduce bugs, waste less work if rejected, merge more easily and roll back more simply. One self-contained change addressing one thing. Rough guide: about 100 lines is usually reasonable, about 1000 usually too large; file count matters too. Tests belong in the same change. Refactorings go separately from features and bug fixes (small local cleanups are fine). Large is acceptable for whole-file deletes and trusted automated refactors.
- Limitation: The line counts are rules of thumb, not measured thresholds.
- Handbook implication: Cap a PR or agent task at one concern with its tests, and keep refactors in separate PRs.

### Source 3: Petrovic, Ivankovic, Fraser, Just, "Does mutation testing improve testing practices?" (ICSE 2021)
- Link: https://arxiv.org/abs/2103.07189 (submitted 12 Mar 2021). **Only the arXiv abstract page was opened**, not the full paper. Venue and pages (ICSE 2021, pp. 910-921) come from a search snippet.
- Tier: T1 peer-reviewed (preprint page opened). Claim type: Observed.
- Finding (abstract): developers exposed to mutation testing write more tests and improve their suites so that fewer mutants remain; mutants are coupled with real faults. Dataset: almost 15 million mutants.
- Study card: task: mutation testing surfaced during Google code review · participants: not stated on the page seen · tools/date: Google's system, 2021 · measure: surviving mutants over time; coupling to past real fixes · result: qualitative only on the abstract page; no effect sizes seen.
- Limitation: Google-internal, diff-based mutation shown in code review; observational; no independent replication found.
- Handbook implication: Use mutation results on changed code as a signal of test sensitivity, not as a repo-wide score target.

### Source 4: Google SRE Book, "Release Engineering"
- Link: https://sre.google/sre-book/release-engineering/ (date not seen)
- Tier: T1. Claim type: Established (practice descriptions).
- Finding: High release velocity means fewer changes per release, which makes testing and troubleshooting easier. Hermetic builds. Release branches cut from a tested mainline revision, fixes cherry-picked. Unit tests rerun on the release branch. A canary of a few production jobs after system tests (no duration or criteria given). Rollout strategy matched to risk. Gated access and code review on release actions. A per-release change report. Sections "It's Not Just for Googlers" and "Start Release Engineering at the Beginning". Rollback is mentioned only briefly; configuration can be fixed through a flag change without a new binary.
- Scale-dependent: the named internal tools, the monorepo, and cluster-by-cluster progressive rollout. Likely applicable small: reproducible builds, a tagged release from a tested revision, a change report, risk-matched rollout.
- Limitation: No rollback procedure detail and no canary metrics.
- Handbook implication: A small team gets most value from reproducible builds, tagged releases, a known last-good version to redeploy, and a short canary only if multiple instances exist.

### Source 5: Clippy lint docs (T3)
- Link: https://doc.rust-lang.org/clippy/lints.html
- Finding: Groups are correctness (deny by default; code that is outright wrong or useless), suspicious, complexity, perf, style, pedantic (expect many `#[allow]`), restriction (the docs say not to enable the whole group; cherry-pick individual lints such as `clippy::unwrap_used`, and either fix or `#[allow]` with a justifying comment), cargo, nursery (not recommended as a whole group). The page did not state default levels for groups other than correctness (the worker noted its knowledge of the other defaults is not from the page). It does not say what Clippy cannot check.
- Handbook implication: Gate on the default groups plus a hand-picked few restriction lints, not whole groups.

### Source 6: cargo-deny checks (T3)
- Link: https://embarkstudios.github.io/cargo-deny/checks/index.html
- Finding: Four checks: licenses, bans (specific crates and duplicates), advisories (vulnerabilities, unmaintained, yanked), sources (where crates come from). `cargo deny check` runs all of them by default. No ordering guidance or limits stated on the page.
- Handbook implication: One command covers supply-chain policy. It checks metadata and advisory databases, not whether the code is correct.

### Source 7: cargo-mutants welcome page (T3)
- Link: https://mutants.rs/welcome.html
- Finding: The tool's stated purpose is finding places where a bug could be inserted without any test failing. The caught/missed definitions and caveats are in a "Cautions" chapter that was not read.
- Handbook implication: A surviving mutant points at a test gap. Treat it as a prompt to look, not a defect count.

### Gaps and not verified
- **Test size/layer, behavior vs implementation, characterization tests: not sourced** (Software Engineering at Google ch. 11 to 14 and Vocke not opened: budget).
- Google's 2018 "State of Mutation Testing at Google" not opened.
- cargo-audit and Miri docs not opened. What they cannot check is unsourced.
- **Gate ordering: no official guidance found.** Cheap-first ordering is the worker's inference, not a source.
- Key question (evidence scaled to risk): the only support seen is the SRE book's rollout matched to risk and Google's tests "as appropriate for the change". No explicit tiering guidance found.
- Rejected lead: the research.google publication URL returned 404; arXiv was used instead.
