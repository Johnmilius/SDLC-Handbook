# Review of the display-name endpoint diff

Verdict: do not merge as is. There are two correctness bugs, one risky panic, a suspicious dependency, and unrelated churn. "CI is green" is not strong evidence here, because the tests were written to match the bugs.

## Blocking

1. **Off-by-one in the length limit** (`src/display_name.rs`)
   - The spec says a 64-character limit, but the code uses `.take(63)`.
   - The truncation test hard-codes a 63-character string, so it passes and hides the bug. Count the `a`s in the expected value: it is 63.
   - Fix: use `.take(64)`. Make the test build its expectation with `"a".repeat(64)` and assert `.chars().count() == 64`, instead of a long literal.

2. **Doc comment promises behaviour the code does not have** (`src/display_name.rs`)
   - The doc says "Returns an error if the name is empty", but the function returns `String`, and an empty or whitespace-only name returns `""`.
   - Either return `Result<String, _>` (or `Option`) and handle it in the handler, for example with a 404/422, or delete the sentence. The task did not ask for an error, so decide deliberately.
   - There is no test for empty or whitespace-only input.

3. **`unwrap()` on user input** (`src/handlers.rs`)
   - `path_param.parse().unwrap()` panics on `/users/abc/display-name` or on an overflowing number. A request from any client can trigger this.
   - Fix: match on the parse result and return a 400 Bad Request. Add a test for it.

4. **Suspicious new dependency** (`Cargo.toml`)
   - `tokio-utils = "0.7"` is not used anywhere in the diff.
   - The crate people usually mean is `tokio-util`. `tokio-utils` is not that crate, and an unvetted name close to a popular one is a typosquatting risk.
   - Remove it. If something is genuinely needed, add `tokio-util` deliberately and say why.

## Should fix

5. **Unrelated changes in `src/util.rs`**
   - The diff reorders imports and renames `fmt_ts` to `format_timestamp`. None of this is part of the task.
   - A rename is an API change. Other call sites, in this crate or others, may still use `fmt_ts`. The diff does not show them, so confirm the build and grep for them.
   - Move this to a separate PR, or revert it.

6. **Handler is untested**
   - The tests only cover the pure `display_name` function. Nothing exercises `get_display_name`: 200, 404 for an unknown user, or a bad id.
   - The PR description says "with tests", but the routing and error paths, where the real bugs are, have none.

7. **Truncation edge cases**
   - `chars()` counts Unicode scalar values, not grapheme clusters. An emoji sequence or a combining accent can be cut mid-character. That is acceptable if documented, but state which unit "64 characters" means.
   - Trim happens before truncation, so a name can end with a space after the cut. For example, 63 characters plus a space plus more text gives a trailing space. Trim again after truncating, or truncate first and then trim.

8. **Missing wiring**
   - I see no route registration for `GET /users/{id}/display-name`, and no `mod display_name;` or `pub mod` in `lib.rs` or `main.rs`. The test imports `app::display_name::display_name`, so it may be declared elsewhere, but it is not in this diff. Confirm the endpoint is actually reachable.

## Minor

- `use` of `display_name` in `handlers.rs` is not shown as imported. It may be present, but the build would fail without it.
- Response content type: `Response::ok(String)` returns a bare string. Check that this matches the other endpoints' convention, for example JSON.

## Suggested fix for the core function

```rust
pub fn display_name(raw: &str) -> Option<String> {
    let name: String = raw.trim().chars().take(64).collect();
    let name = name.trim_end().to_string();
    if name.is_empty() { None } else { Some(name) }
}
```

and in the handler:

```rust
pub fn get_display_name(path_param: &str, store: &UserStore) -> Response {
    let Ok(id) = path_param.parse::<u64>() else {
        return Response::bad_request();
    };
    match store.find(id).and_then(|u| display_name(&u.name)) {
        Some(n) => Response::ok(n),
        None => Response::not_found(),
    }
}
```

Tests to add: exactly 64 characters is unchanged, 65 is cut to 64, empty and whitespace-only input, a non-numeric id gives 400, an unknown id gives 404.
