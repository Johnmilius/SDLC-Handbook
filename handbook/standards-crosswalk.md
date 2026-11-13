# Standards crosswalk: what backs each practice

**Decision this page supports:** Which recognized standard backs this practice, and what evidence do I already have?

<a id="sc-1"></a>**SC-1** A standard records expert consensus or defines what an auditor checks. It is not evidence that a practice improves outcomes. [Proposed]

<a id="sc-2"></a>**SC-2** Cite a clause only if its text was read. Standards that were not read are listed as "not read" with no clause. [Proposed]

<a id="sc-3"></a>**SC-3** One person cannot provide separation of duties, and an AI reviewer is not a second trusted person. [Proposed], from the two clauses cited under "Separation of duties" below.

## Standards that were read

The clause text of these four was read from the published documents on 2026-11-13 ([S13](sources.md#s13), [S14](sources.md#s14), [S15](sources.md#s15), [S16](sources.md#s16)).

| Standard (edition) | Authority type | Clauses to borrow | Handbook page it backs | Solo take |
|---|---|---|---|---|
| NIST SP 800-218 SSDF (v1.1, 2022) [S13](sources.md#s13) | [Standard] consensus guidance, not an audit | §2 (risk-based, not a checklist); PO.3.3 (tools leave an audit trail); PW.4.4 (verify third-party components); PW.7 (review and/or analyze code); PW.8 (test executable code); PS.1 (protect code); PS.3 (archive releases); RV.1 (find vulnerabilities continuously) | [quality-bar.md](quality-bar.md), [verify-a-change.md](verify-a-change.md), [workflow.md](workflow.md) (release) | [Standard] Your CI log and PR history already are the PO.3.3 audit trail ([S13](sources.md#s13)). The role and training tasks in PO.2 are organizational `[Large-org only]`. |
| NIST SP 800-218A (2024) [S14](sources.md#s14) | [Standard] consensus guidance | Scope statement: human-written and AI-generated code are not distinguished, and all code is evaluated before use | [quality-bar.md](quality-bar.md), [working-with-agents.md](working-with-agents.md) | [Standard] Review agent-written code to the same bar as your own ([S14](sources.md#s14)). The profile targets AI model and system producers, not users of coding tools. |
| ISTQB CTFL syllabus (v4.0.1, 2024) [S15](sources.md#s15) | [Standard] consensus syllabus from a certification body | §1.3 (principles); §1.5.3 (independence of testing); §4.2.1 and §4.2.2 (partitions, boundaries); §4.5.2 (acceptance criteria); §5.1.3 (exit criteria); §5.1.6 (test pyramid); §5.2.3 and §5.2.4 (product risk analysis and control) | [plan-tests.md](plan-tests.md), [risk-tiers.md](risk-tiers.md), [verify-a-change.md](verify-a-change.md) | [Standard] Names and section numbers for techniques from CSE 370 Unit 04 ([S15](sources.md#s15)). |
| SLSA (v1.2) [S16](sources.md#s16) | [Standard] specification, self-assessed | Build L1 (provenance exists), L2 (hosted build signs provenance), L3 (hardened); Source L1 to L4 (L4 is two-party review) | [workflow.md](workflow.md) (release) | [Standard] Build on hosted CI and keep provenance: Build L1 to L2 ([S16](sources.md#s16)). Build L3 and Source L4 need more than one person `[Large-org only]`. |

[Standard] ISTQB §5.2.3 says product risk (likelihood and impact) decides test scope, techniques and effort, which backs scaling by risk in [risk-tiers.md](risk-tiers.md) ([S15](sources.md#s15)). SSDF §2 says to apply practices by risk and not as a checklist, at any organization size ([S13](sources.md#s13)).

## Not read, so not cited

No clause is cited for any row here. The reasons are what happened when the research tried to open each one.

| Standard | Why not read | What that means |
|---|---|---|
| OWASP ASVS 5.0.0 (May 2025) | Only the landing page was read: edition and license (free, CC BY-SA 4.0) | No requirement is cited. It is a candidate review checklist for v2. |
| SOC 2 / AICPA Trust Services Criteria | The free PDF is behind a login | No criterion, including CC8.1 and CC7.x, is described. SOC 2 is an attestation report by an auditor against these criteria, not a certification or an engineering standard. |
| ISO/IEC/IEEE 29148, 12207, 42010, 29119; IEEE 1016 | Paywalled | Not used. CSE 370 Unit 02 criteria are used for requirements instead. |
| ISO/IEC 25010 (2023) | Paywalled (HTTP 403) | Not used in v1. |
| SWEBOK v4, OWASP SAMM, CISA Secure by Design, OpenSSF Scorecard | Not opened | Candidates for v2. SAMM leans large-org. |
| ISO/IEC 27001 Annex A, ISO/IEC 42001, NIST AI RMF and AI 600-1 | Not opened (ISO is paywalled) | Not used. Management systems are `[Large-org only]`. |

## Audit-evidence map

**The SOC 2 criteria text was not read. This map does not say what SOC 2 requires.** It lists record-keeping expectations from the standards that were read (SSDF and SLSA), the everyday artifact that already provides each one, and the gap. Add a SOC 2 column only after you have read the free PDF yourself.

| Expectation that was read | Clause | Everyday artifact | Gap for a solo developer |
|---|---|---|---|
| [Standard] Tools produce an audit trail of development practices ([S13](sources.md#s13)) | SSDF PO.3.3 | CI run logs, PR history, commit history | None, if CI logs are kept |
| [Standard] Code is reviewed and/or analyzed; issues are recorded and triaged ([S13](sources.md#s13)) | SSDF PW.7.1, PW.7.2 | Clippy and review findings in the PR; issues for deferred items | The reviewer is the author. See below |
| [Standard] Testing is scoped, designed, performed, and results documented ([S13](sources.md#s13)) | SSDF PW.8.2 | The test plan; the CI test log attached to the PR | None |
| [Standard] Third-party components are verified against your requirements ([S13](sources.md#s13)) | SSDF PW.4.4 | `cargo deny` log; the dependency-approval line in the PR | None |
| [Standard] Each release is archived with integrity and provenance data ([S13](sources.md#s13), [S16](sources.md#s16)) | SSDF PS.3.1; SLSA Build L1 to L2 | Tagged release; provenance from hosted CI | Build L3 needs a hardened platform `[Large-org only]` |
| [Standard] Code is protected from unauthorized change ([S13](sources.md#s13), [S16](sources.md#s16)) | SSDF PS.1.1; SLSA Source L1 to L3 | Version control with protected branches | None |
| [Standard] Changes to protected branches get two-party review ([S16](sources.md#s16)) | SLSA Source L4 | Nothing | Cannot be closed by one person |
| Significant decisions are recorded | no clause read | ADRs | Not applicable |

## Separation of duties

One person cannot be both author and independent approver, and no workflow change fixes that. An AI reviewer is not a second trusted person: [Standard] SLSA v1.2 Source L4 asks for two-party review ([S16](sources.md#s16)), and [Standard] ISTQB CTFL v4.0.1 §1.5.3 describes independence between people, from the author up to outside the organization ([S15](sources.md#s15)). The research found no standard that accepts an AI review as a control, so this handbook does not claim one. What a solo developer can do:

- [Proposed] Keep the evidence a second person would look at: criteria, test plan, CI logs, review findings, the decision.
- [Proposed] Let time pass, then re-read Tier 3 diffs in a fresh session before merging.
- [Proposed] Bring in a human reviewer for Tier 3 changes when a customer or auditor requires it, and say plainly in any questionnaire that routine changes are self-approved.

## Where standards and practice disagree

- **Read every line, or focus on risk?** [Established] Google's review guidance says to look at every line you are asked to review ([S5](sources.md#s5)). [Standard] ISTQB §5.2.3 says risk decides depth ([S15](sources.md#s15)). They fit together: risk decides how deep you go, not which lines you skip.
- **Standards assume more than one person.** [Standard] SLSA Source L4 and ISTQB's higher independence levels cannot be met alone ([S16](sources.md#s16), [S15](sources.md#s15)). This handbook states the gap instead of hiding it.
- **Heavy documents versus lean artifacts.** [Established] Nygard argues that large documents are not kept current ([S8](sources.md#s8)). The formal documentation standards (IEEE 1016, ISO/IEC/IEEE 29119) were not read, so no fair comparison is made. A criticism of 29119 from the testing community came up during the research but was never sourced. Treat it as an unverified lead only.

**CSE 370:** [Unit 04](../cse370-prepare/unit04-testing.md) covers traceability between requirements and tests in both directions, and names the audit trail as one of its benefits.

## Related

- [quality-bar.md](quality-bar.md)
- [risk-tiers.md](risk-tiers.md)
- [plan-tests.md](plan-tests.md)
- [verify-a-change.md](verify-a-change.md)
- [working-with-agents.md](working-with-agents.md)
- [workflow.md](workflow.md)
- [sources.md](sources.md)
