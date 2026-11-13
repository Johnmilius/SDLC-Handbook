# Unit 07 — The SDLC Revisited

Source: https://byui-cse.github.io/cse370-course/unit07/prepare.html
*(Study notes, paraphrased from the course page.)*

## Key points
- **Every SDLC phase is about managing risk.**
- Whatever methodology you use, give **every phase** proper attention.

## Recap of the phases
| Phase | Unit |
|---|---|
| Requirements elicitation | 02 |
| Design | 03 |
| Implementation | (other courses) |
| Testing and verification | 04 |
| Release and maintenance | 05 |

## Methodology vs SDLC (again)
- **SDLC = what. Methodology = how.**
- Waterfall runs the phases once, in order. Agile cycles through all of them repeatedly, maybe every 2 weeks.
- 🚩 **Red flag:** using a methodology as an excuse to skip a phase, e.g., "We don't need design, we're agile." A methodology can change the *timing, frequency or style* of a phase, never whether it happens at all.

## Shared responsibility
- 🚩 Red flags: "Not my problem." "The testers handle that." That's "throwing it over the fence."
- **DevOps** fights this by joining Dev and Ops so everyone shares ownership of the whole process.

## Shift Left
Do tasks, or at least *think about* later phases, **earlier** in the SDLC, so issues are found when they're cheap to fix.

**Principles**
- **Early testing**: start testing work during requirements and design (unit, integration, even some acceptance testing)
- **Early involvement**: testers, QA and Ops are part of the project from day one
- **CI/CD**: integrate and test often, with automated tests on every change
- **Automation**: automated unit and integration tests and code analysis give immediate feedback
- **Static code analysis**: catch errors and vulnerabilities before the code even runs, which avoids tech debt
- **DevOps practices**: IaC and automated deployments enable early, frequent validation

**Benefits**
- **Defects found early** are much cheaper and easier to fix
- **Better quality** from continuous testing
- **Faster time to market**, with fewer late-stage surprises
- **More collaboration** and early alignment
- **Lower risk**, because mitigation starts sooner

## Power skills (soft skills)
Employers weigh technical skill in interviews, but they look hardest at whether you'll **work well with the team** ("cultural fit," sometimes a whole interview round).

BYU-Idaho institutional learning outcomes:
- **Sound thinkers**: frame and solve problems creatively and critically
- **Effective communicators**: ideas that are purposeful, organized and clear
- **Skilled collaborators**: work well with others toward a shared vision
- **Disciples of Jesus Christ**: the university's core mission outcome. The course encourages modeling Christ's teachings in the workplace.

## Suggested follow-up questions
- Explain ___ in the SDLC
- Benefits or drawbacks of ___?
- What's the purpose of shifting left?
- What power skills matter for a developer?
- Why is software engineering hard?
