# Build state
Build plan version: 1.0
Last updated: 2026-11-13 05:50 MST
Current step: B0 (done), B1 next
Branch: `handbook-v1` (baseline commit 27b0a92 is on `main`)

Done:
- [x] B0 Set up: inputs read, gate PASS confirmed in `research/fable-run/00-run-state.md`, `git init`, baseline commit 27b0a92, state and deviations files created

D6 file checklist (the build scope):
- [ ] B1 `README.md`
- [ ] B2 `handbook/quality-bar.md`
- [ ] B3 `handbook/workflow.md`
- [ ] B3 `handbook/risk-tiers.md`
- [ ] B3 `handbook/plan-tests.md`
- [ ] B3 `handbook/verify-a-change.md`
- [ ] B3 `handbook/working-with-agents.md`
- [ ] B3 `handbook/standards-crosswalk.md`
- [ ] B3 `handbook/sources.md`
- [ ] B4 `templates/acceptance-criteria.md`
- [ ] B4 `templates/test-plan.md`
- [ ] B4 `templates/pr-description.md`
- [ ] B4 `templates/adr.md`
- [ ] B5 `skills/test-plan/SKILL.md`
- [ ] B5 `skills/test-plan/reference.md`
- [ ] B6 `examples/README.md`
- [ ] B6 `examples/handbook-examples/Cargo.toml`
- [ ] B6 `examples/handbook-examples/src/lib.rs`
- [ ] B6 `examples/handbook-examples/src/config.rs`
- [ ] B6 `examples/handbook-examples/src/title.rs`
- [ ] B6 `examples/case-1-max-retries.md`
- [ ] B6 `examples/case-2-truncate-title.md`
- [ ] B7 `eval/RUN.md`
- [ ] B7 `eval/rubric.md`
- [ ] B7 `eval/cases/case-01.md` to `case-08.md` (8 files)
- [ ] B7 `eval/keys/key-01.md` to `key-08.md` (8 files)
- [ ] B8 `eval/results/pilot-2026-11-13.md`
- [ ] B8 `build/BUILD-REPORT.md`
- [x] `build/00-build-state.md`
- [x] `build/DEVIATIONS.md`

Next step: B1. Write the root `README.md` (what the handbook is, pick-a-tool-by-task routing, folder map, how to use a skill in another repo, status, maintenance rule from D4 section 5). The D4 folders already exist. Commit, then B2 (`handbook/quality-bar.md` from D4 section 1).

Deviations logged: 2 (see build/DEVIATIONS.md)

Open issues:
- Rule IDs are fixed by the research files: QB-1 to QB-13 (D4), TP-1 to TP-13 (D5). Other pages use RT, WF, VC, AG, SC prefixes and anchors `s1` to `s18` in `handbook/sources.md`.
- Citations in pages take the form `[Label] Source edition §clause ([S15](sources.md#s15))`; `sources.md` holds the external link and date.
