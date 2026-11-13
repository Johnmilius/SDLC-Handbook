# Fix: `truncate_title` panics on emoji

## Cause

Both functions treat `max` as a character count, but they use `str::len()` and byte slicing (`&title[..max]`). `len()` returns bytes, and slicing at a byte index that falls inside a multi-byte UTF-8 character panics. An emoji is 4 bytes, so `&"😀😀"[..3]` panics. The existing test only uses ASCII, so it passes.

`preview_snippet` has the same bug. It was not in the report, but it fails the same way on any non-ASCII body, so it is fixed here too.

## Fix (`src/title.rs`)

Count and cut by `char`. The cut is at a `char` boundary, so it can never panic. Note that a "character" here is a Unicode scalar value, not a grapheme cluster, so a flag emoji, a family emoji or an accented letter written with combining marks can be split mid-cluster. That will not crash, but it can render oddly. If exact visual characters matter, use the `unicode-segmentation` crate (`graphemes(true)`) instead.

```rust
/// Truncates a title to at most `max` characters, adding "…" if cut.
pub fn truncate_title(title: &str, max: usize) -> String {
    match title.char_indices().nth(max) {
        None => title.to_string(),
        Some((idx, _)) => format!("{}…", &title[..idx]),
    }
}

/// Returns the start of a note body for list previews, at most `max` characters.
pub fn preview_snippet(body: &str, max: usize) -> &str {
    match body.char_indices().nth(max) {
        None => body,
        Some((idx, _)) => &body[..idx],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_title_is_unchanged() {
        assert_eq!(truncate_title("Hello", 10), "Hello");
    }

    #[test]
    fn ascii_title_is_cut_with_ellipsis() {
        assert_eq!(truncate_title("Hello world", 5), "Hello…");
    }

    #[test]
    fn title_exactly_max_is_unchanged() {
        assert_eq!(truncate_title("Hello", 5), "Hello");
    }

    #[test]
    fn empty_title() {
        assert_eq!(truncate_title("", 5), "");
    }

    #[test]
    fn zero_max_gives_just_ellipsis() {
        assert_eq!(truncate_title("Hello", 0), "…");
        assert_eq!(truncate_title("", 0), "");
    }

    #[test]
    fn emoji_title_does_not_panic_and_cuts_by_char() {
        // Each emoji is 4 bytes; the old code sliced inside one and panicked.
        assert_eq!(truncate_title("😀😀😀😀", 2), "😀😀…");
        assert_eq!(truncate_title("😀😀", 3), "😀😀");
        assert_eq!(truncate_title("😀😀😀", 3), "😀😀😀");
    }

    #[test]
    fn short_multibyte_title_is_unchanged_by_char_count() {
        // 5 chars but 10 bytes: must not be treated as longer than max 5.
        assert_eq!(truncate_title("héllo", 5), "héllo");
        assert_eq!(truncate_title("日本語", 3), "日本語");
    }

    #[test]
    fn mixed_ascii_and_multibyte_cut() {
        assert_eq!(truncate_title("ab日本語cd", 4), "ab日本…");
        assert_eq!(truncate_title("Hi 👋 there", 4), "Hi 👋…");
    }

    #[test]
    fn every_cut_point_is_safe() {
        let s = "a😀é日b";
        for max in 0..=s.chars().count() + 2 {
            let out = truncate_title(s, max);
            assert!(out.chars().count() <= max + 1); // +1 for the ellipsis
        }
    }

    #[test]
    fn preview_short_body_is_unchanged() {
        assert_eq!(preview_snippet("Hello", 10), "Hello");
        assert_eq!(preview_snippet("Hello", 5), "Hello");
    }

    #[test]
    fn preview_ascii_is_cut() {
        assert_eq!(preview_snippet("Hello world", 5), "Hello");
        assert_eq!(preview_snippet("Hello", 0), "");
    }

    #[test]
    fn preview_emoji_does_not_panic() {
        assert_eq!(preview_snippet("😀😀😀", 2), "😀😀");
        assert_eq!(preview_snippet("a😀b", 2), "a😀");
    }
}
```

## Checks

- `cargo test` passes before the change only because the single test is ASCII. The new emoji and multibyte tests panic on the old code (for example `truncate_title("😀😀😀😀", 2)` slices at byte 2) and pass on the new code.
- I have not run `cargo test` here, since I could only read the one file. Please run it. I traced the expected values by hand: `"😀😀😀😀"` with max 2 cuts at `char_indices().nth(2)` = byte 8, giving `"😀😀…"`. `"ab日本語cd"` with max 4 cuts after `本`, giving `"ab日本…"`.

## Behaviour change to be aware of

`max` now means characters, which is what the doc comments already promised. Before, a non-ASCII title longer than `max` bytes but shorter than `max` characters (for example `"héllo"` with max 5) was needlessly truncated. It now stays whole.
