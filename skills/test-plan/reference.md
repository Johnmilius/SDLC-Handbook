# Rules the `test-plan` skill may cite

Pinned from SDLC-handbook v1.0, 2026-11-13. This file travels with the skill, so rule links work in any repository. The full text of each rule, with links to its source, is in the handbook at the path shown. If the handbook is ever published, add its base URL to this line.

Labels: **[Standard]** defined by a named standard (consensus, not proof that it works). **[Established]** long-standing practitioner guidance. **[Observed]** an empirical result. **[Proposed]** the handbook's own synthesis.

## Test planning (handbook/plan-tests.md)

<a id="tp-1"></a>**TP-1** Derive checks from the acceptance criteria, including the negative cases, not from the implementation's output. *[Standard] ISTQB CTFL v4.0.1 (2024) §4.5.2.*

<a id="tp-2"></a>**TP-2** Cover each input partition, valid and invalid. *[Standard] ISTQB CTFL v4.0.1 §4.2.1.*

<a id="tp-3"></a>**TP-3** Test each boundary and its neighbors. *[Standard] ISTQB CTFL v4.0.1 §4.2.2.*

<a id="tp-4"></a>**TP-4** Reproduce a bug with a failing test before fixing it. *[Proposed].*

<a id="tp-5"></a>**TP-5** Pin current behavior with characterization tests before changing untested code. *[Proposed]. The term is from Feathers (2004).*

<a id="tp-6"></a>**TP-6** Put each check at the lowest layer that can catch the failure, and name the integration check that proves the pieces connect. *[Standard] ISTQB CTFL v4.0.1 §5.1.6.*

<a id="tp-7"></a>**TP-7** Test behavior through the public API so the test survives a refactor. *[Established] Software Engineering at Google, ch. 12.*

<a id="tp-8"></a>**TP-8** Ask whether each test would fail if the code were broken. For Tier 3, check it with mutation testing on the changed code. *[Established] Google eng-practices, "Tests". The mutation-testing evidence is partial.*

<a id="tp-9"></a>**TP-9** Write down what stays unverified and what needs a real environment. *[Standard] ISTQB CTFL v4.0.1 §1.3, principle 1.*

<a id="tp-10"></a>**TP-10** Size the plan to the tier. *[Standard] ISTQB CTFL v4.0.1 §5.2.3.*

<a id="tp-11"></a>**TP-11** Never change a test's expected value to make it pass. Stop and ask. *[Observed] ImpossibleBench, arXiv 2510.20270v1 (2025).*

<a id="tp-12"></a>**TP-12** Stop and ask when the spec is ambiguous or silent on a behavior a test would have to assert. *[Observed] ImpossibleBench §5.3 for flagging conflicts; otherwise [Proposed].*

<a id="tp-13"></a>**TP-13** For "never" and "always" properties (never panics, a bound always holds), propose a property test. *[Proposed].*

## Risk tiers (handbook/risk-tiers.md)

<a id="rt-1"></a>**RT-1** Depth scales with risk (likelihood times impact), not with the size of the diff. *[Standard] ISTQB CTFL v4.0.1 §5.2.3; NIST SP 800-218 v1.1 §2.*

<a id="rt-2"></a>**RT-2** No tier skips a stage. A tier changes how deep a stage goes. *[Established] CSE 370 Unit 07.*

<a id="rt-3"></a>**RT-3** An escalation trigger raises the tier. Only the developer lowers one, and writes down why. *[Proposed].*

| Tier | Typical change | Test plan depth |
|---|---|---|
| 1, low risk | Small bug fix | A regression test that fails before the fix, plus nearby boundary cases |
| 2, normal | Feature | Checks per criterion at the right layer; the integration check named |
| 3, high risk | Hard to reverse, or a trigger fired | Tier 2, plus a rehearsal on production-like data and a list of what CI cannot prove |

**Raise to Tier 3 if the change:** touches authentication, authorization, sessions, secrets or cryptography; migrates or deletes stored data, or changes a schema; touches money; cannot be undone by redeploying the last-good version; adds or changes `unsafe` code or FFI.

**Raise to at least Tier 2 if the change:** changes a public API, a file format or a wire contract; adds or upgrades a dependency; touches concurrency; changes code that has no tests; is larger than about 400 changed lines or 10 files.

<a id="rt-4"></a>**RT-4** The agent may propose a tier. The developer decides. *[Proposed].*

## Quality bar (handbook/quality-bar.md)

<a id="qb-2"></a>**QB-2** Tests would fail if the code were broken. They check behavior through the public API, not the implementation's current output. *[Established] Google eng-practices, "Tests"; Software Engineering at Google, ch. 12.*

<a id="qb-3"></a>**QB-3** No test is deleted, ignored or given a new expected value without the developer's explicit approval. *[Observed] ImpossibleBench (2025).*

<a id="qb-8"></a>**QB-8** Every new or upgraded dependency exists, is the one intended, is justified, and passes policy. *[Observed] Spracklen et al., USENIX Security 2025 (measured for Python and JavaScript, not Rust). [Standard] NIST SP 800-218 v1.1 PW.4.4.*

<a id="qb-13"></a>**QB-13** What was not verified is written down. *[Standard] ISTQB CTFL v4.0.1 §1.3, principle 1.*
