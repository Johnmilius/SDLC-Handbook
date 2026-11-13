# Scoring rubric

Score each output on four criteria, 0 to 2 each, for a total out of 8. Score against the answer key for that case. Source: research deliverable D5, section 3.

| Criterion | 0 | 1 | 2 |
|---|---|---|---|
| **(a) Catches the important risk** | Misses most of the key's "Blocking" items | Catches some "Blocking" items | Catches every "Blocking" item |
| **(b) Grounded in evidence** | References are invented or missing | Mostly real references, some vague | Every finding cites a real file, test or spec sentence from the case material |
| **(c) Explains the reasoning** | A bare list, or code only | Some findings explain why | Each finding runs: finding, evidence, proposed action, why it matters |
| **(d) Proportional to the change** | Far too much or far too little for the tier | Somewhat off | Depth matches the tier, and nothing from the key's "Over-reach" list appears |

Notes for the scorer:

- Criterion (a) is about substance. An output that writes a test which would catch the risk counts, even if it never names the risk.
- "Flag" items in a key earn no extra points. Use them to break a tie between 1 and 2.
- For criterion (d), a long output is not automatically over-reach. Judge it against the key's tier and "Over-reach" list.
- A stop-and-ask question counts as catching a risk when the key says the right move is to ask.

## Also record for each output

| Measure | How to count |
|---|---|
| **False positives** | Findings that are wrong, or irrelevant to this change |
| **Invented references** | Files, functions, tests, line contents or handbook rule IDs that do not exist in the case material or in `skills/test-plan/reference.md` |
| **Review time** | Minutes you needed to read the output and decide what to do with it. Hand-scored outputs only |

## Score sheet

One row per output. Keep the condition column hidden until scoring is finished.

| Case | Run | Blind label | (a) | (b) | (c) | (d) | Total | False positives | Invented references | Review min | Condition (fill in last) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 01 | 1 | | | | | | | | | | |
