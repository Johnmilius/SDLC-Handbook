# Case 2: title truncation that crashes on emoji

**Kind:** bug fix in existing code, Tier 1. **Code:** [handbook-examples/src/title.rs](handbook-examples/src/title.rs). **Exercises:** [TP-4](../handbook/plan-tests.md#tp-4), [TP-5](../handbook/plan-tests.md#tp-5), [TP-10](../handbook/plan-tests.md#tp-10), [TP-12](../handbook/plan-tests.md#tp-12) and [TP-13](../handbook/plan-tests.md#tp-13), through the [`test-plan` skill](../skills/test-plan/SKILL.md).

## The request

> Users report a crash when a note title contains emoji. Fix `truncate_title`.

## Planted defects and risks (deliberate)

| # | Defect or risk | Where |
|---|---|---|
| 1 | `&title[..max]` slices by byte index and panics when `max` falls inside a multi-byte character | `truncate_title` |
| 2 | No test reproduces the crash | Test module |
| 3 | The cut path for ASCII titles is not pinned by any test, so a fix could change it unnoticed | Test module |
| 4 | An open product decision: the doc comment says `max` counts "characters", but the code counts bytes. Bytes, `char`s or grapheme clusters? | Doc comment and code |
| 5 | The same byte slice is in `preview_snippet` | Same file |

## Expected skill output **[Illustrative]**

```
## Test plan: stop truncate_title panicking on multi-byte titles
Tier proposed: 1. Triggers: none.  (You decide.)
Acceptance criteria (drafted, please confirm):
  AC-1 truncate_title never panics for any title and any max.
  AC-2 ASCII titles behave as they do today.

### Findings
1. Finding: no test reproduces the crash.
   Evidence: src/title.rs has one test, short_title_is_unchanged. `&title[..max]` is a byte slice.
   Proposed check: a title with a multi-byte character across `max` does not panic (unit) [to write first, and watch it fail]
   Why it matters for this change: without it there is no proof the fix fixes the report.
   Handbook: TP-4 (reference.md#tp-4)
2. Finding: the cut path for ASCII is not pinned.
   Evidence: the only test uses a title shorter than max.
   Proposed check: truncate_title("Hello, world", 5) == "Hello…", plus max = 0, an empty title, and a title exactly max long (unit) [to write before the fix]
   Why it matters for this change: the fix must not change the ASCII results users see now.
   Handbook: TP-5 (reference.md#tp-5)
3. Finding: preview_snippet has the same byte slice.
   Evidence: src/title.rs, `&body[..max]`.
   Proposed check: report it; fix it in a separate change with its own failing test.
   Why it matters for this change: the same crash is waiting there.
   Handbook: TP-5 (reference.md#tp-5)

### Order of work
1. Answer question 1. 2. Write check 2 (passes now). 3. Write check 1 (fails now). 4. Fix. 5. Rerun.

### Not verified by this plan
- How truncated titles render in the UI.

### Questions for you (blocking)
1. What does `max` count: bytes, chars or grapheme clusters? The doc comment says "characters"; the code counts bytes. The expected values in check 1 depend on the answer.
2. "Never panics" is a property. A property test would cover it but needs a dev-dependency. Do you want one?
```

## What to notice

- **It stops on the product decision.** Choosing bytes, `char`s or graphemes silently would bake a guess into the tests. A plain "fix this" request usually picks one without saying so.
- **It orders the work.** Pin first, then fail first, then fix.
- **It stays short.** This is a Tier 1 change. The plan has three findings, not ten.
- **It reports the second site instead of widening the fix.** One concern per change ([QB-7](../handbook/quality-bar.md#qb-7)).
