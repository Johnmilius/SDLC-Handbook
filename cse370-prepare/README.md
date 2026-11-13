# CSE 370 — Prepare Material (Study Notes)

Course site: https://byui-cse.github.io/cse370-course/
Notes paraphrased from each unit's "Prepare" learning-activity page (captured 2026-11-12).

| Unit | Topic | File |
|---|---|---|
| 01 | The Software Development Life Cycle: DevOps, Waterfall, V-Model, Agile, Scrum, Kanban | [unit01-sdlc.md](unit01-sdlc.md) |
| 02 | Requirements Elicitation: user stories, use cases, functional vs non-functional, quality criteria | [unit02-requirements.md](unit02-requirements.md) |
| 03 | Software Design: levels of design, UML class and sequence diagrams, data dictionaries | [unit03-design.md](unit03-design.md) |
| 04 | Software Testing: pyramid, coverage, white/black box, boundary values, traceability | [unit04-testing.md](unit04-testing.md) |
| 05 | Release and Maintenance: tech debt, CI/CD, feature flags, versioning, monitoring | [unit05-release-maintenance.md](unit05-release-maintenance.md) |
| 06 | Project Management: psychological safety, risk, Iron Triangle, Brooks' Law, estimation | [unit06-project-management.md](unit06-project-management.md) |
| 07 | The SDLC Revisited: Shift Left, shared responsibility, power skills | [unit07-sdlc-revisited.md](unit07-sdlc-revisited.md) |

## Themes across the whole course
1. **Every phase is about managing risk.**
2. **Traceability:** requirement → design → code → test.
3. **No methodology is an excuse to skip a phase.**
4. **Shift Left:** think about testing and operations from day one.
5. **Software is built by people:** communication, honesty, psychological safety.

## Ideas for the TDD / CI/CD skill
- **Traceability:** each test should map to a requirement or acceptance criterion (Unit 02 Given/When/Then → Unit 04 matrix).
- **Acceptance criteria as test specs:** Given/When/Then translates directly into test cases.
- **Test design techniques:** boundary values and equivalence partitions (Unit 04).
- **Coverage is a signal, not a goal:** watch for gamed coverage (Unit 04).
- **Pyramid layering in CI:** unit (local and CI) → integration (CI) → E2E, performance and UAT (staging) → monitoring (production).
- **Regression suite on every commit**, and continuous deployment only with high test confidence (Unit 05).
- **Testing debt** is one kind of tech debt, so track it (Unit 05).
- **Static analysis and shift-left automation** as pipeline gates (Units 04, 07).
