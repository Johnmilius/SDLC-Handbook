# Unit 06 — Project Management

Source: https://byui-cse.github.io/cse370-course/unit06/prepare.html
*(Study notes, paraphrased from the course page.)*

## Key points
- **People write software, and people aren't robots.**
- Successful teams build a culture of **psychological safety**.
- **Project management is about managing risk.**

## Software is a people project
People have feelings, make mistakes, get creative, get frustrated, take vacations. No plan survives contact with reality, because the project will never match the spreadsheet. **People are the most valuable resource**, and how they work together decides whether the project succeeds.

## Psychological safety
An environment where people can take risks, share ideas and disagree **without fear of judgment or punishment**. The aim is to get as many good ideas into the open as possible, even unpopular or wrong ones. Everyone helps build it, including junior members.

**Strategies**
- **Open communication**: active listening, managers asking for input. Remember introverts may prefer to share ideas outside meetings or asynchronously.
- **Treat failure as learning**: reward experimentation, reflect instead of blaming.
- **Handle conflict constructively**: focus on solutions. *Seek first to understand, then to be understood.*

The course connects this to the principle that "revelation is scattered among us": counseling together with people who see things differently produces better solutions than any one person alone.

## Risk management
Failure will happen, especially when people and uncertainty are involved. Don't freeze up. **Identify risks and manage them.**

| Category | Examples |
|---|---|
| **Technical** | Platform or browser compatibility, scalability and performance, integration, uncertain feasibility |
| **Schedule** | Underestimated tasks, external dependencies and approvals, scope creep, people shared across projects |
| **Cost** | Underestimated budget, unexpected expenses, changing resource costs |
| **Quality** | Too little testing, tech debt, ambiguous requirements, poor docs |
| **Communication** | Misread requirements, poor collaboration, weak channels, culture and language barriers |
| **Security** | Code vulnerabilities, weak authentication and access control, compliance gaps, unprotected sensitive data |
| **Legal and compliance** | Licensing and IP, ownership disputes, missed standards, changing laws |

## The Iron Triangle
Three linked constraints: **Scope**, **Time** and **Cost**. Change one and the others move.
- **Scope** almost always *grows*, which costs more time and/or money.
- **Time**: speeding up needs more resources or less scope.
- **Cost**: cutting the budget means fewer features or a longer timeline.
- You usually can't optimize all three. Prioritize and make explicit trade-offs.

**As a developer:** don't just say "yes" to more features with no extra time, or a faster delivery with no cut in scope. Help managers and customers see the trade-off, e.g., "Sure, that adds X time," or "To ship earlier, which features can move to the next release?"

## Brooks' Law
> "Adding manpower to a late software project makes it later." (Fred Brooks, *The Mythical Man-Month*, 1975)

Why:
- **Communication overhead** grows much faster than headcount
- **Ramp-up time**: newcomers need to learn the domain, stack and codebase
- **Dependencies and coordination** get more complex and create bottlenecks

## Communication
- **Intra-team** (inside the team): most Scrum ceremonies (stand-up, planning, retro) exist to maximize this.
- **Inter-team** (with the wider org): where a project manager adds the most value, by getting resources and help and flagging risks to others.
- **Be open and honest.** Junior devs often hide problems out of embarrassment. Don't. Surfacing risks early is *your job*, because it lets the PM and team help.

## Estimation
- It's hard because software is almost always **new work** (unlike building similar houses).
- Agile **shortens the event horizon**: a 2-week sprint is far more predictable than 6 months.
- Engineers are **chronically optimistic** and quote the best-case date. Build in the risks.
- **To get better:** estimate each task, track the actual time, compare, repeat. Over time you get better at estimating *and* at spotting risks, which is exactly what the team needs to hear about.

## Team velocity
- How much work the *team* finishes per time period (e.g., story points per sprint).
- It's **not** for predicting the whole project or comparing individuals. It's for planning sprint capacity. Track it across sprints to make commitments reliable.

## Metrics and KPIs
Quantitative measures tied to company strategy. The PM keeps the team's work moving them.
- **Revenue:** total revenue, ARR, MRR, ARPU, customer lifetime value (CLTV)
- **Customer:** acquisition cost (CAC), retention rate, Net Promoter Score (NPS), churn rate
- **Product:** user engagement, feature adoption, bug-fix turnaround, release frequency
- **Operational:** dev velocity, time to market, **code quality metrics (coverage, complexity, tech debt)**, support response time
- **Financial:** gross margin, operating expenses, profit margin

(The course also points to *Preach My Gospel* ch. 8 on goals and plans.)

## Flow and interruptions
- Interruptions (email, notifications, drop-by visits) have a big recovery cost before you're back in flow.
- The tension: Agile wants information flowing freely (open offices, Slack or Teams) but people also need focus.
- **Managers:** don't micromanage.
- **Individuals:** find what works for you, such as asynchronous communication, **time blocking**, or noise-canceling headphones. Stay helpful *and* productive.

## Suggested follow-up questions
- Explain team velocity / KPIs / psychological safety / Brooks' Law
- How can a junior engineer contribute to project management?
- Example of ___ in a software project?
- How can team communication improve?
- Why is project management hard?
