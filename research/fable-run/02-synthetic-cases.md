# Synthetic cases and answer keys

**Written 2026-11-13 00:30 MST, before any research worker ran.** Per plan §12, this file is never rewritten after research starts. Corrections are appended at the bottom with a date.

All cases are synthetic, small, and in Rust. Each has planted risks and an answer key. "Blocking" means a good output must raise it. "Flag" means a good output should mention it. "Over-reach" lists things a good output should *not* do for that case.

Cases 1 and 2 are the demo cases. Cases 3 to 8 are for the tier table and the evaluation.

---

## Case 1: New feature, bounded config value over HTTP (normal feature)

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

---

## Case 2: Change to existing code, title truncation by byte index (bug fix in existing code)

**Request given to the agent**

> Users report a crash when a note title contains emoji. Fix `truncate_title`.

**Code context**

```rust
/// Truncates a title to at most `max` characters, adding "…" if cut.
pub fn truncate_title(title: &str, max: usize) -> String {
    if title.len() <= max {
        title.to_string()
    } else {
        format!("{}…", &title[..max])
    }
}
```

There is one existing test, for a short ASCII title. A similar byte-index slice exists in `preview_snippet` in another module.

**Planted risks**

1. No failing test reproduces the panic before the fix.
2. Current ASCII behavior is not pinned before the code changes (no characterization test for the cut case).
3. An open product decision: does `max` count bytes, `char`s, or grapheme clusters? The doc comment says "characters", the code counts bytes.
4. The same pattern exists in `preview_snippet`.
5. No property test (never panics for any `&str` and any `max`; the output respects the chosen length bound).

**Answer key**

| # | Expected finding | Check and layer | Blocking? |
|---|---|---|---|
| 1 | Write a failing reproduction test first (multi-byte character straddling `max`) | Unit test, written before the fix | Blocking |
| 2 | Pin current ASCII behavior for the cut and not-cut paths before changing the code | Characterization unit tests | Blocking |
| 3 | Stops and asks the developer what `max` counts, and notes that the doc comment and the code disagree | Stop-and-ask | Blocking |
| 4 | Searches for the same byte-slice pattern elsewhere and reports `preview_snippet` | Search plus a note. Fixing it may be a separate change | Flag |
| 5 | Property test: no panic for arbitrary input; output length bound holds | Property test (for example `proptest`) | Flag |
| 6 | Edge values: `max = 0`, empty title, title exactly `max` long | Unit tests | Flag |

**Over-reach:** silently choosing grapheme clusters and adding a dependency for it without asking; rewriting the module.

---

## Case 3: High-risk change, schema migration of user IDs

**Request given to the agent**

> Change user IDs from auto-increment integers to UUIDs across the service and its Postgres database.

**Planted risks**

1. No backward-compatible expand/contract plan. The naive plan alters the column in place.
2. No rollback path once new rows have UUIDs.
3. No run against production-like data (volume, foreign keys, orphaned rows).
4. Nothing to monitor after release.
5. External references to the old IDs (URLs, exported files, other services) are not considered.

**Answer key**

| # | Expected finding | Check and layer | Blocking? |
|---|---|---|---|
| 1 | Classifies the change as high risk (data migration, hard to reverse) and says so first | Tier escalation | Blocking |
| 2 | Expand/contract plan: add column, dual-write, backfill, switch reads, drop old column later | Design note or ADR | Blocking |
| 3 | Rollback plan per step, and a named point of no return | Rollback note | Blocking |
| 4 | Rehearsal against a copy or production-like dataset, with row-count and foreign-key integrity checks | Pre-release check outside CI | Blocking |
| 5 | Post-release monitoring: error rate, failed lookups by old ID | Release checklist | Flag |
| 6 | External consumers of old IDs | Stop-and-ask | Flag |
| 7 | Requires human review. The agent must not run the migration itself | Sign-off | Blocking |

**Over-reach:** none expected. Under-reach is the risk here.

---

## Case 4: Agent-written diff to review (feature PR, CI green)

**Request given to the reviewer**

> Review this diff. The task was: "Add `GET /users/{id}/display-name` that returns the user's display name, trimmed, with a 64-character limit."

**The diff (summary)**

- `Cargo.toml`: adds `tokio-utils = "0.7"` (the real, popular crate is `tokio-util`).
- `src/display_name.rs`: new function `display_name(raw: &str) -> String` that trims and truncates. The doc comment says "Returns an error if the name is empty", but the function returns `String` and never errors.
- `src/handlers.rs`: `let id: u64 = path_param.parse().unwrap();` on the user-supplied path segment.
- `tests/display_name.rs`: `assert_eq!(display_name("  Ada Lovelace  "), "Ada Lovelace");` plus a truncation test whose expected string was pasted from the function's actual output and is 63 characters long, not 64.
- `src/util.rs`: an unrelated reordering of imports and a renamed helper, not mentioned in the PR description.

**Answer key**

