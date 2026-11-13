# Unit 01 — The Software Development Life Cycle (SDLC)

Source: https://byui-cse.github.io/cse370-course/unit01/prepare.html
*(Study notes, paraphrased from the course page.)*

## Key points
- Every project passes through the SDLC phases in some form. Only the order, frequency and repetition change.
- Understanding the SDLC and picking a fitting methodology helps you spot risks early.
- No single methodology is best for everyone.
- Software is built by people, so communication and collaboration matter most.

## Context
- Software engineering is young and still changing. There's no governing body that defines "the" process. What we have instead are principles and good and bad practices the community has tested.
- **SDLC = the *what*** (the phases the software goes through).
- **Methodology = the *how*** (how the team coordinates and manages those phases).

## The SDLC phases
Every project does some version of these: decide what to build → plan how → build it → verify it → deliver it → fix and enhance it.

| Phase | Course unit |
|---|---|
| Requirements elicitation: gather functional and non-functional requirements | Unit 02 |
| Design: architecture and system design | Unit 03 |
| Implementation: write the code (not covered; other courses do this) | — |
| Testing and verification: make sure it works and has no defects | Unit 04 |
| Release and maintenance: ship it, support it, update it | Unit 05 |

## DevOps
- In the past, devs built the product and handed it to IT Ops to deploy. DevOps joins **Dev + Ops** across the whole life cycle.
- Goal: a shorter cycle and continuous delivery of high-quality software.

**Core principles**
- **Collaboration culture**: break down silos and share responsibility.
- **Automation**: of integration, testing, deployment and infrastructure. It cuts human error.
- **CI/CD**
  - *Continuous Integration*: merge to a shared repo often, with automated build and tests on every merge.
  - *Continuous Delivery*: after the tests pass, automatically deploy to staging or production.
  - *Continuous Deployment*: every change that passes the tests goes straight to production.
- **Infrastructure as Code (IaC)**: infrastructure is defined in version-controlled config files, which gives repeatable environments.
- **Monitoring and logging**: ongoing insight into how healthy the system is and how it performs.
- **DevSecOps**: security built into every stage, including automated security tests and compliance checks.

**DevOps stages (an infinite loop):** Plan → Create → Verify → Package → Release → Configure → Monitor → (back to Plan)
- *Plan*: requirements, business metrics, release and security planning
- *Create*: design and code
- *Verify*: acceptance, regression, performance and security testing
- *Package*: approvals, packaging, staging releases
- *Release*: coordinate, deploy or promote, fallback and recovery
- *Configure*: provision infrastructure (storage, DB, network)
- *Monitor*: release impact, infrastructure performance, user experience

## Traditional (sequential) methodologies

### Waterfall
Linear phases, each finished before the next starts: Requirements analysis → System design → Implementation → Integration and testing → Deployment → Maintenance.
- **Pros:** heavy documentation, clear milestones and deliverables. Good for regulated or well-defined work (e.g., government contracts).
- **Cons:** rigid, so late discoveries are costly. The schedule it promises usually slips once unknowns appear.
- Takeaway: its *phases* are still the necessary work. Newer methods just move through them faster and more iteratively.

### V-Model (Verification and Validation)
An extension of Waterfall where **each development phase has a matching test phase**. For example, requirements analysis is checked by user acceptance testing.
- Strength: each phase produces documentation detailed enough to verify it. Verification is a systematic review at every level, not a single event at the end.

## Agile

### Agile Manifesto (2001)
It values the items on the left more than those on the right (the right side still has value):
- Individuals and interactions **over** processes and tools
- Working software **over** comprehensive documentation
- Customer collaboration **over** contract negotiation
- Responding to change **over** following a plan

**12 principles (summarized):** satisfy the customer through early, continuous delivery • welcome changing requirements • deliver often (weeks, not months) • business and devs work together daily • trust motivated people • face-to-face conversation is best • working software measures progress • keep a sustainable pace • technical excellence and good design boost agility • simplicity (maximize work *not* done) • self-organizing teams produce the best designs • reflect and adjust regularly.

### Scrum
Iterative work in time-boxed **sprints** (often 2 weeks).

- **Artifacts**
  - *Product backlog*: ordered list of everything the product might need. It keeps changing.
  - *Sprint backlog*: items chosen for this sprint, plus the plan to deliver them.
  - *Product increment*: the usable, "done" version of the product at the end of the sprint.
- **Roles**
  - *Product owner*: voice of the customer. Sets backlog **priority** but not sprint scope (the team estimates).
  - *Scrum master*: guards the process and removes blockers. Not the manager or tech lead.
  - *Development team*: cross-functional and self-organizing.
- **Events**
  - *Sprint*: 1–4 weeks, same length every time.
  - *Sprint planning*: choose backlog items and set a sprint goal. Break tasks down so each takes **< 1 day**.
  - *Daily scrum / stand-up*: 15 minutes. What I did, what I'll do, what's blocking me.
  - *Sprint review*: demo the increment to stakeholders and adjust the backlog.
  - *Retrospective*: what went well, and what we'll improve.

### Kanban
- Comes from Japanese manufacturing. A visual workflow focused on continuous flow without overloading the team.
- **Kanban board**: columns (Backlog / To Do / In Progress / Done) with cards that move across. "Swimlanes" are rows for different roles or teams.
- Key rule: **limit work in progress (WIP)**.
- Looser than Scrum: no fixed roles, events or iterations. Kanban optimizes *continuous flow*, while Scrum uses *time-boxed iterations*.

### ScrumBan
Scrum's meetings and process combined with a Kanban board for sprint tasks. Cards often move during stand-up.

### Agile and the future
- Almost every process today borrows from Agile, and it does help teams deliver reliably.
- Critique: teams can over-formalize Agile until it becomes rigid. Don't lose sight of the spirit, which is to talk to the customer and adapt.

## Choosing a methodology
| Factor | Leans Waterfall / V-Model | Leans Agile |
|---|---|---|
| Requirements | Stable, well defined | Changing, evolving |
| Team experience | Traditional | Experienced with Agile |
| Size and complexity | Large and complex (needs structure) | Smaller, less complex |

## Glossary
SDLC • Methodology • DevOps • DevSecOps • CI / CD (delivery vs deployment) • IaC • Waterfall • V-Model • Agile Manifesto • Scrum • Sprint • Product/Sprint backlog • Increment • Definition of done • Product owner • Scrum master • Stand-up • Retrospective • Kanban • WIP limit • Swimlane • ScrumBan
