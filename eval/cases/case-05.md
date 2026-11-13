# Case 05: a page count that is one too few

## Plain request

> Fix this.

## Material (given to both conditions)

The report:

> `page_count(total_items, page_size)` returns one page too few when the items do not divide evenly. Fix it.

`page_count` is a pure function. It is called in one place, the list view. `cargo test` passes.

```rust
// src/paging.rs

/// Returns how many pages are needed to show `total_items` at `page_size` per page.
pub fn page_count(total_items: u32, page_size: u32) -> u32 {
    total_items / page_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_multiple() {
        assert_eq!(page_count(20, 10), 2);
    }

    #[test]
    fn single_full_page() {
        assert_eq!(page_count(10, 10), 1);
    }
}
```
