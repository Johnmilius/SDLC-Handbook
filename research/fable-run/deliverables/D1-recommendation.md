# D1. Recommendation

The recommendation is the first two sections (about 250 words). The one-line verdicts follow. Details and evidence are in D2 to D6. Claim numbers refer to [04-claim-check.md](../04-claim-check.md).

## The three decisions

1. **What the handbook contains.** A short field guide plus self-contained agent skills. Twelve pages are planned and eight are built first. Each page answers one decision, and each rule has a stable ID, a label for its evidence, and a source. One page, the quality bar, is the single reference for "good enough to merge".
2. **Which workflow to adopt first.** Three risk tiers that change how deep each stage goes and never skip a stage. Deterministic checks run first, then an independent AI review, then human review. Hard rules sit in hooks and CI, not in instructions.
3. **What to build first.** A `test-plan` skill. For a spec or a diff it says what could break, which check would catch it and at which layer, and what stays unverified. It is evaluated on eight synthetic cases against a plain request.

## The tradeoffs that drove this

- **Test planning before review.** The scores are close (4.3 against 4.1). Test planning wins because it fixes the oracle before the code exists, and a fresh-context review command already ships with the tool.
- **Short and cited before complete.** Large documents go stale. The cost is that four pages wait for v2.
- **Honest standards coverage before broad coverage.** Only standards whose text was read are cited by clause. SOC 2 and the ISO/IEEE standards were not read and are marked so.

## Verdicts on the hypotheses

| # | Verdict | One line |
|---|---|---|
| H1 | Supported | Short guide plus skills (C1, C2, C3) |
| H2 | Supported, with an addition | Test planning first; it must also guard test integrity (C4) |
| H3 | Modified | Layout holds, but each skill carries its own `reference.md`, because links back to `handbook/` break when a skill is copied |
| H4 | Modified | Links run through stable rule IDs; a skill may cite only IDs in its `reference.md` |
| H5 | Modified | Scale by risk (likelihood and impact), not by size (C12) |
| H6 | Supported in part | "Done when" has standard backing (C15). Dependencies and risk per step are [Proposed], used for Tier 3 only |
| H7 | Supported as [Proposed] | The order is reasoned from cost. No source tests it against parallel review |
| H8 | Modified | Independence of inputs is vendor guidance, untested for a same-model reviewer (C34). It is a filter, not a control |
| H9 | Supported | Instruction files are not guaranteed; hooks are deterministic (C18, C19) |
| H10 | Supported as [Proposed] | Tiering scales depth and never skips a stage (C14). The triggers have no source (C16) |
| H11 | Supported in part | Works for the free standards that were read. Not shown for paywalled ones or SOC 2 (C42, C44) |

## Verdicts on the outside claims (§3b)

| Claim | Verdict |
|---|---|
| Intent and specs as code | Adapt. Write criteria first, at the smallest size the tier needs. No outcome evidence found (C37) |
| Steering and skills files auto-load guardrails | Modified. Skills load by description match, and instructions are not guaranteed to be followed. Enforce with hooks |
| TDD and DDD baked in | TDD: supported, with test-integrity guardrails. DDD: not researched; left out of v1 |
| AI picks the lifecycle and skips stages | Rejected. No evidence found, and it contradicts Unit 07 |
| Separate author and reviewer | Supported in direction. An AI reviewer does not replace human review |
| Deterministic anchors; validate dependencies | Supported for dependency validation (measured for Python and JavaScript, not Rust). The order is [Proposed] |
| Adversarial AI testing | Not researched. Use it as a source of test ideas and check test quality with mutation testing |
| Humans focus review on high-risk areas | Supported as "risk sets depth". It does not mean skipping lines |
| "Drastically accelerates velocity" | Unsupported. The one trial opened found experienced developers 19% slower (study card in D3) |
| Enterprise framing | Marked [Large-org only] and kept out of v1 |
