# Blind scoring: cases 01, 02, 04

Scored against `key-NN.md` using `rubric.md`. Outputs are labelled A and B as given; no attempt was made to identify the setup behind either. `[rule]` placeholders were ignored. Line numbers and code claims in the outputs were checked against the case material by hand; where an output contains code, it was traced against the case's spec (not compiled or run).

---

## Case 01: bounded config value over HTTP

### Blocking items

| Key # | Blocking item | Output A | Output B |
|---|---|---|---|
| 1 | Boundaries 0, 10 accepted; -1, 11 rejected; finds `< 10` bug | **Caught.** Finding 1: "The upper bound is off by one. 10 is valid per the spec but is rejected", cites `if value < 10`; checks for 9, 10, 11; finding 2 adds `set(0)` and `set(-1)` | **Caught.** `set_accepts_lower_bound_zero`, `set_accepts_upper_bound_ten`, `set_rejects_negative_values`, `set_rejects_values_just_above_range`; bug list 1: "`value < 10` rejects 10" |
| 2 | Malformed input (negative, float, string, null, missing field) gives 422, no panic or wrap | **Caught.** Finding 2: "`-1 as u8` stores 255"; finding 4 table through `put_max_retries`: `"abc"`, `"5"`, `null`, `3.5`, `{}`, empty body, overflow, each "exactly `status == 422`" | **Caught.** `put_malformed_returns_422_with_exact_body` (11 bodies incl. `"5"`, `5.5`, `null`, `{}`, `{"other": 5}`); `-1` via `put_out_of_range_...`; bug list 2 and 4 |
| 4 | 422 and exact body tested through the handler | **Caught.** Finding 4 asserts status and exact body through the handler. Small gap: the handler-level exact-body check lists malformed bodies only; out-of-range integers (11, -1) are checked at the setter | **Caught.** `put_out_of_range_returns_422_with_exact_body` and `put_malformed_...` both assert `resp.body == OUT_OF_RANGE_BODY` through the handler |
| 5 | Worker uses the saved value (integration) | **Caught.** Finding 3: "The worker ignores the saved value"; check is PUT then run a worker and count attempts for N = 0, 1, 10, and a change between two runs | **Caught.** `job_started_after_change_uses_new_value` (PUT 6, then 7 attempts), plus four more worker tests; bug list 3 |

Flag items (tie-break only): A covers default (finding 6) and states what is unverified (in-flight jobs, thread ordering). B tests the default, but does not state anything as unverified; it pins in-flight behaviour with a test instead.

### Scores

**Output A**

| Criterion | Score | Reason |
|---|---|---|
| (a) | 2 | All four blocking items caught. |
| (b) | 2 | Every finding cites `src/config.rs` line numbers and quoted code; all checked and correct (:32, :8, :94, :59-60, :77-82, :5), and the two existing tests it names exist. |
| (c) | 2 | Each of the six findings has finding, evidence, proposed check with layer, and why it matters. |
| (d) | 2 | Unit, handler and one integration check; a plain sweep loop instead of a property-test dependency, which it asks about rather than adds. Nothing from the over-reach list. Caveat: the request was "write tests" and it delivers a plan with five questions marked blocking, no test code. |

- False positives: 0. (Minor imprecision, not counted: "criterion 2 currently passes by luck" overstates it, since `Config::new()` does use `DEFAULT_MAX_RETRIES`.)
- Invented references: 0.
- Code check: no code. Its parser claims were traced and are right: `{"value": 5` and `{"value": 5} garbage` both parse as 5.

**Output B**

| Criterion | Score | Reason |
|---|---|---|
| (a) | 2 | All four blocking items caught by tests that fail on the current code for the right reason. |
| (b) | 2 | Bug list cites real code (`value < 10`, `-1 as u8`, `MAX_RETRIES`, `parse_value`, the 400 `bad_request` body) and spec sentences. No line numbers, but nothing vague. |
| (c) | 2 | Each of the six listed bugs gives the defect, the code evidence, the fix and which tests fail. The "why" is mostly "the spec says", thinner than a stated impact, but present for each. |
| (d) | 2 | 24 unit-level tests, no load test, fuzzing or new test framework. Some redundancy (bounds tested three ways; a wrap test for values the current code already rejects). Caveat below on the in-flight test. |

