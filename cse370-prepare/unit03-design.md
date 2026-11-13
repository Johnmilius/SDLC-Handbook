# Unit 03 — Software Design

Source: https://byui-cse.github.io/cse370-course/unit03/prepare.html
*(Study notes, paraphrased from the course page.)*

## Key points
- Design time surfaces and reduces risk **early** in the SDLC.
- Design can be documented formally or informally. Match the approach to the team and its goals.
- Design is the **bridge** between business people (requirements) and developers (implementation). At its core it's a **communication tool**, and it has to balance being thorough with being concise.

## Connection to requirements
- Design turns requirements into a blueprint for how the system will meet them.
- Designing often exposes **ambiguities or inconsistencies** in the requirements. That sends you back to stakeholders and improves the requirements.
- **Traceability:** every design element should trace to a requirement, and no requirement should be left out. The chain runs requirement → design → code → tests.

## Design as communication

**With business stakeholders**
- *Visual clarity*: wireframes, mockups and UML that non-technical people can follow
- *Requirement verification*: confirms requirements were understood correctly
- *Early feedback*: catches mismatches before coding starts
- *Clarifies scope*: limits scope creep
- *Risk communication*: sets realistic expectations
- *Shared reference*: helps settle disputes
- *Supports decisions*: about priorities, resources and trade-offs

**Among developers**
- A shared understanding of the architecture, plus clear specs for components and interfaces
- Common terminology and consistent code patterns
- Diagrams that make complex interactions easier to see
- A baseline for **code review** (does the code match the design?)
- **Change management**: assess the impact of a change against the whole system
- **Onboarding** for new team members
- **Cross-team coordination** of interfaces and dependencies

**For future developers**
- Explains *why* earlier decisions were made, and the interfaces between components.
- Preserves **"tribal knowledge"**, the undocumented know-how that's otherwise lost when people leave.

## How design reduces risk
| Risk | How design helps |
|---|---|
| Misread requirements | Detailed specs create shared understanding |
| Complexity | Splits the system into manageable modules |
| Late surprises | Technical problems are found before coding |
| Inconsistency | Standards and patterns cut bugs |
| Scalability and performance | Load is planned for |
| Security | Threats are considered early |
| Change impact | A clear blueprint shows ripple effects |
| Maintainability | Less technical debt, easier to extend |
| Knowledge loss | Documentation survives team turnover |
| Compliance | Legal and industry standards are built in |

## Design patterns
A **reusable, general solution template** for a common design problem. It's not finished code.
- **Reusability**: proven solutions, so you don't reinvent the wheel
- **Abstraction**: independent of language or platform
- **Common language**: names like Singleton, Observer and Factory make discussion quick and precise

## Levels of design (high to low)
| Level | Defines | Typical outputs |
|---|---|---|
| **Architectural** | Overall structure: major modules, layers (presentation / business / data), architecture style (client-server, microservices), deployment | Architecture and component diagrams, tech-stack decisions |
| **Component** | Inside each component: classes, methods, relationships, algorithms, internal interfaces | Component, class and sequence diagrams |
| **Module** | Breaks components into cohesive, loosely coupled units | Module interface specs, dependency diagrams |
| **Class** (OOP) | Attributes, methods, inheritance, associations | UML class diagrams, class specs |
| **Function / method** | Algorithm, parameters, return values, error handling | Pseudocode, flowcharts, API docs |

**Example: order processing system**
- Architecture: Order, Inventory and Customer microservices
- Component: Order Management uses `Order`, `OrderService` and `OrderRepository`
- Module: `OrderService` covers `createOrder`, `cancelOrder` and `updateOrderStatus`
- Class: `Order` has `orderId`, `customerId`, `orderItems`, `calculateTotalPrice()` and `addItem()`
- Method: `calculateTotalPrice()` loops over the items and applies discounts

### Interfaces are contracts
At every level, the **interface between two components** is a contract both sides can rely on.

## Design tools

### UML class diagram (static structure)
- A class is a box with 3 compartments: **name / attributes / operations**
- **Relationships:**
  - *Association*: a structural link with multiplicity (1-1, 1-many)
  - *Generalization*: inheritance
  - *Dependency*: one class uses another (parameters, return types)
  - *Aggregation*: whole and part, where the parts can live on their own (car → wheels)
  - *Composition*: a stronger whole-part link, where the part's lifetime is owned by the whole
- **Visibility:** `+` public, `-` private, `#` protected

### UML sequence diagram (behavior over time)
- **Actors and objects** across the top, with a **lifeline** (dashed vertical line) under each
- **Messages:** synchronous (sender waits), asynchronous (sender doesn't wait), return
- **Activation boxes:** show when an object is busy processing
- Read top to bottom for the order in which calls happen

### Data dictionary
A central reference that defines each data element: **name, description, data type, length, format, valid values, constraints** (required, unique and so on).
- Very useful for **APIs and interfaces**.
- Best written *before* building, but it can also be generated from code afterward.

## How much detail?
- Formal or detailed design is more **precise and less ambiguous**, but it's **harder to keep current** and drifts out of sync.
- "Formal tool" isn't the same as "detailed": a whiteboard sketch can be thorough, and a formal UML diagram can be shallow.
- Choose a process your team will **actually use and maintain**. A stale design can be **worse than none**, because it spreads false information.

## Design in an AI world
AI code generation works best when the problem is **well defined**. As AI takes over more implementation, engineers may spend most of their time on **design, review and testing**, so good design matters even more.

## Suggested follow-up questions
- Explain UML / sequence diagrams / design patterns / architectural design
- Example of a sequence diagram?
- What are the benefits of design patterns?
- Why do developers skip design?
- How much design does a project need?
