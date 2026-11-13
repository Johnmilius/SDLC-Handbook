# Build state
Build plan version: 1.0
Last updated: 2026-11-13, build complete
Current step: COMPLETE (B0 to B8 done)
Branch: `handbook-v1` (baseline commit 27b0a92 is on `main`)

Done:
- [x] B4 templates (683a72f), B5 skill (2649bc0), B6 examples (55c8ba8), B7 eval kit (b02c35c), B8 pilot (656eb47) and build report (final commit)
- [x] B1 README, B2 quality bar, B3 all eight pages: committed one page per commit (see `git log`). Six pages were drafted by Sonnet workers and reviewed and corrected by the lead; quality-bar and plan-tests were written by the lead
- [x] B0 Set up: inputs read, gate PASS confirmed in `research/fable-run/00-run-state.md`, `git init`, baseline commit 27b0a92, state and deviations files created

D6 file checklist (the build scope):
- [x] B1 `README.md`
- [x] B2 `handbook/quality-bar.md`
- [x] B3 `handbook/workflow.md`
- [x] B3 `handbook/risk-tiers.md`
- [x] B3 `handbook/plan-tests.md`
- [x] B3 `handbook/verify-a-change.md`
- [x] B3 `handbook/working-with-agents.md`
- [x] B3 `handbook/standards-crosswalk.md`
- [x] B3 `handbook/sources.md`
- [x] B4 `templates/acceptance-criteria.md`
- [x] B4 `templates/test-plan.md`
- [x] B4 `templates/pr-description.md`
- [x] B4 `templates/adr.md`
- [x] B5 `skills/test-plan/SKILL.md`
- [x] B5 `skills/test-plan/reference.md`
- [x] B6 `examples/README.md`
- [x] B6 `examples/handbook-examples/Cargo.toml`
- [x] B6 `examples/handbook-examples/src/lib.rs`
- [x] B6 `examples/handbook-examples/src/config.rs`
- [x] B6 `examples/handbook-examples/src/title.rs`
- [x] B6 `examples/case-1-max-retries.md`
- [x] B6 `examples/case-2-truncate-title.md`
- [x] B7 `eval/RUN.md`
- [x] B7 `eval/rubric.md`
- [x] B7 `eval/cases/case-01.md` to `case-08.md` (8 files)
- [x] B7 `eval/keys/key-01.md` to `key-08.md` (8 files)
- [x] B8 `eval/results/pilot-2026-11-13.md`
- [x] B8 `build/BUILD-REPORT.md`
- [x] `build/00-build-state.md`
- [x] `build/DEVIATIONS.md`

Next step: None. The build is complete. Read `build/BUILD-REPORT.md`. Three decisions are waiting for the developer there (keep or change the first skill after a full evaluation; shorten the skill; merge `handbook-v1` into `main`).

Deviations logged: 4 (see build/DEVIATIONS.md)

Open issues:
- Rule IDs are fixed by the research files: QB-1 to QB-13 (D4), TP-1 to TP-13 (D5). Other pages use RT, WF, VC, AG, SC prefixes and anchors `s1` to `s18` in `handbook/sources.md`.
- Citations in pages give a label, the source with its edition and clause, and a link to the matching `s1` to `s18` anchor in `handbook/sources.md`, which holds the external link and date.
