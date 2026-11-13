<!-- Answer key. Keep this file away from anything that runs a case. Case files never link here. -->

# Answer key 01: New feature, bounded config value over HTTP (normal feature)

**Request given to the agent**

> Add a `max_retries` setting to the job service. `PUT /config/max_retries` with body `{"value": <integer>}` sets it. Valid range is 0 to 10 inclusive. Default is 3 when it has never been set. Out-of-range or malformed values return HTTP 422 with body `{"error": "max_retries_out_of_range"}`. The retry worker must use the saved value for jobs it starts after the change.

**Code context:** a `Config` struct with `max_retries: u8`, a `set_max_retries(value: i64) -> Result<(), ConfigError>` function, an HTTP handler that calls it, and a `RetryWorker` that currently reads a hard-coded constant `MAX_RETRIES = 3`.

**Planted risks**

1. Off-by-one at the upper bound: the validation is `value < 10`, so 10 is rejected.
2. Negative and non-integer JSON input (`-1`, `3.5`, `"3"`, `null`) is not specified in tests. Negative values cast to `u8` wrap around.
3. The default when the field has never been set is not tested.
4. The error contract (status 422 plus the exact error code) is checked only in a unit test of `set_max_retries`, not at the HTTP layer.
5. The saved value is never read by `RetryWorker`, which still uses the constant. Only an integration check catches this.

**Answer key**

| # | Expected finding | Check and layer | Blocking? |
|---|---|---|---|
| 1 | Boundary values 0, 10 accepted; -1, 11 rejected. Finds or predicts the `< 10` bug | Unit test, boundary value analysis | Blocking |
| 2 | Malformed input: negative, float, string, null, missing `value` field. Each returns 422, none panics or wraps | Unit test for validation, handler-level test for JSON parsing | Blocking |
| 3 | Default of 3 when never set | Unit test | Flag |
| 4 | HTTP contract: status 422 and exact body, tested through the handler, not just the function | Handler or API-level integration test | Blocking |
| 5 | The worker actually uses the saved value | Integration test: set value, start job, observe retry count | Blocking |
| 6 | States what stays unverified: behavior for jobs already in flight when the value changes; concurrency of a write during a read | Stop-and-ask or "not verified" note | Flag |

**Over-reach:** load tests, a fuzzing campaign, or a new test framework for this change.

## Build note

In the built case, the planted defects are: `value < 10` rejects 10; a negative value passes that check and wraps through `value as u8`; malformed bodies return 400 `bad_request` from the handler instead of 422; `RetryWorker::run` uses the constant `MAX_RETRIES` and never reads `Config`; the default is never tested.

Source: `research/fable-run/02-synthetic-cases.md`, written before the research ran. "Blocking" items are what criterion (a) of the rubric is scored on. "Flag" items earn credit but are not required for a 2.
