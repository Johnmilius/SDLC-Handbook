# Plan tests: what could break and what would catch it

**Decision this page supports:** what could break in this change, and which check would catch it?

A test plan is written **before the code changes**. It answers four questions: what could break, which check would catch it, why that check belongs at that layer, and what stays unverified. The [`test-plan` skill](../skills/test-plan/SKILL.md) applies this page. The [test-plan template](../templates/test-plan.md) is its output format.

**CSE 370:** [Unit 04](../cse370-prepare/unit04-testing.md) teaches boundary value analysis, equivalence partitioning, the test pyramid and the traceability matrix. [Unit 02](../cse370-prepare/unit02-requirements.md) teaches Given/When/Then acceptance criteria. This page puts them in the order you use them.

## Where the checks come from

<a id="tp-1"></a>**TP-1** Derive checks from the acceptance criteria, including the negative cases, not from the implementation's output. If you learn the expected value by running the code, the test can only confirm what the code already does. [Standard] ISTQB CTFL v4.0.1 (2024) §4.5.2 treats acceptance criteria as the test conditions and asks for positive and negative scenarios ([S15](sources.md#s15)).

<a id="tp-2"></a>**TP-2** Cover each input partition, valid and invalid. A partition is a group of inputs the code should treat the same way. [Standard] ISTQB CTFL v4.0.1 §4.2.1 ([S15](sources.md#s15)).

<a id="tp-3"></a>**TP-3** Test each boundary and its neighbors. For a range of 0 to 10, that is -1, 0, 10 and 11. [Standard] ISTQB CTFL v4.0.1 §4.2.2 ([S15](sources.md#s15)).

<a id="tp-13"></a>**TP-13** For "never" and "always" properties (never panics, a length bound always holds), propose a property test, which checks the property over many generated inputs. [Proposed]. No source on property testing was opened in the research.

## Order of work

<a id="tp-4"></a>**TP-4** Reproduce a bug with a failing test before fixing it. A test you have never seen fail may not be testing anything. [Proposed]. It fits regression testing in [Unit 05](../cse370-prepare/unit05-release-maintenance.md).

<a id="tp-5"></a>**TP-5** Pin current behavior with characterization tests before changing untested code. A characterization test records what the code does today, right or wrong, so a refactor cannot change it silently. [Proposed]. The term comes from Feathers, *Working Effectively with Legacy Code* (2004), which was not opened in the research.

## Where each check goes

<a id="tp-6"></a>**TP-6** Put each check at the lowest layer that can catch the failure, and name the integration check that proves the pieces connect. Unit tests cannot show that a saved value is ever read by the code that needs it. [Standard] ISTQB CTFL v4.0.1 §5.1.6 describes lower layers as small, isolated and fast, and higher layers as slower and less isolated ([S15](sources.md#s15)).

<a id="tp-7"></a>**TP-7** Test behavior through the public API so the test survives a refactor. A test that has to change when the implementation is restructured is a warning sign. [Established] *Software Engineering at Google* ch. 12, "Test via Public APIs" and "Strive for Unchanging Tests" ([S7](sources.md#s7)).

## Whether the tests can be trusted

<a id="tp-8"></a>**TP-8** Ask whether each test would fail if the code were broken. For Tier 3, check it with mutation testing on the changed code. [Established] Google eng-practices, "Tests": tests do not test themselves, and a person has to check they are valid ([S5](sources.md#s5)). The mutation-testing evidence is partial: a Google study reports that developers shown mutants wrote more tests, but only its abstract was read, by a research worker ([S17](sources.md#s17)).

<a id="tp-11"></a>**TP-11** Never change a test's expected value to make it pass. Stop and ask. [Observed] ImpossibleBench, arXiv 2510.20270v1 (Oct 2025): on tasks where the tests contradicted the spec, models often passed anyway by editing or special-casing the tests ([S3](sources.md#s3)). The tasks were built to be impossible, so this shows the behavior exists, not how often it happens in daily work.

<a id="tp-12"></a>**TP-12** Stop and ask when the spec is ambiguous or silent on a behavior a test would have to assert. [Observed] In ImpossibleBench §5.3, giving the agent a way to flag the task for a person cut test exploitation sharply for some models ([S3](sources.md#s3)). Applying that to ambiguous specs is [Proposed].

## How much, and what is left

<a id="tp-10"></a>**TP-10** Size the plan to the tier. A Tier 1 fix needs a failing regression test and its neighbors, in about 15 lines of plan. A Tier 3 change needs a rehearsal and a list of what CI cannot prove. [Standard] ISTQB CTFL v4.0.1 §5.2.3: product risk decides test scope, techniques and effort ([S15](sources.md#s15)). The 15-line figure is [Proposed]. Tiers are defined in [risk-tiers.md](risk-tiers.md).

<a id="tp-9"></a>**TP-9** Write down what stays unverified and what needs a real environment. [Standard] ISTQB CTFL v4.0.1 §1.3, principle 1: testing shows that defects are present, not that they are absent ([S15](sources.md#s15)).

## Worked examples

- [Case 1: a bounded config value over HTTP](../examples/case-1-max-retries.md) (new feature)
- [Case 2: title truncation](../examples/case-2-truncate-title.md) (bug fix in existing code)

## Related

[Quality bar](quality-bar.md) (QB-1, QB-2, QB-3, QB-4) · [Risk tiers](risk-tiers.md) · [Verify a change](verify-a-change.md) · [Working with agents](working-with-agents.md) · [Workflow](workflow.md) · [Sources](sources.md)
