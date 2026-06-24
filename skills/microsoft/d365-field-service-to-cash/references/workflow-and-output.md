# Workflow and output contract

Use this reference only when performing the full Dynamics 365 Field Service to cash (service-to-deliver) review or formatting the final answer.

## Review domains

Check these areas before giving a verdict:

- Work order design: incident types and task templates, work order types, characteristics, territories, priorities
- Scheduling: schedule board usage, schedule assistant, Universal Resource Scheduling, Resource Scheduling Optimization (and its separate license)
- Bookable resources: skills/characteristics, working hours, geographic territories, resource types (employee, contractor, equipment)
- First-time-fix: parts on truck, skill matching, incident-type task completeness, preventive maintenance coverage
- Mobile execution: booking status lifecycle (Scheduled → Traveling → In Progress → Completed), timestamps, booking journals, offline readiness
- Inventory and truck stock: consumption tracking, replenishment, purchase orders, returns
- Billing: work-order-to-invoice flow, products/services, actuals, finance and operations integration (worker alignment)
- KPIs: resource utilization, first-time-fix rate, mean time to resolution, schedule adherence, service revenue capture

## Safe workflow

1. **Frame scope**
   - Lifecycle stage in scope (work order design / scheduling / mobile execution / inventory / billing):
   - Field Service edition and add-ins (RSO, Field Service mobile, finance & operations integration):
   - Required outcome (scheduling efficiency / first-time-fix / revenue capture / KPI review):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer exported reports: work order aging, booking utilization, first-time-fix, inventory variance, uninvoiced completed work.
   - Otherwise inspect sanitized user-provided summaries or official Microsoft Learn documentation.
   - Label each finding as `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test risk**
   - What work orders are unscheduled or repeatedly rescheduled, and why?
   - Where does first-time-fix break down (parts, skills, task templates)?
   - What completed bookings never generated an invoice (revenue leakage)?
   - Is truck-stock consumption tracked and replenished, or is inventory drifting?
   - What evidence is missing that would change the verdict?

4. **Recommend the smallest safe action**
   - Prefer configuration and process fixes over scheduling-engine changes.
   - Production scheduling-engine, RSO parameter, and billing-configuration changes require live-guard escalation with a rollback plan.

## Output contract

Return this structure:

```markdown
# D365 Field Service to Cash Review: <scope>
## Executive verdict
- Status: HEALTHY / HEALTHY WITH RISKS / AT RISK / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Reports or checks to review:
- Expected result:
## Residual risk
- <risk or explicit none>
```
