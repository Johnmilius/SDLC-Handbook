# Template: acceptance criteria

Use at stage 2 of the [workflow](../handbook/workflow.md#wf-2), for every tier. Rule: [QB-1](../handbook/quality-bar.md#qb-1). The test plan is built from these ([TP-1](../handbook/plan-tests.md#tp-1)).

```markdown
## Change: <one line>
Problem: <who is affected, and what "fixed" looks like>
Not included: <what this change will not do>
Tier: <1 | 2 | 3>  Triggers: <list, or "none">   (the developer decides)

### Acceptance criteria
AC-1  Given <starting state>, when <action>, then <observable result>.
AC-2  Given <...>, when <invalid or failing action>, then <observable result>.   (negative case)

### Open questions
- <anything the criteria do not settle. Answer these before writing the test plan>
```

Each criterion has one observable result. Include at least one negative case. If two people could read a criterion differently, rewrite it.

## Filled-in example (synthetic)

```markdown
## Change: add a max_retries setting to the job service
Problem: operators cannot tune retries without a redeploy. Fixed means they can set it through the API.
Not included: per-job overrides; changing retries for jobs already running.
Tier: 2  Triggers: public API contract

### Acceptance criteria
AC-1  Given no value was ever set, when the worker starts a job, then it retries at most 3 times.
AC-2  Given the service is running, when PUT /config/max_retries is sent {"value": 10}, then the response is 200 and the value is 10.
AC-3  Given the service is running, when the value is 11, -1, 3.5, "3" or missing, then the response is 422 with body {"error": "max_retries_out_of_range"} and the value is unchanged.
AC-4  Given the value was set to 5, when the worker starts a new job, then it retries at most 5 times.

### Open questions
- Should a change apply to jobs that are already running? (Assumed no. See "Not included".)
```
