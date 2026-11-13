# Build state
Build plan version: 1.0
Last updated: 2026-11-13 (after second usage-limit stop) B3 complete
Current step: B5 (first skill)
Branch: `handbook-v1` (baseline commit 27b0a92 is on `main`)

Done:
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

Next step: B5. Re-read the Claude Code skills docs (https://code.claude.com/docs/en/skills) for frontmatter and loading rules, then write `skills/test-plan/SKILL.md` to the D5 section 1 spec and `skills/test-plan/reference.md` (pinned copy of TP-1 to TP-13, RT-1 to RT-4, QB-2, QB-3, QB-8, QB-13). Commit. Then B6 examples.

Deviations logged: 3 (see build/DEVIATIONS.md)

Open issues:
- Rule IDs are fixed by the research files: QB-1 to QB-13 (D4), TP-1 to TP-13 (D5). Other pages use RT, WF, VC, AG, SC prefixes and anchors `s1` to `s18` in `handbook/sources.md`.
- Citations in pages take the form `[Label] Source edition §clause ([S15](sources.md#s15))`; `sources.md` holds the external link and date.
