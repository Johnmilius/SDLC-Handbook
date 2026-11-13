## Test plan: add a `max_retries` setting (PUT /config/max_retries, range 0 to 10, default 3) that the retry worker honors
Tier proposed: 2. Triggers: changes a public API / wire contract (new endpoint, fixed 422 body); touches concurrency (`AtomicU8` shared with the worker). No Tier 3 trigger fired on what I read, but see question 3 ("saved" may mean persisted storage, which would raise it).  (You decide.)

Acceptance criteria (1-5 are from the task; 6-7 are drafted, please confirm):
1. `PUT /config/max_retries` with `{"value": n}` for integer n in 0..=10 sets the value (0 and 10 are valid).
2. If never set, the value is 3.
3. An out-of-range integer returns 422 with body `{"error": "max_retries_out_of_range"}`.
4. A malformed value returns the same 422 and body.
5. The retry worker uses the saved value for jobs it starts after the change.
6. (drafted) Given the saved value is 5, when a PUT of 11 (or -1) is rejected, then the value is still 5.
7. (drafted) Given max_retries = n, when a job always fails, then the worker makes n + 1 attempts (first try plus n retries), including n = 0 (1 attempt) and n = 10 (11 attempts).

Not read: any caller of `Config`, `RetryWorker` or `MAX_RETRIES` outside `src/config.rs`, any HTTP routing layer, and the output of `cargo test`. I only had the task text and `src/config.rs`.

### Findings
1. Finding: The upper bound is off by one. 10 is valid per the spec but is rejected.
   Evidence: src/config.rs:32 `if value < 10` (comment on the function says "0 to 10 inclusive"); existing test `set_rejects_value_above_range` only tries 11, so it cannot see this.
   Proposed check: `set_max_retries(10)` is `Ok(())` and `max_retries() == 10`; `set_max_retries(9)` is Ok; `set_max_retries(11)` is `Err(OutOfRange)` (this last one exists). (unit) [exists: `set_rejects_value_above_range` for 11 | to write for 10 and 9]
   Why it matters for this change: the top of the range is the boundary users are most likely to hit, and today it returns 422 for a legal value.

2. Finding: Negative values are accepted and wrap. `-1 < 10` is true, and `-1 as u8` stores 255.
   Evidence: src/config.rs:32-33 `if value < 10 { self.max_retries.store(value as u8, ...) }`. No lower-bound comparison exists. Existing tests never use a negative number.
   Proposed check: a std-only sweep test over `-1000..=1000` plus `i64::MIN` and `i64::MAX`. Each value is accepted if and only if it is in 0..=10, and `max_retries()` is in 0..=10 after every call. Also `set(0)` gives 0, and `set(-1)` is `Err(OutOfRange)` with the previous value unchanged. (unit) [to write]
   Why it matters for this change: a negative PUT would silently set 255 retries, the opposite of the intended limit, and criterion 6 is the guard.

3. Finding: The worker ignores the saved value. It reads the constant `MAX_RETRIES` (3), not `Config`, so the setting has no effect on retries.
   Evidence: src/config.rs:8 `pub const MAX_RETRIES: u8 = 3;` and src/config.rs:94 `attempts > u32::from(MAX_RETRIES)`. `RetryWorker` is a unit struct with no access to `Config` (src/config.rs:~87). Existing test `worker_stops_after_retry_limit` asserts 4 with the hardcoded constant, so it passes whatever is saved.
   Proposed check: through the public API, `put_max_retries(&config, r#"{"value": N}"#)` returns 200, then a worker started afterward with an always-failing job makes N + 1 attempts, for N = 0, 1 and 10. Also change the value between two worker runs and check each run uses the value current at its start. (integration: handler + Config + worker) [to write]. This is the one integration check that proves the real pieces connect.
   Why it matters for this change: this is criterion 5, the point of the feature, and every unit test of the setter passes while it is broken.

