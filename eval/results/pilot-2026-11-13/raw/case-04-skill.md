## Test plan: add GET /users/{id}/display-name (trimmed, 64-character limit), reviewed as an agent-written diff
Tier proposed: 2. Triggers: adds a dependency (`tokio-utils`); adds a new endpoint (wire contract); renames a `pub fn` in `src/util.rs` (public API); `get_display_name` has no tests in this diff. No Tier 3 trigger is clearly met, but see blocking question 1 (authorization). (You decide.)
Acceptance criteria (drafted from the task sentence, please confirm):
1. Given a user named "  Ada Lovelace  ", when GET /users/{id}/display-name, then 200 and body "Ada Lovelace".
2. Given a user whose trimmed name is exactly 64 characters, then all 64 are returned.
3. Given a user whose trimmed name is longer than 64 characters (for example 65), then exactly the first 64 are returned.
4. Given an id with no user, then the not-found response.
5. Negative: given a non-numeric or out-of-range id (for example "abc", "-1", "99999999999999999999999"), then a client-error response and no panic. The status code is not in the spec.
6. Empty or whitespace-only name: expected behavior unknown. See question 3.

### Findings
1. Finding: Off-by-one. The limit is 64 but the code takes 63 characters.
   Evidence: `src/display_name.rs` diff: `trimmed.chars().take(63).collect()`. Task: "64-character limit".
   Proposed check: a name of 64 characters comes back as 64; a name of 65 comes back as the first 64; 63 is unchanged. Written from the spec, not from the output. (unit) [to write]
   Why it matters for this change: it is the one stated requirement of the feature and it is wrong by one.
   Handbook: TP-3 (reference.md#tp-3)

2. Finding: The existing test `truncates_long_names` expects the implementation's output, not the spec's. Its expected literal appears to be 63 characters (matching `take(63)`; I counted by eye, please confirm). It passes, which is why CI is green despite finding 1.
   Evidence: `tests/display_name.rs` diff, `truncates_long_names`; the spec sentence says 64.
   Proposed check: after you approve, change the expected value to 64 characters (`"a".repeat(64)`) so it fails against the current code. I will not change it myself. (unit) [exists: truncates_long_names, needs your approval to change]
   Why it matters for this change: a test whose expected value came from the code cannot fail when the code is wrong.
   Handbook: TP-1 (reference.md#tp-1), TP-11 (reference.md#tp-11), QB-2 (reference.md#qb-2)

3. Finding: Boundaries and partitions are mostly untested. Only one short happy case and one 80-character case exist.
   Evidence: `tests/display_name.rs` diff has two tests, `trims_whitespace` and `truncates_long_names`.
   Proposed check: a table of cases: empty string; whitespace only; 63, 64 and 65 characters; whitespace around a 64-character name (proves trimming happens before the limit); 70 characters with a space at position 64 (proves whether truncation can leave trailing whitespace, depends on question 2); multibyte input such as 64 "é" characters and an emoji at the boundary (proves characters, not bytes). (unit) [to write]
   Why it matters for this change: the bug in finding 1 sits exactly on a boundary no test visits.
   Handbook: TP-2 (reference.md#tp-2), TP-3 (reference.md#tp-3)

4. Finding: The doc comment says "Returns an error if the name is empty", but the function returns `String` and has no error path.
   Evidence: `src/display_name.rs` diff, doc comment and signature `-> String`.
   Proposed check: once the empty-name behavior is decided (question 3), one test asserts it. (unit) [to write]
   Why it matters for this change: the comment and the code disagree and the spec is silent, so either could be the intended behavior.
   Handbook: TP-12 (reference.md#tp-12)

5. Finding: The handler panics on a bad id.
   Evidence: `src/handlers.rs` diff, `path_param.parse().unwrap()`.
   Proposed check: call `get_display_name("abc", &store)`, `"-1"`, `""` and an overflowing number; assert a client-error `Response` and no panic. (unit) [to write]
   Why it matters for this change: any caller can crash the handler with one malformed URL.
   Handbook: TP-2 (reference.md#tp-2)

6. Finding: `get_display_name` has no test for found or not-found. Nothing in the diff proves the route is connected to it.
   Evidence: `src/handlers.rs` diff adds the function; the diff shows no router change, no `mod display_name;` declaration and no `use` for it, but `tests/display_name.rs` imports `app::display_name::display_name`. Either those lines exist outside the diff or the "CI is green" claim cannot be right. Router, `lib.rs` and the `UserStore` type: Not read.
   Proposed check: the one integration check that proves the pieces connect: send GET /users/{id}/display-name through the real router with a real or in-memory `UserStore`; assert 200 with the trimmed, limited body for a known user and the not-found response for an unknown id. Unit tests of the handler cover found, not-found and bad id on top of it. (integration) [to write]
   Why it matters for this change: unit tests of `display_name()` pass even if the endpoint is never reachable.
   Handbook: TP-6 (reference.md#tp-6), TP-7 (reference.md#tp-7)

7. Finding: New dependency `tokio-utils = "0.7"` is added and nothing in the diff uses it. The well-known crate is named `tokio-util`; I could not check the registry from here, so I cannot say whether `tokio-utils` exists or who publishes it.
   Evidence: `Cargo.toml` diff; no `tokio_utils` or `tokio_util` reference in any other hunk.
   Proposed check: confirm the crate is the intended one, justify it or remove it, and run your dependency policy check (for example `cargo deny` or `cargo audit`, if the project has them). (CI / policy) [to write; needs your decision]
   Why it matters for this change: an unused, possibly misspelled dependency is a supply-chain risk and the feature does not need it.
   Handbook: QB-8 (reference.md#qb-8)

8. Finding: Unrelated change in `src/util.rs`: `fmt_ts` is renamed to `format_timestamp` and imports are reordered. `fmt_ts` is `pub`, and only the call in `log_line` was updated.
   Evidence: `src/util.rs` diff. Callers in other files and other crates: Not read.
   Proposed check: search the whole repository for `fmt_ts`; run a full `cargo build --all-targets` and the full test suite; or ask for the rename to be split into its own change. (build / integration) [to write]
   Why it matters for this change: a public rename outside the task's scope can break callers this diff does not show, and it hides in a review of the endpoint.
   Handbook: TP-5 (reference.md#tp-5), TP-6 (reference.md#tp-6)

### Integrity pass (diff given)
- Tests deleted or ignored: none in the diff.
- Tests with a new expected value: none changed, but `truncates_long_names` is new and its expected value matches the code, not the spec (finding 2). Needs your decision (TP-11, QB-3).
- Mocks replacing a real integration: none in the diff. No integration test exists at all (finding 6).
- New or changed dependencies: `tokio-utils = "0.7"` (finding 7). Needs your decision (QB-8).
- "CI is green" settles none of these (TP-8). The tests cannot fail on the off-by-one.

### Order of work
1. Answer the blocking questions below.
2. Write the 64/65-character tests from the spec first and see them fail against `take(63)` (TP-4). That failure reproduces finding 1.
3. Fix the limit. Then add the boundary and partition table (finding 3) and the empty-name test (finding 4).
4. Add the handler unit tests (finding 5, found, not found).
5. Add the router-level integration check (finding 6).
6. Resolve the dependency (finding 7) and the `util.rs` rename (finding 8); run the full build and test suite.
7. Optional: check that the new tests are trustworthy by temporarily breaking `display_name` (for example restoring `take(63)`) and confirming they fail (TP-8).

### Not verified by this plan
- Whether `user.name` is the intended "display name" field. The `User` type was not read.
- Authorization and authentication on the new route; the sibling handler `render_user` and the router were not read.
- Response format and content type (plain string versus JSON); the spec does not say.
- Grapheme-cluster handling (accented letters built from combining marks, emoji sequences) beyond what question 2 decides.
- Behavior against the real user store and a real server, concurrency, and performance; CI with an in-memory store cannot show these.
- The "CI is green" claim itself; I saw no CI output.
- Whether the `tokio-utils` crate exists and is safe; this needs a registry and policy check I could not run.

### Questions for you (blocking)
1. Tier: do you accept Tier 2? And should this endpoint sit behind the same authorization as the existing user handler? If it exposes user data without it, this becomes Tier 3.
2. "64-character": count Unicode scalar values (`chars()`, what the code does), bytes, or grapheme clusters? And after truncating, should trailing whitespace be trimmed again?
3. Empty or whitespace-only name: return an error (as the doc comment says), an empty string (as the code does), or something else? Which one is the spec? (TP-12)
4. Non-numeric or out-of-range id: which status do you want, 400 or 404?
5. May I change `truncates_long_names` so it expects 64 characters instead of 63? I will not touch it without your approval. (TP-11, QB-3)
6. `tokio-utils`: is it the crate you meant (perhaps `tokio-util`), and should it stay at all, given nothing uses it? (QB-8)
7. Is the `fmt_ts` rename meant to be part of this change, or should it be split out? Can you show me the router, `lib.rs` and `mod` changes the diff does not include?
8. A "never panics for any id" property test would need a property-testing dependency such as `proptest`. Do you want that, or should the table of bad ids in finding 5 be enough? (TP-13)