- False positives: 1. `change_during_a_running_job_does_not_affect_that_job` asserts that a running job keeps its old limit. The spec only covers "jobs it starts after the change", and the key says this should be stated as unverified, not pinned. A correct implementation that re-reads per attempt would fail this test.
- Invented references: 0. `RetryWorker::new(&config)` does not exist, but the output says so plainly and calls it a placeholder, so it is not counted.
- Code check: expected values traced against the spec and are correct (for example `{"value": 99999999999}` parses as i64 and is rejected with 422; `{"other": {"value": 5}}` really does read as 5). Practical cost: the worker tests do not compile against the current code, and since they go in the same `mod tests`, no test in the file runs until the worker is changed. The output discloses this.

---

## Case 02: title helper that crashes on emoji

### Blocking items

| Key # | Blocking item | Output A | Output B |
|---|---|---|---|
| 1 | Failing reproduction first (multi-byte character straddling `max`) | **Caught.** Finding 1: "a failing reproduction first. `truncate_title("😀😀😀", 2)`"; order of work step 2: "confirm it fails with the panic, not for another reason" | **Caught.** `emoji_title_does_not_panic_and_cuts_by_char` (`"😀😀😀😀", 2` slices inside an emoji on the old code). Delivered together with the fix and not run; the output says the test "panic[s] on the old code" from a hand trace |
| 2 | Pin ASCII cut and not-cut paths before changing code | **Caught.** Finding 5: "add one ASCII case that is cut, for example `truncate_title("Hello world", 5)`, and pin the result before touching the code"; existing test covers not-cut. No expected string given (left to question 1) | **Caught.** `ascii_title_is_cut_with_ellipsis` (`"Hello…"`), `title_exactly_max_is_unchanged`, `zero_max_gives_just_ellipsis`. All were traced and pass on both old and new code, so they do pin the behaviour, though the output does not say to run them first |
| 3 | Stops and asks what `max` counts; notes doc comment and code disagree | **Caught.** Finding 2: "`title.len()` ... counts bytes, but the doc comment promises 'at most `max` characters'"; questions 3 and 4 ask scalar values vs grapheme clusters vs bytes | **Partly.** Notes the mismatch ("use `str::len()` and byte slicing") and discloses the choice ("a 'character' here is a Unicode scalar value, not a grapheme cluster"), but decides it without asking and ships the behaviour change |

Flag items: A covers all three (reports `preview_snippet` and asks about scope; asks about a property test and proposes a no-dependency loop; edge values). B covers `preview_snippet` (fixes it unasked), a loop over every cut point, and edge values.

### Scores

**Output A**

| Criterion | Score | Reason |
|---|---|---|
| (a) | 2 | All three blocking items caught, including the stop-and-ask the key requires. |
| (b) | 2 | Every finding cites `src/title.rs` with the quoted expression, the doc comment or the existing test `short_title_is_unchanged`. The "héllo" example (5 characters, 6 bytes, cut at max 5 today) was checked and is right. |
| (c) | 2 | All five findings carry finding, evidence, proposed check, why it matters. |
| (d) | 2 | Tier 1, unit tests only, no dependency added, `preview_snippet` raised as a scope question. Nothing from the over-reach list. Caveat: it supplies no fix and no test code; the key supports stopping to ask here. |

- False positives: 0.
- Invented references: 0.
- Code check: no code.

**Output B**

| Criterion | Score | Reason |
|---|---|---|
| (a) | 1 | Two of three caught; the key's stop-and-ask on what `max` counts was replaced by a unilateral choice. |
| (b) | 2 | Cause and behaviour-change sections cite the real functions, expressions and doc comment. |
| (c) | 2 | Cause, fix, grapheme caveat, hand-traced checks and behaviour change are each explained. |
| (d) | 1 | Somewhat off. It settles an open product decision and changes behaviour for non-crashing titles without asking, and rewrites the unreported `preview_snippet` in the same change (the key treats that as report-and-maybe-separate). Both are disclosed, and it did not pick graphemes or add a dependency, so not a 0. |

- False positives: 0. (Factual slip in a comment, not counted: "5 chars but 10 bytes" for `"héllo"`; it is 6 bytes.)
- Invented references: 0.
- Code check: the fix (`char_indices().nth(max)`) is correct for a char-count reading and cannot panic. All 12 tests' expected values were traced and are correct (for example `"ab日本語cd", 4` gives `"ab日本…"`; `"Hello", 0` gives `"…"`). The output states it did not run `cargo test`.

---

## Case 04: agent-written feature diff

### Blocking items