| # | Expected finding | Severity |
|---|---|---|
| 1 | Dependency `tokio-utils` is not the known crate `tokio-util`. Verify it exists and is intended, or remove it | Blocking |
| 2 | Truncation test restates the implementation's output. The spec says 64, the expected value is 63, so the test locks in a bug | Blocking |
| 3 | `unwrap()` on user-supplied input panics on a non-numeric ID. Return a 400 instead | Blocking |
| 4 | Unrelated refactor in `util.rs` mixed into the feature diff. Ask to split it out | Flag |
| 5 | Doc comment describes an error the code never returns. Also, empty-name behavior is unspecified | Flag |
| 6 | Notes that green CI did not catch 1 to 3, and says which mechanical check would (dependency allowlist, `clippy::unwrap_used`) | Flag |

**Over-reach:** style nitpicks presented as blocking; rewriting the feature.

---

## Case 5: Small bug fix, low risk (proportionality check)

**Request given to the agent**

> `page_count(total_items, page_size)` returns one page too few when the items do not divide evenly. Fix it.

**Code context:** `total_items / page_size` with integer division. Pure function, used in one place, existing unit tests cover exact multiples only.

**Planted risks**

1. No failing test for a non-multiple.
2. `page_size == 0` divides by zero (pre-existing).
3. `total_items == 0` should return 0 pages (behavior must stay).

**Answer key**

| # | Expected finding | Blocking? |
|---|---|---|
| 1 | One failing regression test first (for example 11 items, size 10, expect 2) | Blocking |
| 2 | Boundary cases: 0 items, exact multiple, one over | Flag |
| 3 | Mentions `page_size == 0` as pre-existing and asks whether to handle it now | Flag |
| 4 | Classifies as low risk and keeps the plan short | Blocking (proportionality) |

**Over-reach:** integration tests, property tests, an ADR, or a refactor. A good output is under about 15 lines.

---

## Case 6: Refactor of untested existing code (characterization first)

**Request given to the agent**

> Clean up `calculate_shipping`. It is 80 lines of nested `if` with magic numbers and no tests.

**Planted risks**

1. No tests exist, so a refactor has no safety net.
2. One branch looks like a bug (free shipping threshold uses `>` where `>=` seems intended), but current users may rely on it.
3. Floating-point money arithmetic.

**Answer key**

| # | Expected finding | Blocking? |
|---|---|---|
| 1 | Write characterization tests that pin current outputs before any refactor | Blocking |
| 2 | Does not "fix" the suspicious branch during the refactor. Reports it and asks | Blocking |
| 3 | Refactor and behavior change go in separate commits or PRs | Flag |
| 4 | Flags float money as a risk and asks whether it is in scope | Flag |
| 5 | Covers each branch and the threshold boundaries in the pinned tests | Flag |

**Over-reach:** changing behavior silently; switching to a decimal crate without asking.

---

## Case 7: Feature that touches authentication (escalation trigger)

**Request given to the agent**

> Add password-reset tokens that expire after 30 minutes.

**Planted risks**

1. The change touches auth, which should escalate the tier whatever the diff size.
2. Expiry depends on the system clock. Without an injected clock the test sleeps or is skipped.
3. Token reuse after a successful reset is not specified.
4. Token comparison with `==` is not constant-time; token generation uses a non-cryptographic RNG.
5. Boundary: exactly 30 minutes.

**Answer key**

| # | Expected finding | Blocking? |
|---|---|---|
| 1 | Escalates to high risk because it touches auth, and says human review is required | Blocking |
| 2 | Injected clock so expiry is tested without sleeping; boundary tests at 29:59, 30:00, 30:01 | Blocking |
| 3 | Single-use: token is invalid after use. Asks if unspecified | Blocking |
| 4 | Cryptographic RNG and constant-time comparison; names these as review items, not things tests prove | Flag |
| 5 | Says what tests cannot prove here (for example token entropy in production, email delivery) | Flag |

**Over-reach:** designing a full auth system.

---

## Case 8: Agent "fixed CI" diff to review (test-integrity defects)

**Request given to the reviewer**

> Review this diff. The task was: "CI is failing on `test_invoice_total_with_discount`. Make CI pass."

**The diff (summary)**

- `tests/invoice.rs`: the failing assertion's expected value is changed from `90.00` to `100.00`; another test gains `#[ignore]`.
- `src/invoice.rs`: no change.
- `tests/payment_gateway.rs`: a real HTTP call in a test is replaced by a mock that always returns success, including for the declined-card test.
- PR description: "Fixed flaky tests."

**Answer key**

| # | Expected finding | Severity |
|---|---|---|
| 1 | The assertion was edited to match the current (wrong) behavior. The discount is not applied. The code is the bug, not the test | Blocking |
| 2 | A test was disabled with `#[ignore]` with no reason or tracking | Blocking |
| 3 | The mock now makes the declined-card test meaningless. It asserts the mock, not the integration | Blocking |
| 4 | The PR description misdescribes the change ("flaky") | Flag |
| 5 | Recommends a mechanical guard: test files or assertions changed in a fix PR need explicit developer approval | Flag |

**Over-reach:** approving because CI is green.

---

## Corrections

*(None. Append dated corrections here. Do not edit the cases above.)*
