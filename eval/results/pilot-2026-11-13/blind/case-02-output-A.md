## Test plan: stop `truncate_title` panicking on emoji titles (and decide what to do about `preview_snippet`, which has the same pattern)
Tier proposed: 1. Triggers: none fired. (You decide.) I considered "changes code that has no tests": `truncate_title` has one test, but it covers only the no-cut path, so the cut path is effectively untested. If you read it that way, this goes to Tier 2.
Acceptance criteria: drafted, please confirm
1. Given a title containing emoji or other multi-byte characters, when `truncate_title` is called with any `max`, then it never panics.
2. Given a title longer than `max` characters, when truncated, then the result is cut on a character boundary and ends with "…".
3. Given a title of `max` characters or fewer, when called, then it is returned unchanged. A multi-byte title that fits within `max` characters must not be cut (negative case).
4. Given an ASCII title, behavior is exactly what it is today.

### Findings
1. Finding: `truncate_title` slices at a byte offset, `&title[..max]`, which panics when `max` falls inside a multi-byte character. This is the reported crash.
   Evidence: src/title.rs, `&title[..max]` in `truncate_title`; the report ("crash when a note title contains emoji").
   Proposed check: a failing reproduction first. `truncate_title("😀😀😀", 2)` must not panic and must return a string that is valid and ends in "…" (unit) [to write]. The exact expected string depends on question 1.
   Why it matters for this change: this is the bug, and a test that fails first proves the fix does something.

2. Finding: the length check uses `title.len()`, which counts bytes, but the doc comment promises "at most `max` characters". A title like "héllo" (5 characters, 6 bytes) is cut at `max` = 5 today even though it fits. A fix that only avoids the panic could leave this wrong.
   Evidence: src/title.rs, `title.len() <= max` and the doc comment "at most `max` characters".
   Proposed check: a multi-byte title whose character count equals `max` is returned unchanged (unit) [to write]. A title one character over `max` is cut (unit) [to write]. The values for these come from the doc comment, not from running the code.
   Why it matters for this change: the fix changes behavior for non-emoji titles that do not crash today.

3. Finding: boundaries and partitions are untested: `max` = 0, `max` equal to the length, `max` one below and one above, an empty title, and a title of only emoji.
   Evidence: the only test is `short_title_is_unchanged` (`truncate_title("Hello", 10)`), which never reaches the cut branch.
   Proposed check: a table-driven unit test over these cases through the public function [to write]. Also loop `max` from 0 to length+1 over one fixed string mixing ASCII, "é" and emoji, asserting only "no panic" (unit, no new dependency) [to write].
   Why it matters for this change: `cargo test` passes today only because nothing exercises the cut path.

4. Finding: `preview_snippet` has the identical bug (`&body[..max]` on a byte offset, same `len()` check) and will panic on the same inputs. It is not in the report.
   Evidence: src/title.rs, `preview_snippet`.
   Proposed check: the same reproduction and boundary tests against `preview_snippet`, with the failing test written first (unit) [to write]. Scope is question 2.
   Why it matters for this change: fixing only the reported function leaves the same crash one call away in list previews.

5. Finding: ASCII behavior must not change.
   Evidence: src/title.rs; existing test `short_title_is_unchanged`.
   Proposed check: add one ASCII case that is cut, for example `truncate_title("Hello world", 5)`, and pin the result before touching the code (unit) [to write]. Run it unchanged after the fix. The expected value is derived from the doc comment and question 1, not from the output.
   Why it matters for this change: it shows the fix did not change the common case.

### Order of work
1. Settle the blocking questions below.
2. Write the failing emoji reproduction for `truncate_title` and confirm it fails with the panic, not for another reason ([rule]).
3. Write the ASCII, boundary and "fits within `max` characters" tests, and the same set for `preview_snippet` if it is in scope. Run them before the fix. The ASCII ones should pass and the multi-byte ones should fail.
4. Fix, then run the whole suite.

### Not verified by this plan
- Callers of `truncate_title` and `preview_snippet`. I have not read them and cannot tell whether any depend on byte-based lengths, for example a fixed-width column or a database field limit. Not read.
- Other tests, Cargo.toml and CI. Not read.
- "Character" as a user sees it. Emoji with skin tones, flags and joined sequences are several Unicode scalar values. Depending on question 3, a cut could split one visible emoji in two without panicking.
- Rendering of "…" or of cut text in the real UI.

### Questions for you (blocking)
1. Does `max` include the "…"? "At most `max` characters, adding … if cut" can mean the result is `max` characters plus the ellipsis, or `max` in total. The spec is silent, and the expected value of every cut test depends on it.
2. Is `preview_snippet` in scope? It has the same bug and no report. I propose fixing and testing it too. Say if you want it kept out.
3. What counts as a character: Unicode scalar values (`chars()`), or user-visible characters (grapheme clusters)? Grapheme handling would need a new dependency such as `unicode-segmentation`. I am not proposing it without your approval, because a dependency must be checked and justified ([rule]).
4. Today, titles with non-emoji multi-byte characters such as "é" are cut earlier than `max` characters, because length is counted in bytes. Counting characters changes that. Is anything relying on the byte behavior, or is it also a bug to fix?
5. The "never panics" property: is the loop over `max` enough, or do you want a property test ([rule])? A property test would need a dependency such as `proptest`.
