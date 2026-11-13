# Unit 04 — Software Testing

Source: https://byui-cse.github.io/cse370-course/unit04/prepare.html
*(Study notes, paraphrased from the course page. This is the unit most relevant to the TDD / testing skill.)*

## Key points
- **Testing is about managing risk.**
- Everyone on the team owns quality across the *whole* SDLC.
- **Shift Left**: bring testing thinking into requirements, design and development, not just the end.

## Why test, if you can't prove there are no bugs?
Dijkstra's point: tests can show that bugs *exist* but can't show they're *absent*. So testing doesn't prove correctness. It **finds and reduces risk**.

**Goals of testing:** verify and validate against requirements • find defects • improve quality • ensure security • ensure performance.

## How much testing is enough?
- It depends on context and the cost of failure: aircraft, medical and finance need more than an internal tool. Even inside one product, some parts are more critical than others.
- Name the possible failures, then set your tolerance for each.
- ⚠️ **Trap:** calling something "non-critical" to excuse skipping tests. Users count on even "small" systems (a homework submission tool, a grocery scale). Example: the **UK Post Office Horizon scandal**, where accounting-software bugs led to wrongful convictions and ruined lives.
- **If software is worth writing, it's worth making sure it works.**

## Types of testing

### Functional testing: does it do what it should?
**The V-Model** pairs each development phase with a test phase. **The testing pyramid** shows the same idea as layers:

| Layer | What it tests | Volume / speed / cost | Example tools |
|---|---|---|---|
| **Unit** (base) | Single functions or components, given inputs → expected outputs | Very many, very fast, cheap. Can run on simulators | JUnit, NUnit, PyTest |
| **Integration** | Components working **together** | Fewer, slower, more setup. Real hardware where it applies | JUnit/NUnit, Spring Test, Postman |
| **System / end-to-end** | Full real-user scenarios from start to finish | Few, slow, complex, expensive | Selenium, Cypress, Appium |
| **UAT** (top) | Stakeholders confirm requirements are met | **Scripted** use cases (not casual clicking), in a production-like environment | — |

### Non-functional testing: how well does it work?
- **Performance**: speed, responsiveness and stability at scale
  - *Load* (expected volume) • *Stress* (beyond the limits) • *Soak* (long duration) • *Spike* (sudden surge)
  - Often finds problems that never show up in development environments
- **Usability**: UX. Nielsen Norman's 5 components:
  - **Learnability** • **Efficiency** • **Memorability** • **Errors** (how many, how bad, how recoverable) • **Satisfaction**
  - Real users do tasks while being recorded
- **Security**: authentication, authorization, data protection, logging
  - *Vulnerability scanning* (automated search for known issues, missing patches, misconfigurations)
  - *Penetration testing* (experts simulate real attacks)
  - *Security auditing* (review code and config against policies)
  - *Compliance* (accessibility, financial and other regulations)

## Designing and measuring tests

### Code coverage
- % of code exercised by tests. Example: an if/else needs tests for **both** branches.
- 100% is hard to reach, **doesn't mean there are no bugs**, and may not be worth chasing.
- ⚠️ It's easy to **game**: padding coverage with tests on trivial getters and setters.

### White-box vs black-box
| | White-box (clear / glass box) | Black-box (behavioral) |
|---|---|---|
| Knowledge | Full view of the code and logic | Only inputs and expected outputs |
| Based on | Paths, branches, loops, data flow | Requirements, specs, user stories |
| Pros | Finds logic flaws early, exercises all paths, spots inefficient code | No coding skills needed, user focused, good for functional, usability and performance testing |
| Cons | Needs programming skill, slow and complex, weak at UX and integration | Misses internal logic errors, limited structural coverage, can miss edge cases |

### Boundary value analysis
Bugs cluster at the **edges** of input ranges. For a valid range of **1–100**, test:
`0, 1, 2, 99, 100, 101`

### Equivalence partitioning
Split inputs into groups the software should treat the same way, then test one or more representatives of each:
- `< 1` (invalid) • `1–100` (valid, e.g., 50) • `> 100` (invalid)

## Other quality practices
- **Code reviews**: peers check changes before merge. They catch defects, improve readability and spread knowledge.
- **Pair programming**: one person writes while the other reviews in real time.
- **Static code analysis**: tools find errors and standards violations *without running* the code.

## Where tests run
| Environment | Use |
|---|---|
| **Local dev** | Unit (and some integration) tests for fast feedback |
| **CI** | Unit, integration and **regression** tests on every build |
| **Staging / pre-prod** | Mirrors production (minus live traffic). E2E, performance and UAT |
| **Production** | Only **non-intrusive** checks: monitoring, logging, A/B tests |

Production rollout strategies:
- **Canary deployment**: release to a small subset of users first.
- **Blue/green**: two live versions side by side, so you can switch over or back.

**Simulation environments** are useful for hardware, hard-to-copy external parts, or extreme load. They aren't the real thing, so pair them with tests in real environments.

## Traceability
Tests should trace back to requirements, design and code.

**Traceability matrix**: a table mapping requirements to test cases, with status:

| Req ID | Requirement | Test ID | Test case | Status |
|---|---|---|---|---|
| R1 | Login | TC1 | Valid credentials log in | Passed |
| R2 | Password reset | TC2 | Reset flow works | Failed |
| R3 | Secure checkout | TC3.1 / 3.2 / 3.3 | Add to cart → enter payment securely → confirmation | Passed |

- **Forward**: requirement → tests (is every requirement covered?)
- **Backward**: test → requirement (does every test have a purpose?)
- **Bidirectional**: both
- **Benefits:** coverage • impact analysis • defect tracking • accountability and audit trail • regulatory compliance
- Tools: JIRA, TestRail, requirements-management tools

## The tester mindset
Developers solve the problem. Testers look for **how it fails**.
- Focus on quality assurance • attention to detail and edge cases • the **user's perspective** (empathy)
- **Skeptical**: assume there are bugs and challenge assumptions
- Analytical: structured methods and root-cause analysis
- Collaborative: clear defect reports and documentation
- Adaptable: learn from past failures

## When you might *not* fix a bug
- **Low impact**: minor, or an easy workaround exists
- **Low priority**: rare, or in a non-critical area
- **High cost**: the fix is complex, or **regression risk** is high
- **Resource limits**: time or budget
- **Upcoming changes**: a redesign is planned or the feature is deprecated
- **User acceptance**: no complaints, or users actually prefer the behavior
- **Strategy**: business priorities or competitive features come first
- **Can't reproduce it**

## Who's responsible for correctness?
**Everyone.** Not just QA or UAT.
- **Shift Left**: think about testing while writing requirements, while designing and while coding.
- As a developer, **you own the correctness of your code**. You can't hand that off to testers.

## Suggested follow-up questions
- Explain white-box / performance / soak testing
- What bugs does ___ testing find?
- Difference between ___ and ___?
- What bugs can remain even after ___?
- Why is testing hard?
