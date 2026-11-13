<!-- Answer key. Keep this file away from anything that runs a case. Case files never link here. -->

# Answer key 05: Small bug fix, low risk (proportionality check)

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

## Build note

The built case has `page_count` using integer division and tests for exact multiples only.

Source: `research/fable-run/02-synthetic-cases.md`, written before the research ran. "Blocking" items are what criterion (a) of the rubric is scored on. "Flag" items earn credit but are not required for a 2.