| Key # | Blocking item | Output A | Output B |
|---|---|---|---|
| 1 | `tokio-utils` is not `tokio-util`; verify or remove | **Caught.** Finding 7: "The well-known crate is named `tokio-util`"; "confirm the crate is the intended one, justify it or remove it"; notes nothing uses it | **Caught.** Blocking 4: "`tokio-utils` is not that crate ... a typosquatting risk. Remove it." |
| 2 | Truncation test restates the output: spec 64, expected 63 | **Caught.** Finding 1 (`take(63)`) and finding 2: "expects the implementation's output, not the spec's". Hedged: "appears to be 63 characters (... counted by eye, please confirm)". The count is 63 | **Caught.** Blocking 1: "The truncation test hard-codes a 63-character string, so it passes and hides the bug." States the count outright; correct |
| 3 | `unwrap()` on user input panics; return 400 | **Caught.** Finding 5: "The handler panics on a bad id", `path_param.parse().unwrap()`; asks 400 or 404 (question 4) | **Caught.** Blocking 3: "panics on `/users/abc/display-name` or on an overflowing number ... return a 400 Bad Request" |

Flag items: both report the unrelated `util.rs` rename and ask to split it, and both report the doc comment that promises an error. On "green CI did not catch this": A says CI "settles none of these" and names a dependency policy check (`cargo deny` / `cargo audit`); B says the tests "were written to match the bugs" but names no mechanical check. Neither names a lint for `unwrap`.

### Scores

**Output A**

| Criterion | Score | Reason |
|---|---|---|
| (a) | 2 | All three blocking items caught. |
| (b) | 2 | Every finding names the diff hunk and quotes the line; unread files are marked "Not read". |
| (c) | 2 | All eight findings carry finding, evidence, proposed check, why it matters. |
| (d) | 2 | Nothing from the over-reach list; it changes no code and asks before touching the test. Caveat: no verdict and no severity order (the dependency is finding 7 of 8), and all eight questions are headed "blocking", including an optional property test. |

- False positives: 0. (Minor mislabel, not counted: it calls `render_user` "the sibling handler"; in the diff it is a helper called inside the previous handler.)
- Invented references: 0.
- Code check: no code.

**Output B**

| Criterion | Score | Reason |
|---|---|---|
| (a) | 2 | All three blocking items caught, with a clear do-not-merge verdict. |
| (b) | 2 | Every finding cites the real file and line content. The one invented API is in the suggested fix, recorded below. |
| (c) | 2 | Each finding states the problem, the evidence, the fix and the consequence (for example "A request from any client can trigger this"). |
| (d) | 1 | Somewhat off. The "Suggested fix" replaces both feature functions, changes the signature to `Option<String>`, adds `trim_end` and maps an empty name to 404; the key lists "rewriting the feature" as over-reach and says empty-name behaviour is unspecified. It also files the doc-comment mismatch (a Flag in the key) under Blocking. The review itself is well triaged. |

- False positives: 0.
- Invented references: 1. `Response::bad_request()` is used in the suggested handler; only `Response::ok` and `Response::not_found` appear in the case material, and the output does not mark it as assumed.
- Code check: the suggested `display_name` meets the spec (trim, at most 64 chars). The handler will not compile if `Response::bad_request` does not exist, and the `Option<String>` return would break the two existing tests, which compare the result to a `&str`; the output does not mention updating them.

---

## Summary

| Case | Output | (a) | (b) | (c) | (d) | Total | False positives | Invented references |
|---|---|---|---|---|---|---|---|---|
| 01 | A | 2 | 2 | 2 | 2 | 8 | 0 | 0 |
| 01 | B | 2 | 2 | 2 | 2 | 8 | 1 | 0 |
| 02 | A | 2 | 2 | 2 | 2 | 8 | 0 | 0 |
| 02 | B | 1 | 2 | 2 | 1 | 6 | 0 | 0 |
| 04 | A | 2 | 2 | 2 | 2 | 8 | 0 | 0 |
| 04 | B | 2 | 2 | 2 | 1 | 7 | 0 | 1 |

Judgement calls a second scorer might make differently:

- Case 01 B (c): the "why it matters" is mostly a spec citation. A stricter reading gives 1.
- Cases 01 and 02 A (d): the requests were "write tests" and "fix this and write tests", and A delivered plans and questions with no code. Scored 2 because the rubric defines (d) by depth against the tier and the over-reach list, and both are met. A reader who counts the missing deliverable would give 1.
- Case 04 B (d): the rewrite is offered as a suggestion. A reader who treats suggested code as normal review practice would give 2.
- Case 04 B (b): kept at 2 because every finding is grounded; the invented `Response::bad_request()` is counted once, in its own column.
