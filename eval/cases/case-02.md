# Case 02: a title helper that crashes on emoji

## Plain request

> Fix this and write tests.

## Material (given to both conditions)

The report:

> Users report a crash when a note title contains emoji. Fix `truncate_title`.

`cargo test` passes.

```rust
// src/title.rs
/// Truncates a title to at most `max` characters, adding "…" if cut.
pub fn truncate_title(title: &str, max: usize) -> String {
    if title.len() <= max {
        title.to_string()
    } else {
        format!("{}…", &title[..max])
    }
}

/// Returns the start of a note body for list previews, at most `max` characters.
pub fn preview_snippet(body: &str, max: usize) -> &str {
    if body.len() <= max {
        body
    } else {
        &body[..max]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_title_is_unchanged() {
        assert_eq!(truncate_title("Hello", 10), "Hello");
    }
}
```
