# Unit 02 — Requirements Elicitation

Source: https://byui-cse.github.io/cse370-course/unit02/prepare.html
*(Study notes, paraphrased from the course page.)*

## Key points
- If you don't identify and document requirements clearly, problems later are almost guaranteed.
- **Every later SDLC phase traces back to requirements**, so take the time to do them well.
- There are many ways to gather and document requirements. Pick the one that fits the team and the goal.

## Why it's hard
- Asking customers "what do you want?" quietly asks them to design the solution. Usually they just know they have a *problem*.
- Your job is to help them define their needs. Because needs shift as understanding grows, Agile's adaptability helps here.

## Traceability
- Design elements, code and tests should each **trace back to a documented requirement**.
- Requirements matter most during verification and testing, because that's where you prove the product meets them.
- After rollout, monitoring how well users' needs are met feeds the next round of requirements.

## Working with the customer
1. **Initiate communication / identify stakeholders.** Find the people directly involved or affected. Also map outside forces: other projects, budgets, timelines and company politics (people with their own agendas). These are all risks.
2. **Explore similar projects.** "I want something like X." Walk through it together and note what's the same and what's different.
3. **Prioritize requirements.** Ask what goes wrong if a requirement isn't met. Separate what's needed *now* from *later*. Teach stakeholders when something that sounds simple is hard technically.
4. **Document assumptions.** Not every detail is known up front. Write the assumptions down so they're visible and can be checked later.
5. **Iterate.** When you find gaps, go back to stakeholders and refine.

## Techniques for gathering requirements
| Technique | What it is |
|---|---|
| Document analysis | Review existing reports and manuals to learn the current state and spot gaps |
| Interviewing | Talk directly with stakeholders to surface needs and concerns |
| Observation | Watch users and processes in action to find real pain points |
| Prototyping | Build simple versions so stakeholders can react and give feedback |
| Validating | Run reviews and feedback sessions to confirm requirements are accurate and match the goals |

## Documenting requirements
How formal you get depends on **the problem and the cost of failure**, not on personal style.
- Defense or safety-critical work: very rigorous and detailed.
- A fast-moving consumer app: lighter, because heavy detail goes stale and doesn't pay off.

### 1. Bulleted lists
Simplest form, but usually **too thin** to drive design and development.
> e.g., "Users must be able to log in with their credentials."

### 2. User stories
- Format: **"As a [user], I want [action] so that [benefit]."**
- They focus on the user's role, goal and value, which makes them easier to prioritize and trace.
- They come with **acceptance criteria** in **Given / When / Then** form:
  > *Given* I'm on the login page, *when* I enter valid credentials and click Login, *then* I'm sent to my dashboard.
- Example acceptance criteria for "User Login": valid login → dashboard • bad credentials → error and retry • "Forgot password" → recovery page • session stays active until logout or timeout • logout → back to the login page.

### 3. Use cases
A structured, process-focused narrative of how **actors** interact with the system to reach a goal.

| Component | Meaning |
|---|---|
| Title | Short name |
| Actors | Primary (direct users) and secondary (supporting systems) |
| Preconditions | What must be true before it starts |
| Main flow | The "happy path" steps |
| Alternate flows | Different paths to the same goal |
| Exception flows | Error or unexpected paths |
| Postconditions | System state afterward |
| Triggers | The event that starts it |

Use cases are more process-oriented. User stories are short statements from the user's point of view.

### 4. Other formats
Companies have their own standard templates. You'll learn yours on the job.

## Types of requirements

### Functional (*what* the system does)
- **System behaviors**: operations, step sequences, input → processing → output
- **Data management**: what data is accepted or returned, its formats and storage (API endpoints especially)
- **External interactions**: interfaces to internal or third-party systems
- **User interactions**: the UI (screens, buttons, menus) plus user roles and permissions

### Non-functional (*how* it does it)
- **Performance**: response time, throughput, scalability
- **Environment**: operating environment, software dependencies, external interfaces
- **Hardware**: CPU, memory, storage and so on

## Judging requirement quality (all four must hold)
| Criterion | Question to ask |
|---|---|
| **Completeness** | Does it cover everything that needs to happen, with no gaps? |
| **Ambiguity / precision** | Could someone implementing or testing it read it differently? |
| **Feasibility** | Can it actually be built within the limits of time, budget, technology and people? |
| **Timeline / scope** | Is there a clear, reasonable time frame or scope? |

**Ambiguity red flags:** vague words ("user-friendly", "fast", "efficient") • missing specifics ("many users" instead of a number) • more than one possible reading.
**Fixes:** clear, specific language • examples and scenarios • reviews with stakeholders.
**Feasibility checks:** feasibility studies or proofs of concept • involve technical experts • compare budget and resources to scope.

> If a project has trouble, you can usually trace it back to a requirement that failed one or more of these criteria.

## Suggested follow-up questions (to ask an AI)
- What problems can come from software requirements?
- What are good ways to gather requirements?
- How can requirements be documented?
- Functional vs non-functional requirements?
- How do you evaluate requirement quality?
