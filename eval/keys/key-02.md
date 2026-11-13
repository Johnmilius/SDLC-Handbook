<!-- Answer key. Keep this file away from anything that runs a case. Case files never link here. -->

# Answer key 02: Change to existing code, title truncation by byte index (bug fix in existing code)

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

## Build note

The built case contains `truncate_title` and `preview_snippet` in one file, with one existing test for a short ASCII title.

Source: `research/fable-run/02-synthetic-cases.md`, written before the research ran. "Blocking" items are what criterion (a) of the rubric is scored on. "Flag" items earn credit but are not required for a 2.
