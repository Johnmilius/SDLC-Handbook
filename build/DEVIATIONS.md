# Deviations from the blueprint

Each entry: what the blueprint or plan said · what was done · why · whether the developer should review it.

| # | Step | Blueprint or plan said | What was done | Why | Review? |
|---|---|---|---|---|---|
| 1 | B0 | Baseline commit of the existing files in `cse370-prepare/` and `research/` | `research/gemini-prompts.md` was left untracked and was not opened | Both plans say not to touch that file. Leaving it untracked is the reading that touches it least | Yes, low priority. Run `git add research/gemini-prompts.md` if you want it in history |
| 2 | B0 | Commit after every build step (no branch named) | The baseline commit is on `main`. All build commits are on a branch, `handbook-v1` | You can review the whole build as one diff (`git diff main..handbook-v1`) and merge when satisfied. Nothing was merged or pushed | Yes. Merge with `git switch main && git merge handbook-v1`, or keep working on the branch |
| 3 | B3 | Every claim is labeled [Standard], [Established], [Observed] or [Proposed] | Claims that rest on vendor documentation carry the marker `[T3 docs]` instead of one of the four labels | Vendor docs say how a tool works. They fit none of the four labels, and calling them [Established] would overstate them. The marker is explained in the README and in `handbook/sources.md` | Yes, low priority |
