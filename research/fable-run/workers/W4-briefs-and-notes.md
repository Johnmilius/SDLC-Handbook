# W4: Standards and compliance frameworks (RQ6)

Model: Sonnet 5.5. Brief written 2026-11-13 00:40 MST.

## Brief sent

```
ROLE: Research assistant for a software-engineering handbook. Return evidence notes, not prose.
QUESTION:
  1. For each standard in the list below: governing body, current edition/year, free or paywalled, the specific borrowable clauses or criteria with section numbers, authority type (consensus standard / audit or attestation framework / empirically tested), what one developer can take from it in a line, whether it is large-organization only, and any relevance to AI-assisted development.
  2. What evidence trail does a SOC 2 change-management review expect (AICPA Trust Services Criteria CC8.1 and related CC7.x), and what do the criteria say about separation of duties?
  3. NIST SP 800-218A: who does it target (model producers, AI system producers, acquirers)? Does it say anything usable by a developer who merely uses an AI coding assistant?
  4. ISO/IEC/IEEE 29119: what is the documented professional criticism of it?
  Key question: which criteria from recognized standards are worth citing in a one-person handbook, and what evidence trail would satisfy a SOC 2-style change-management review at small scale?
READER: A junior developer, solo or on a small team, maintaining real software (mainly Rust, some TypeScript or Python). Uses AI coding agents interactively and for bounded autonomous tasks.
STANDARDS LIST:
  Lifecycle: ISO/IEC/IEEE 12207; SWEBOK Guide v4 (IEEE Computer Society)
  Requirements: ISO/IEC/IEEE 29148 (characteristics of good requirements)
  Design: ISO/IEC/IEEE 42010; IEEE 1016
  Testing: ISO/IEC/IEEE 29119; ISTQB Foundation Level syllabus
  Quality: ISO/IEC 25010
  Secure development: NIST SP 800-218 (SSDF) and SP 800-218A; OWASP ASVS; OWASP SAMM; CISA Secure by Design
  Supply chain: SLSA; OpenSSF Scorecard
  Compliance: SOC 2 (AICPA Trust Services Criteria, CC7.x and CC8.x); ISO/IEC 27001 Annex A secure-development controls
  AI: NIST AI RMF and NIST AI 600-1; ISO/IEC 42001
SCOPE: what each standard defines, its edition, access and borrowable criteria. OUT OF SCOPE: whether practices improve outcomes (a standard is consensus, not proof); vendor compliance products.
SOURCE RULES: Open every source you report as OPENED (use WebSearch to find it and WebFetch to read it; if you have no such tools, say so at the top and stop). Never report a detail (number, section, date) you did not see on the page.
EXTRA RULES FOR STANDARDS:
  - Prefer the official source (iso.org, ieee.org, nist.gov, csrc.nist.gov, aicpa-cima.com, owasp.org, slsa.dev, securityscorecards.dev, cisa.gov, computer.org, istqb.org) over vendor compliance blogs. Vendor SOC 2 explainers are T4.
  - If a standard is paywalled, say so. Use only the official abstract, a free preview, or a public document derived from it, and state which one you read. Never paraphrase a clause you could not read as if you had.
  - Mark each standard's row OPENED (you read an official page) or NOT OPENED (edition seen only in a search listing, or recalled from general knowledge). Be strict about this mark.
BUDGET: Up to 8 opened sources. Priority order: (1) NIST SP 800-218 SSDF practices, (2) AICPA Trust Services Criteria CC8.1 text or the closest official or free rendering, (3) SLSA current spec levels, (4) OWASP ASVS current version, (5) ISO/IEC/IEEE 29148 official abstract or a free rendering of the requirement characteristics, (6) ISO/IEC 25010:2023 characteristics, (7) NIST SP 800-218A, (8) one of SWEBOK v4, ISTQB CTFL, or the 29119 criticism. Cover the remaining standards as NOT OPENED rows.
PRIVACY: Use no project names, private details or personal information in any search. You have no local file access and need none. Do not write any files.
OUTPUT: one compact block per standard with the fields in sub-question 1, then short answers to sub-questions 2 to 4, then gaps and rejected leads. Keep the whole reply under 1300 words.
```

## Notes returned

Returned 2026-11-13 about 03:20 MST (Sonnet 5.5, resumed once after a usage-limit stop). **6 pages opened.** Not opened: iso.org pages (HTTP 403), the AICPA criteria PDF (needs a free login; only the landing page was read), and the SSDF practice IDs (the landing page does not list them). Anything marked "recalled" was **not** read on a page and must be verified before citing. Saved as returned, lightly reformatted.