4. Finding: Malformed input returns 400 `bad_request`, not the specified 422 `max_retries_out_of_range`.
   Evidence: src/config.rs:59-60 (`status: 400`, body `bad_request`) returned when `parse_value` gives `None`; spec sentence "Out-of-range or malformed values return HTTP 422 with body ...".
   Proposed check: a table test through `put_max_retries`. Bodies `{"value": "abc"}`, `{"value": "5"}`, `{"value": null}`, `{"value": 3.5}`, `{}`, `` (empty) and `{"value": 99999999999999999999}` each give exactly `status == 422` and body `{"error": "max_retries_out_of_range"}`. Each of these also leaves the saved value unchanged. (unit on the handler) [to write]. Expected values come from the spec text, not from current output.
   Why it matters for this change: clients that follow the documented contract will not handle a 400, and this is the negative case for criterion 4.

5. Finding: `parse_value` is a string search, not a JSON parser, so it may accept bodies that are not valid JSON and mis-handle valid ones.
   Evidence: src/config.rs:77-82 (`split_once("\"value\"")`, then `split_once(':')`, then split on `,` or `}`). For example `{"value": 5` (no closing brace) would parse as 5, and the same would happen for `{"value": 5} garbage`.
   Proposed check: through the handler, `{"other": 1, "value": 5}` is accepted as 5 (valid, key order differs); a truncated body `{"value": 5` and a body with trailing text get the response chosen in question 1. (unit on the handler) [to write, after question 1]
   Why it matters for this change: criterion 4 says "malformed", and a hand-rolled parser decides what that means by accident.

6. Finding: The default of 3 is not asserted anywhere. It is also stored in two separate constants (`DEFAULT_MAX_RETRIES` and `MAX_RETRIES`), which is how finding 3 happened.
   Evidence: src/config.rs:5 and :8; no existing test calls `Config::new().max_retries()`.
   Proposed check: `Config::new().max_retries() == 3`, and a worker over a fresh `Config` with an always-failing job makes 4 attempts. The second assertion keeps the meaning of the existing `worker_stops_after_retry_limit` (expected value 4 stays as is). (unit, then the integration check from finding 3) [to write]
   Why it matters for this change: criterion 2 currently passes by luck, and it would break silently if one constant were changed.

### Order of work
1. Confirm tier and answer the blocking questions below.
2. Write the failing reproductions first and watch them fail for the stated reason: finding 1 (10 rejected), finding 2 (-1 accepted), finding 3 (worker ignores the setting), finding 4 (400 instead of 422) ([rule]).
3. Write finding 6 (default) and the existing-behavior checks that should already pass today: set 5, reject 11, 4 attempts at default. These pin current behavior before the worker is touched ([rule]).
4. Change the code until the new checks pass. Do not change any existing test's expected value; if one seems to need it, stop and ask ([rule]).
5. Add finding 5 once question 1 is answered.

### Not verified by this plan
- Persistence. `Config` holds the value in memory only; the plan does not check that a saved value survives a restart.
- Thread ordering. `AtomicU8` with `SeqCst` is not exercised under real concurrency. Threaded tests would be flaky, so the plan uses sequential "set, then start a job" tests only.
- Jobs already running when the value changes. The plan assumes they keep the old value; the spec only covers jobs started after.
- Real HTTP behavior: method and path routing, content-type, body size limits, and the real framework's own error responses. The handler is a plain function over a string.
- Callers and other uses of `MAX_RETRIES` outside `src/config.rs` (not read).
- Whether `cargo test` passes today (not run; I took the task's statement). A green run would not show the bugs above, since the existing tests do not touch them.

### Questions for you (blocking)
1. What counts as malformed, and is 422 right for all of it? I propose: missing key, non-integer (string, null, 3.5), integer overflow, and invalid JSON all give 422 `max_retries_out_of_range`. Is `3.0` an integer? Should a non-JSON body also give 422 rather than 400?
2. Success response: the code returns 200 with `{"value": n}`, but the spec says nothing about it. Should the test assert that body, or only status 200?
3. "Saved value": does it have to survive a restart (a database or file)? If so the plan needs a persistence check and probably Tier 3.
4. How should `RetryWorker` get the config (constructor argument, parameter to `run`, shared handle)? The tests in finding 3 depend on this public shape. Also confirm that `max_retries = 3` means 3 retries after the first try (4 attempts), as the existing test assumes.
5. Do you accept Tier 2? And would you like a property test for "never panics, stored value always in 0..=10" ([rule])? That would need a new dependency such as `proptest` ([rule]); the plan uses a plain sweep loop instead unless you say yes.
