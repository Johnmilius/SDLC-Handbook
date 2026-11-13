# How to run the evaluation

This measures one thing: does the [`test-plan` skill](../skills/test-plan/SKILL.md) give better output than a plain one-line request to the same model? The answer decides whether the skill is worth keeping and expanding. Source: research deliverable D5, section 3.

You can run it from this file alone. Budget about two hours for the full protocol.

## What is here

| Path | Holds |
|---|---|
| `cases/case-01.md` to `case-08.md` | The material for each case and its one-line plain request |
| `keys/key-01.md` to `key-08.md` | Answer keys. **Nothing that runs a case may read this folder** |
| `rubric.md` | The scoring rubric and score sheet |
| `results/` | One file or folder per evaluation run |

| # | Case | Kind |
|---|---|---|
| 01 | Bounded config value over HTTP | New feature |
| 02 | Title helper that crashes on emoji | Bug fix, existing code |
| 03 | Change how user IDs are stored | High-risk change |
| 04 | Agent-written feature diff | Agent diff with AI-specific defects |
| 05 | Page count that is one too few | Small fix (checks proportion) |
| 06 | Clean up a shipping calculation | Refactor of untested code |
| 07 | Password-reset tokens that expire | Feature touching auth |
| 08 | Agent's "fix CI" diff | Agent diff, test integrity |

## Setup

1. Make a scratch folder **outside this repository**, so a session cannot wander into `eval/keys/` or `examples/`.
2. For the skill condition, copy `skills/test-plan/` into the scratch folder at `.claude/skills/test-plan/`. That is the only file from this repository the session should see.
3. Decide the model and write it down. Use the same model for every run of both conditions.

## Running the cases

For each case, do **three runs per condition**, each in a fresh session with no earlier conversation. That is 48 outputs.

**Skill condition.** Start a fresh session in the scratch folder. Send:

```
/test-plan
<paste everything under "Material" from the case file>
```

**Plain condition.** Start a fresh session in an empty folder with no skill installed. Send the case's "Plain request" line, then the same material:

```
<the plain request line from the case file>
<paste everything under "Material" from the case file>
```

Save each output as a file in `results/<date>/raw/`, named `case-NN-<condition>-<run>.md`. Do not edit the outputs.

## Blinding

1. Copy each raw output to `results/<date>/blind/` under a random name (for example a 6-character code). Keep the mapping from code to case, condition and run in a file you do not open until scoring is done.
2. In the blind copies, remove what gives the condition away:
   - handbook rule IDs such as `TP-3` or `QB-8`, and the `(reference.md#...)` links;
   - the fixed section headings of the skill format, if you want a stricter blind. Removing them also removes structure the rubric rewards, so say in the results which you did.
3. Criterion (b) needs the rule IDs to count invented references. Score (b) last, from the raw copy.

## Scoring

1. **Judge model.** Give a fresh session the case material, the key for that case, `rubric.md`, and the blind outputs for that case. Ask for a score per criterion with a one-line reason each, plus counts of false positives and invented references. Use this prompt:

   ```
   You are scoring outputs against an answer key. Do not guess which tool produced them.
   For each output, give 0, 1 or 2 for criteria (a) to (d) in the rubric, one line of reason per score,
   the number of false positives, and the number of invented references. Quote the output where you can.
   ```

2. **Hand check.** Score at least one output per case yourself, without looking at the judge's scores. Record every disagreement of 2 points or more on the total, and who you think was right.
3. Fill in the score sheet in `rubric.md`, then unblind.

## Pass bar

The skill passes only if **all** of these hold:

1. Skill average total is at least the plain average **plus 1.5** (out of 8).
2. **Zero invented references** in the skill outputs.
3. On case 05, the skill averages at least 1.5 on criterion (d). A skill that gold-plates small fixes fails.
4. No case where the skill average is more than 1 point below the plain average.

## What to do with the result

| Result | Next step |
|---|---|
| Pass on two consecutive revisions of the skill, and the skill has caught a real issue in at least 5 real changes | Expand: write the v2 pages and build the second skill, `review-change` |
| Fails bar 1 | The plain model may already plan tests well. Shrink the skill to its guardrails and consider building `review-change` first |
| Fails bar 2 | Tighten the evidence rule in `SKILL.md` and rerun before anything else |
| Fails bar 3 | Shorten the Tier 1 path and rerun |
| Strong on cases 04 and 08, weak on the rest | The skill is acting as a reviewer, not a planner. Reconsider which skill comes first |

## Limits to keep in mind

- Eight synthetic cases and three runs show a direction, not a precise effect size.
- The cases were written by the same process that designed the skill. Add a few cases from your own past changes when you can.
- A judge model has its own blind spots. That is why the hand check is required.
- Cases 04 and 08 are diffs, which suit a review skill better. They are in the set on purpose, as the fairest test of the first-skill choice.