### Opened evidence
- **O1** https://csrc.nist.gov/pubs/sp/800/218/final : SP 800-218 v1.1, final dated 3 Feb 2022. Supersedes CSWP 13. Authors Souppaya, Scarfone, Dodson. Direct download and DOI offered. Practice groups not shown on this page.
- **O2** https://slsa.dev/spec/v1.0/levels : the page notes that v1.2 is current and marks v1.0 as retired. Only a Build track exists in v1.0. L0 none. L1 provenance exists. L2 hosted build platform that signs provenance. L3 hardened builds (builds isolated from each other, signing secret unreachable by build steps). The v1.2 text itself was not read.
- **O3** https://csrc.nist.gov/pubs/sp/800/218/a/final : SP 800-218A, "Secure Software Development Practices for Generative AI and Dual-Use Foundation Models: An SSDF Community Profile". Final 26 Jul 2024. Written for EO 14110. Extends SSDF v1.1 with AI-specific practices, tasks and recommendations. Audience: producers of AI models, producers of AI systems that use them, and acquirers of those systems.
- **O4** https://asvs.owasp.org : ASVS 5.0.0, published 30 May 2025. CC BY-SA 4.0. Requirement IDs are `<chapter>.<section>.<requirement>`; cite as `v5.0.0-1.2.x`. Levels and the full chapter list were not on the page.
- **O5** aicpa-cima.com Trust Services Criteria landing page: "2017 TSC (With Revised Points of Focus, 2022)". The PDF needs a free account. **No criteria text was visible, so no CC8.1 or CC7.x wording was read.**
- **O6** https://quality.arc42.org/articles/iso-25010-update-2023 (third-party, not official): the 2023 changes are Safety added; Usability renamed Interaction capability; Portability renamed Flexibility; new subcharacteristics Inclusivity, Self-descriptiveness, Resistance, Scalability; User engagement replaces UI aesthetics; Faultlessness replaces Maturity.

### Per-standard blocks

| Standard | Opened? | Edition | Access | Borrowable | Authority | One-developer take | Large-org only? | AI relevance |
|---|---|---|---|---|---|---|---|---|
| NIST SP 800-218 SSDF | OPENED (O1, landing page) | v1.1, Feb 2022 | Free | Groups PO/PS/PW/RV and IDs such as PW.7, PW.8, PS.1, RV.1 are **recalled, not seen** | Consensus guidance, not an audit | Checklist for review, testing, protecting code, handling vulnerabilities | No (some tasks are organizational) | Indirect, via 800-218A |
| NIST SP 800-218A | OPENED (O3, landing page) | Final Jul 2024 | Free | PDF body not read | Consensus guidance (community profile) | Not determined | Targets AI model and system producers and acquirers | Direct, but for producers |
| SLSA | OPENED (O2, retired v1.0 page) | v1.2 current (not read) | Free | Build L1, L2, L3 | Specification with self-assessed levels | Reach Build L1 to L2 by building on hosted CI that emits signed provenance | No | None seen |
| OWASP ASVS | OPENED (O4) | 5.0.0, May 2025 | Free, CC BY-SA 4.0 | Requirement IDs by chapter. Levels recalled, not seen | Community consensus, not an audit | Pick requirements by chapter as a review checklist for web/API code | No | None seen |
| ISO/IEC 25010 | NOT OPENED on iso.org (403). Partial via O6 | 2023 | Paywalled | Characteristic names only, from snippets and O6 | Consensus standard | A vocabulary for naming non-functional requirements | No | None seen |
| ISO/IEC/IEEE 29148 | NOT OPENED | 2018 (recalled) | Paywalled | Requirement characteristics recalled, not seen | Consensus standard | Check each requirement for ambiguity and testability | No | None known |
| ISO/IEC/IEEE 12207, 42010; IEEE 1016; ISO/IEC/IEEE 29119; ISTQB CTFL; SWEBOK v4; OWASP SAMM; CISA Secure by Design; OpenSSF Scorecard; ISO/IEC 27001 Annex A; NIST AI RMF and AI 600-1; ISO/IEC 42001 | NOT OPENED | Not verified | Not verified | **Cite nothing from this worker for these** | n/a | n/a | n/a | n/a |

### Sub-question 2 (SOC 2 change management)
**Not read.** The criteria text was behind a login. From recall only: CC8.1 covers authorizing, designing, developing, testing, approving and implementing changes; CC7.x covers detecting and responding to anomalies and incidents. Separation of duties: nothing read. A small-scale evidence trail is therefore unsupported by anything this worker read. No vendor SOC 2 explainer was used.

### Sub-question 3 (SP 800-218A)
Targets AI model producers, AI system producers and acquirers (O3). It is a community profile that adds model-development practices to SSDF and does not replace it. Whether it says anything usable by someone who only *uses* an AI coding assistant: not determined (PDF body not read).

### Sub-question 4 (29119 criticism)
**Not researched.** Recall only: the 2014-era "Stop 29119" petition and criticism from context-driven testing figures arguing that it imposes heavy documentation and is not evidence-based. No source opened.

### Gaps and next steps listed by the worker
1. The AICPA criteria PDF needs a free login.
2. Download the SP 800-218 PDF for practice IDs.
3. Read the SLSA v1.2 levels.
4. ISO pages returned 403.
5. Find a source for the 29119 criticism.
6. Read the 800-218A PDF for coding-assistant relevance.

Rejected leads: vendor SOC 2 blogs and ISO 25010 aggregator sites were not opened.
