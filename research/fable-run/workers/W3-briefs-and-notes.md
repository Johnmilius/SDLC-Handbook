# W3: AI-assisted development and agent workflows

Model: Sonnet 5.5. Brief written 2026-11-13 00:40 MST.

Budget note: the plan's template gives each worker 4 opened sources. W3's slice covers RQ2, RQ5 and most of the §3b claims, so the lead gave it 6 and reduced its own allowance to stay inside the run total of about 28.

## Brief sent

```
ROLE: Research assistant for a software-engineering handbook. Return evidence notes, not prose.
QUESTION:
  1. What do controlled or field studies show about AI coding tools on speed, correctness, security and review effort, measured separately? Read the study design carefully and report each measure on its own. Never merge them into one "productivity" claim.
  2. Package hallucination: what did Spracklen et al. measure (languages, models, rates), and does it say anything about Rust/crates.io?
  3. Self-review limits: what do Huang et al. (ICLR 2024) and/or Olausson et al. (ICLR 2024) show about a model correcting or repairing its own output, and is there evidence that external feedback, a fresh context or a different model helps?
  4. Agent behavior around tests and instructions: is there measured evidence (not anecdote) of agents editing or weakening tests, writing tests that mirror the implementation, duplicating code, or ignoring long instruction files? What does official vendor guidance say about instruction files (CLAUDE.md / AGENTS.md), skills (how they load), hooks, and separating writer and reviewer?
  Key question: which decisions should the developer keep, and what evidence justifies delegating more to an agent?
READER: A junior developer, solo or on a small team, maintaining real software (mainly Rust, some TypeScript or Python). Uses AI coding agents interactively and for bounded autonomous tasks.
SCOPE: empirical studies of AI coding assistants and agents; LLM self-correction; package hallucination; official agent-workflow guidance. OUT OF SCOPE: general testing practice, requirements/design artifacts, formal standards (other workers cover those).
SOURCE RULES: Open every source you report (use WebSearch to find it and WebFetch to read it; if you have no such tools, say so at the top and stop). Prefer official handbooks, original engineering guidance, peer-reviewed or clearly labeled preprints, and public repos with real artifacts. Vendor docs only for how features work. Treat productivity claims by their evidence. Never report a detail (number, section, date) you did not see on the page. Mark any result from models released before 2025 as [Older-generation tools].
LEADS (unverified; use, replace, or reject):
  - METR, "Measuring the Impact of Early-2025 AI on Experienced Open-Source Developer Productivity" (July 2025), metr.org or arXiv 2507.09089
  - Spracklen et al., "We Have a Package for You! A Comprehensive Analysis of Package Hallucinations by Code Generating LLMs" (USENIX Security 2025), arXiv 2406.10279
  - Huang et al., "Large Language Models Cannot Self-Correct Reasoning Yet" (ICLR 2024), arXiv 2310.01798
  - Olausson et al., "Is Self-Repair a Silver Bullet for Code Generation?" (ICLR 2024), arXiv 2306.09896
  - Perry et al., "Do Users Write More Insecure Code with AI Assistants?" (ACM CCS 2023), arXiv 2211.03622
  - DORA, "State of AI-assisted Software Development" 2025 report (dora.dev)
  - Anthropic, Claude Code best practices (code.claude.com/docs or anthropic.com/engineering/claude-code-best-practices); Agent Skills docs
  - GitClear code-quality report (T4: weigh the method)
  - An "Anthropic AI-native playbook" was mentioned to us without a link. Its existence is unconfirmed. Spend at most one search on it and report found or not found.
BUDGET: Up to 6 opened sources (this slice is the widest). Priority order: the METR study; Spracklen; one self-correction paper; the Claude Code best-practices doc; then your best two for sub-questions 1 and 4. Stop when your sub-questions are answered.
PRIVACY: Use no project names, private details or personal information in any search. You have no local file access and need none. Do not write any files.
OUTPUT, one block per source:
  - Title, author/org, date/version, direct link
  - Tier (T1 official standard, original practitioner guidance or peer-reviewed / T2 labeled preprint or public repo artifact / T3 vendor docs / T4 vendor claims, blogs, surveys) and claim type (Established / Observed)
  - Finding: the specific claim it supports, with the section/page where you saw it
  - Study card (empirical sources only): task · participants (count, experience) · tools/models + date · measure (speed / correctness / review effort / security / satisfaction) · result
  - Limitation: why this may not apply to the reader
  - Handbook implication: one sentence
THEN: list any gaps or conflicts you noticed and any leads you rejected and why. Keep the whole reply under 1100 words.
```

## Notes returned

Returned 2026-11-13 about 03:20 MST (Sonnet 5.5, resumed once after a usage-limit stop). 6 sources opened. **For the papers, only abstract or landing pages were read**, so section-level detail is absent. Saved as returned, lightly reformatted.

### Source 1: METR randomized controlled trial
- Becker, Rush, Barnes, Rein, "Measuring the Impact of Early-2025 AI on Experienced Open-Source Developer Productivity", arXiv 2507.09089 (v1 12 Jul 2025, v2 25 Jul 2025). https://arxiv.org/abs/2507.09089
- Tier: T2 (labeled preprint). Claim type: Observed.
- Finding (abstract page): when AI was allowed, task completion time increased by 19% (slower). Developers forecast a 24% reduction beforehand and still believed in a 20% reduction afterward. Economists predicted 39% and ML experts 38% reductions.
- Study card: task: real issues in mature open-source repos, 246 tasks, randomized per task to allow or disallow AI · participants: 16 developers, average 5 years of prior experience on their own repo, moderate AI experience · tools: Cursor Pro, Claude 3.5/3.7 Sonnet, early 2025 · measure: **speed only** on the page seen (no correctness, review or security measure seen) · result: slower, with perception opposite to measurement.
- Limitation: 16 people, their own large familiar repos, one tool generation. The authors do not rule out artifacts. A junior on unfamiliar code may differ.
- Handbook implication: do not trust your own feeling of being faster. Time a few tasks both ways.

