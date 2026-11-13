# Verify a change: what each check proves

**Decision this page supports:** Is the evidence enough for this tier?

**CSE 370:** [Unit 04](../cse370-prepare/unit04-testing.md) supports the idea that testing reduces risk without proving correctness and that simulated environments need real-environment tests. [Unit 07](../cse370-prepare/unit07-sdlc-revisited.md) supports the point that no phase is skipped and that automated checks and static analysis give early feedback.

Two points before the table:

- Cheap, deterministic checks run first, so review attention goes to code that already builds and passes. [Proposed]
- This order is [Proposed]. No source read for this handbook tests it against running AI review in parallel.

Passing checks produce evidence, not proof. [Standard] ISTQB CTFL v4.0.1, §1.3, principles 1 and 7 ([S15](sources.md#s15)). Tier names and triggers are defined in [risk-tiers.md](risk-tiers.md). The checks map to rules in [quality-bar.md](quality-bar.md): VC-1 to VC-3 enforce QB-9, and VC-2 also enforces QB-5; VC-4 enforces QB-3; VC-5 enforces QB-8; VC-6 checks QB-2; VC-7 and VC-8 cover the rules that need judgment.

| Rule | Check | Tier | What it can show | What it cannot show |
|---|---|---|---|---|
| <a id="vc-1"></a>**VC-1** | `cargo fmt --check`, `cargo check` [Proposed] | All | The code is formatted and compiles | Anything about behavior |
| <a id="vc-2"></a>**VC-2** | `cargo clippy -- -D warnings`, plus a few chosen `restriction` lints such as `clippy::unwrap_used`. [T3 docs] Clippy lint groups, read by a research worker only, not re-opened by the lead (claim-check C29, "partly") ([S18](sources.md#s18)) | All | Known bad patterns are absent | That the logic is right |
| <a id="vc-3"></a>**VC-3** | `cargo test` | All | The tests pass | That the tests check the right behavior, or that users get the right result. [Established] Google eng-practices, "Tests" ([S5](sources.md#s5)) |
| <a id="vc-4"></a>**VC-4** | Test-integrity check: list changed or deleted tests, changed expected values, new `#[ignore]`. [Observed] Models told to make tests pass sometimes edit or special-case the tests, on benchmark tasks built to be impossible ([S3](sources.md#s3)) | All | Whether the oracle moved during the change | Whether the move was right. The developer decides |
| <a id="vc-5"></a>**VC-5** | `cargo deny check` (advisories, bans, licenses, sources) and confirmation of any new dependency. [T3 docs] cargo-deny checks, read by a research worker only (claim-check C30, "partly") ([S18](sources.md#s18)) | All | Dependencies meet policy; a new name is on the allowlist | That a dependency is the one you meant. [Observed] Invented package names are measured for Python and JavaScript and unmeasured for Rust ([S2](sources.md#s2); claim-check C25, C26) |
| <a id="vc-6"></a>**VC-6** | `cargo mutants` on changed files. [T3 docs] ([S18](sources.md#s18)) | Tier 2 optional, Tier 3 required | Places where the code can change without a test failing | Missing requirements. A surviving mutant is a prompt to look, not a defect count |
| <a id="vc-7"></a>**VC-7** | Independent AI review in a fresh context. Inputs: acceptance criteria, diff, test results, not the author's conversation. [T3 docs] Claude Code best practices ([S10](sources.md#s10)) | Tier 2 and 3 | Mismatches with the spec, unhappy paths, tests that restate the implementation | It is not a control. Whether a same-model reviewer catches more than the author is untested (claim-check C34). Expect some false findings |
| <a id="vc-8"></a>**VC-8** | Human review of every line, deepest where the triggers point. [Established] Google eng-practices, "Every Line" ([S5](sources.md#s5)) | All, depth by tier | Design fit, simplicity, whether the change should exist | Runtime behavior in the real environment |
| <a id="vc-9"></a>**VC-9** | Real-environment checks: real hardware, production-like data, real users. [Established] [CSE 370 Unit 04](../cse370-prepare/unit04-testing.md) | When CI cannot reach it | What simulation and CI cannot | Everything else |

Caveats to keep in view:

- The Clippy and cargo-deny documentation (VC-2, VC-5) was read by a research worker only. Treat it as a description of how the tools work, not as a checked result. [T3 docs] ([S18](sources.md#s18))
- Independent AI review (VC-7) is an inexpensive extra filter. It does not replace the human review in VC-8, and Tier 3 needs a line-by-line human review. [Proposed]
- A reviewer should ask whether the tests would fail if the code were broken, because tests do not validate themselves. [Established] Google eng-practices, "Tests" ([S5](sources.md#s5))
- Running tests, lints and static analysis on every change gives early feedback. [Established] [CSE 370 Unit 07](../cse370-prepare/unit07-sdlc-revisited.md)
- Review and analysis of code, testing of executable code, and verification of third-party components are named practices. [Standard] NIST SP 800-218 v1.1, PW.7, PW.8, PW.4.4 ([S13](sources.md#s13))

Keep a written list of real-environment checks (VC-9) for each project, because CI and simulation cannot cover them. Review the list when a change touches hardware, data or users that tests do not reach.

## Related

- [quality-bar.md](quality-bar.md)
- [risk-tiers.md](risk-tiers.md)
- [plan-tests.md](plan-tests.md)
- [working-with-agents.md](working-with-agents.md)
- [workflow.md](workflow.md)
- [sources.md](sources.md)
- [../templates/pr-description.md](../templates/pr-description.md)
