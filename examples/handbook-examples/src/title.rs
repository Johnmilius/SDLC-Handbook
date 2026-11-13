//! Case 2: title truncation in existing code.
//!
//! Users report a crash when a note title contains emoji. This module has a
//! planted defect. See `examples/case-2-truncate-title.md`.

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