### Source 2: Spracklen et al., "We Have a Package for You!"
- USENIX Security 2025 (arXiv 2406.10279, v1 Jun 2024, v3 2 Mar 2025). https://arxiv.org/abs/2406.10279
- Tier: T2 preprint (venue stated on the page). Claim type: Observed. **[Older-generation tools]** (2024 model set).
- Finding: 16 code LLMs, 2 prompt datasets, 576,000 code samples, 205,474 unique hallucinated package names. Average hallucination rate at least 5.2% for commercial models and 21.7% for open-source models. The mitigations tested reduce the rates.
- Study card: measure: correctness and supply-chain security (non-existent package names), not speed.
- Rust/crates.io: **not mentioned on the page.** Two languages were studied; the abstract page does not name them. Do not claim crate rates.
- Limitation: no Rust data. Generation only, not agent loops with a build or registry check.
- Handbook implication: verify that every dependency an agent adds exists and is the intended crate before merging. Treat the Rust rate as unmeasured.

### Source 3: Huang et al., "Large Language Models Cannot Self-Correct Reasoning Yet"
- ICLR 2024, arXiv 2310.01798 (v2 14 Mar 2024). https://arxiv.org/abs/2310.01798
- Tier: T1 (peer-reviewed). **[Older-generation tools]**
- Finding (abstract only): without external feedback, LLMs struggle to self-correct their reasoning, and performance sometimes degrades. Models, tasks and method details not seen.
- Limitation: reasoning tasks, not code.

### Source 4: Olausson et al., "Is Self-Repair a Silver Bullet for Code Generation?"
- ICLR 2024, arXiv 2306.09896 (v5 2 Feb 2024). https://arxiv.org/abs/2306.09896
- Tier: T1 peer-reviewed. **[Older-generation tools]**
- Finding (abstract page): Code Llama, GPT-3.5 and GPT-4 on HumanEval and APPS. Gains from self-repair are modest once the cost of repair is counted, inconsistent, and sometimes vanish. The authors hypothesize the bottleneck is the model's feedback on its own code. Feedback from a stronger model gives substantially larger gains. In a small study, GPT-4 given human feedback still falls short of human debugging.
- Study card: measure: correctness (benchmark pass rate).
- Limitation: benchmark problems, 2023 models.
- Handbook implication: this is the best evidence for "a different or stronger reviewer helps". A fresh-context same-model reviewer is **not** tested here.

### Source 5: Anthropic, "Best practices for Claude Code"
- https://code.claude.com/docs/en/best-practices (fetched 13 Nov 2026; no version date seen)
- Tier: T3 vendor docs (how features work only). Guidance, not evidence of effect.
- Findings:
  - Keep CLAUDE.md short: it is loaded every session, so every line costs attention. Include only what the agent cannot infer (test and lint commands, misleading code, off-repo decisions, things corrected twice).
  - Rules that cannot have exceptions go in harness configuration, not CLAUDE.md. The doc says CLAUDE.md rules are followed "almost every time".
  - Hooks: command and http hooks are deterministic; prompt and agent hooks ask another model, so they are not. A failure mode of "block" is available.
  - Skills: the name and description are visible each session; the full file is read only when the task calls for it.
  - Reviewer: a second session or a read-only subagent (Read, Grep, Glob) that has not seen how the code was written. The doc warns that a reviewer asked for problems will always find something, so tell it to skip style. A fresh session is recommended for building from a written SPEC.md.
- Handbook implication: keep instruction files short, put must-never rules in hooks and permissions, review in a separate read-only context.

### Source 6: Ogenrwot and Businge, "How AI Coding Agents Modify Code"
- MSR 2026 Mining Challenge, arXiv 2601.17581 (v3 6 Apr 2026). https://arxiv.org/abs/2601.17581
- Tier: T2. Observed. AIDev dataset: 24,014 merged agentic PRs vs 5,081 merged human PRs. Measures PR size, files, commits and description-to-diff similarity. Does **not** cover tests, duplication or reviewer effort.
- Handbook implication: PR-shape context only. Not usable for sub-question 4.

### Gaps and conflicts
- Sub-question 1: security (Perry et al.), DORA 2025 and GitClear not opened (budget). No review-effort measurement found. The METR page shows speed only.
- **Sub-question 4: no opened source measures agents editing or weakening tests, mirrored tests, or ignoring long instruction files.** Candidates seen only as titles or snippets, unverified: Haque et al., "An Empirical Study of Tests in Agentic Pull Requests" (MSR 2026); "More Code, Less Reuse"; Arabat and Sayagh on test coverage of AI PRs; Gonzalez and El Mezouar on clone genealogies. The vendor doc's "almost every time" is a vendor statement, not a measurement.
- Self-correction: Huang details unverified. **Evidence that a fresh context with the same model helps is absent** in what was read. Only vendor guidance asserts it.
- Agent Skills docs and AGENTS.md guidance not opened. The skill-loading claim comes only from the best-practices page.
- "Anthropic AI-native playbook": the search was not spent. Unconfirmed.

### Worker's answer to the key question
Keep dependency choice, test intent and acceptance criteria, and merge approval. Delegate bounded work only where a deterministic check (tests, build, hook) can catch errors. Evidence for delegating more is thin: METR shows no speedup for experts, and self-repair gains depend on external feedback.
