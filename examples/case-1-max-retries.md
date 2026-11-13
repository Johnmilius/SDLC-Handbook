# Case 1: a bounded config value over an HTTP-style API

**Kind:** new feature, Tier 2 (trigger: public API contract). **Code:** [handbook-examples/src/config.rs](handbook-examples/src/config.rs). **Exercises:** [TP-1](../handbook/plan-tests.md#tp-1), [TP-2](../handbook/plan-tests.md#tp-2), [TP-3](../handbook/plan-tests.md#tp-3), [TP-6](../handbook/plan-tests.md#tp-6) and [TP-9](../handbook/plan-tests.md#tp-9), through the [`test-plan` skill](../skills/test-plan/SKILL.md).

## The request

> Add a `max_retries` setting to the job service. `PUT /config/max_retries` with body `{"value": <integer>}` sets it. Valid range is 0 to 10 inclusive. Default is 3 when it has never been set. Out-of-range or malformed values return HTTP 422 with body `{"error": "max_retries_out_of_range"}`. The retry worker must use the saved value for jobs it starts after the change.

A first implementation exists in `src/config.rs`. Its three tests pass.

## Planted defects (deliberate)

| # | Defect | Where | Why the existing tests miss it |
|---|---|---|---|
| 1 | The upper bound is off by one: 10 is rejected | `set_max_retries`, `value < 10` | The tests try 5 and 11, never 10 |
| 2 | A negative value is accepted and wraps: -1 is stored as 255 | `set_max_retries`, `value as u8` | No negative input is tested |
| 3 | Malformed input (`3.5`, `"3"`, `null`, a missing field) returns 400 `bad_request`, not the 422 the spec requires | `put_max_retries` | No test goes through the handler |
| 4 | The worker never reads the saved value. It uses the constant `MAX_RETRIES` | `RetryWorker::run` | The worker test passes because the constant equals the default |
| 5 | The default of 3 is not tested | `Config::new` | No test reads the value before setting it |

## Expected skill output **[Illustrative]**

```
## Test plan: add a max_retries setting (0 to 10, default 3) settable over PUT /config/max_retries
Tier proposed: 2. Triggers: public API contract.  (You decide.)
Acceptance criteria:
  AC-1 Never set: the worker retries at most 3 times.
  AC-2 PUT {"value": n} with n in 0..=10 returns 200 and stores n.
  AC-3 Out-of-range or malformed values return 422 {"error": "max_retries_out_of_range"}; the value is unchanged.
  AC-4 After a set, jobs started later use the saved value.

### Findings
1. Finding: the upper bound is rejected.
   Evidence: src/config.rs, set_max_retries uses `value < 10`; the spec says "0 to 10 inclusive".
   Proposed check: set_max_retries accepts 0 and 10, rejects -1 and 11 (unit) [to write]
   Why it matters for this change: the documented maximum cannot be set.
   Handbook: TP-3 (reference.md#tp-3)
2. Finding: a negative value passes validation and is cast to u8.
   Evidence: src/config.rs, `value as u8` after `value < 10`.
   Proposed check: -1 returns OutOfRange and the stored value is unchanged (unit) [to write]
   Why it matters for this change: -1 would become 255 retries.
   Handbook: TP-2 (reference.md#tp-2)
3. Finding: malformed bodies do not follow the 422 contract.
   Evidence: src/config.rs, put_max_retries returns 400 "bad_request" when parse_value fails; AC-3 requires 422.
   Proposed check: send 3.5, "3", null and {} through put_max_retries; assert status 422 and the exact body (integration, handler level) [to write]
   Why it matters for this change: clients match on the status and the error code.
   Handbook: TP-6 (reference.md#tp-6)
4. Finding: the worker does not read the saved value.
   Evidence: src/config.rs, RetryWorker::run compares against the constant MAX_RETRIES; nothing reads Config.
   Proposed check: set the value to 1, run a failing job, assert 2 attempts (integration) [to write]
   Why it matters for this change: every unit test can pass while the feature does nothing.
   Handbook: TP-6 (reference.md#tp-6)
5. Finding: the default is untested.
   Evidence: the tests in src/config.rs never read max_retries before a set.
   Proposed check: Config::new().max_retries() == 3 (unit) [to write]
   Why it matters for this change: AC-1.
   Handbook: TP-1 (reference.md#tp-1)

### Order of work
1. Write checks 1 to 5 and watch 1 to 4 fail. 2. Fix the code. 3. Rerun.

### Not verified by this plan
- Jobs already running when the value changes.
- A write during a read under load.

### Questions for you (blocking)
1. Should a change apply to jobs already in flight? The spec says only "jobs it starts after the change".
```

## What to notice

- **It starts from the spec.** Findings 1 and 3 come from comparing the code with the criteria. Tests written by reading the code would encode `value < 10` as correct.
- **It places checks at layers.** Findings 3 and 4 cannot be caught by a unit test of `set_max_retries`.
- **It says what is left.** The last two sections are the difference between "the tests pass" and "I know what I verified".
- A plain request such as "write tests for this" usually produces test code straight away, with no tier, no questions and no statement of what is unverified.
