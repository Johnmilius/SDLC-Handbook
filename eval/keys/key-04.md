<!-- Answer key. Keep this file away from anything that runs a case. Case files never link here. -->

# Answer key 04: Agent-written diff to review (feature PR, CI green)

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

## Build note

In the built diff the suspicious crate is `tokio-utils` (the well-known crate is `tokio-util`). The implementation takes 63 characters where the task says 64, and the truncation test expects a 63-character string. The `unwrap()` is on the parsed path segment in `src/handlers.rs`.

Source: `research/fable-run/02-synthetic-cases.md`, written before the research ran. "Blocking" items are what criterion (a) of the rubric is scored on. "Flag" items earn credit but are not required for a 2.
