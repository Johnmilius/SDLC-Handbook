# Unit 05 — Release and Maintenance

Source: https://byui-cse.github.io/cse370-course/unit05/prepare.html
*(Study notes, paraphrased from the course page. The CI/CD section is directly relevant to the CI/CD tool set.)*

## Key points
- Deployed software can't be forgotten. It needs ongoing **care and feeding**.
- **Preventive maintenance** keeps you able to change things later.
- **Technical debt** is normal. Know it's there and plan for it, or you'll live with it for a long time.

## Four types of maintenance
| Type | Purpose | Examples |
|---|---|---|
| **Corrective** | Fix defects found after release | Root cause → fix → test → patch |
| **Adaptive** | Keep working in a changing environment | New OS or hardware, third-party changes, new regulations |
| **Perfective** | Improve and enhance for users | New features, faster algorithms, better UI, stronger security |
| **Preventive** | Head off problems before they happen | Refactoring, audits, monitoring, **updating dependencies** |

## Technical debt
The future cost of choosing the quick or easy solution over the robust one.
- **How it happens:** shortcuts to hit deadlines (sloppy code, skipped docs, skipped tests). It's hard to make time to go back.
- **It compounds:** like money owed, it gathers "interest" as the codebase grows.
- **Types:**
  - *Design debt*: poor architecture
  - *Code debt*: hard to read, maintain or extend
  - *Testing debt*: too little coverage, skipped tests
  - *Documentation debt*: missing or stale docs
- **Effects:** lower productivity • higher maintenance cost • more failures • harder to scale
- **Managing it:** prioritize refactoring, code reviews and QA. Balance speed now against health later, and pay it down regularly.

## Change control
As usage grows, define a careful process for changes: approvals to roll out or roll back, and doc and artifact updates.
- Agile-style processes favor **small, frequent deployments**. Each one is lower risk and easier to roll back, and the process becomes routine.

## CI/CD
Automate **build → test → deploy**.

- **Continuous Integration (CI):** merge to a shared repo **several times a day**. Each merge triggers an automated build, tests and artifacts. The goal is to catch integration errors early and keep the mainline working.
- **Continuous Delivery (CD):** every build that passes is **deployment-ready**. It's deployed automatically to staging for validation, then the *same artifacts* go to production with little manual work.
- **Continuous Deployment:** every passing build goes **straight to production with no human step**. This needs **high confidence in the automated tests**.

**Pipeline components**
- **Version control** (Git, plus some SVN and Mercurial)
- **CI server**: Jenkins, GitHub Actions, Travis CI, CircleCI, GitLab CI/CD
- **Automated tests**: unit, integration, E2E
- **Deployment pipeline**: build → test → staging → production
- **Infrastructure as Code**: Terraform, AWS CloudFormation, for consistent environments

**Benefits:** faster time to market • better code quality (bugs caught early) • less risk (less human error) • more collaboration

**Drawbacks to watch for**
- Deployments stop being "events", so teams may skip big-picture thinking about each change.
- The focus on getting it working can let **technical debt** pile up.
- It's easy to forget **documentation** (requirements, institutional knowledge), so knowledge ends up only in engineers' heads.
- All of these can be managed, but only if you're aware of them.

## Regression testing
- Re-test the **unchanged** parts after a change to make sure they still work.
- Manual or automated. Automated is preferred, and it fits naturally in CI/CD: run on every commit.

## Rolling out features: feature flags (toggles / switches)
Ship the code, then enable or disable it **at runtime** based on conditions.
- **Dynamic control**: turn features on or off without redeploying
- **Gradual rollouts**: start with a subset of users and monitor
- **A/B testing**: compare variants with data
- **Risk mitigation**: switch off a bad feature instead of rolling back the whole release
- **Continuous delivery**: smaller, more frequent releases
- **Centralized logic**: easier to see what's on where
- **Personalization**: target user segments

## Supporting multiple versions
Needed for offline or downloaded software, or services whose callers can't all upgrade at once. With microservices, put the **version in the API call**.
- **Versioning scheme**: **SemVer** `MAJOR.MINOR.PATCH`
- **LTS releases**: extended support for stability
- **Version control**: a branch per major version
- **Isolated environments** per version
- **Backward compatibility** wherever possible
- **Documentation**: release notes, upgrade guides, compatibility info
- **Parallel support**: budget for fixes and security patches on every supported version
- **Communication**: end-of-life dates and upgrade paths
- **Automated tests per version** in CI/CD
- **Migration paths**: tools and scripts for upgrading

## End of life (EOL) / sunsetting
- Plan for retirement from the start.
- Key: **clear communication** of timing, process, rationale and remaining support.
- Users resist change (familiarity, cost). Point out the benefits (security, compatibility) and provide migration tools and guidance.

### EOL of dependent libraries
- **Cost of upgrading:** dev time, regression risk, and **cascading** upgrades to other libraries.
- **Cost of not upgrading:** **security vulnerabilities**, less vendor support, compatibility and compliance gaps, growing technical debt.

## Migrating infrastructure (steps)
1. **Assess and plan**: inventory the current setup, set goals and scope, assess risk, write a migration plan
2. **Prepare**: back up data, document dependencies, update docs, notify stakeholders
3. **Set up the new infrastructure**: provision, install dependencies, configure the network (IPs, DNS, firewalls, load balancers)
4. **Execute**: migrate data (integrity), install software, test, run UAT
5. **Validate**: functional, performance and security testing
6. **Go live**: low-traffic window, deploy with a rollback plan, monitor closely
7. **Post-migration support**: training, doc updates, performance tuning
8. **Evaluate**: lessons learned, gather feedback

> Note: a migration *is itself* a software project, so the same SDLC steps apply.

## Monitoring
**Performance metrics:** response time • throughput • resource use (CPU, memory, disk I/O, network) • error rates • availability (uptime)

**Tools and techniques:**
- **Logging**: capture and analyze component logs
- **Metrics collection**: real-time and trend dashboards
- **Alerting**: notify when thresholds are crossed
- **Profiling and tracing**: find code hotspots and follow requests through the system

**Prognostic / preventive monitoring:** don't just react. Watch for anomalies and forecast future needs so you can prepare ahead.

## Suggested follow-up questions
- Explain CI/CD / technical debt / feature flags
- Benefits or drawbacks of ___?
- How do you upgrade software while it's in use?
- How do you retire a product?
- What should you monitor on a live product?
- Why is maintenance hard?
