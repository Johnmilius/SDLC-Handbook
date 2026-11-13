# Template: architecture decision record (ADR)

Use at stage 3 of the [workflow](../handbook/workflow.md#wf-3) for Tier 3 changes, and for any decision that is hard to reverse. Write one for a decision that affects structure, non-functional characteristics, dependencies, interfaces or construction techniques. One decision per record, one or two pages. [Established] Nygard, "Documenting Architecture Decisions" (2011) ([S8](../handbook/sources.md#s8)).

Store ADRs in the project repository (`docs/adr/NNNN-title.md`), not in this handbook.

```markdown
# NNNN. <short noun phrase>

Status: <proposed | accepted | superseded by NNNN>   Date: <YYYY-MM-DD>

## Context
<the forces at play, stated as facts: constraints, what exists today, what is driving the decision>

## Decision
We will <the decision, in full sentences>.

## Consequences
<everything that follows, good and bad: what gets easier, what gets harder, what must be watched, how to undo it>
```

## Filled-in example (synthetic)

```markdown
# 0007. Migrate user IDs to UUIDs with an expand/contract sequence

Status: accepted   Date: 2026-11-13

## Context
User IDs are auto-increment integers. They appear in URLs and in exports, and they reveal how many users exist.
The users table has foreign keys from four other tables. The service cannot be taken offline for a migration.
Changing the column type in place cannot be rolled back once new rows are written.

## Decision
We will add a UUID column next to the integer ID, write both for new and updated rows, backfill old rows,
switch reads to the UUID, and drop the integer column in a later release. Each step ships on its own.

## Consequences
Every step before the drop can be rolled back by redeploying the previous version.
Dropping the integer column is the point of no return, so it waits one full release after reads have switched.
Both columns must be kept in step during the dual-write period, which adds code we will delete later.
Old links that contain integer IDs stop working at the drop unless we keep a lookup table. That is an open question.
The backfill must be rehearsed on a copy of production data, with row counts and foreign keys checked before and after.
```
