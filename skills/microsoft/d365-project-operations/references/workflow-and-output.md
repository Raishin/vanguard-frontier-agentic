# Workflow and output contract

Use this reference only when performing the full Dynamics 365 Project Operations review or formatting the final answer.

## Review domains

Check these areas before giving a verdict:

- Project contracts: contract lines, billing methods (time-and-material vs. fixed-price), milestones, not-to-exceed limits, funding sources, contract-line revenue-recognition profiles
- Project planning and scheduling: WBS task structure, Project for the Web scheduling engine, project calendar, task dependencies, tracking vs. planned schedule
- Resource management: named vs. generic resource bookings, booking vs. assignment reconciliation (Resource Reconciliation tab), utilization rates, Schedule Board, resource request fulfillment
- Time and expense: time entry submission and approval workflow, expense categories and policies, receipts, per diem, mileage
- Project budgeting and cost control: budget setup, cost forecasting accuracy, WIP accounts, actuals vs. budget variance, cost-and-revenue profiles
- Billing and invoicing: proforma invoice generation, invoice schedule adherence, retainer and advance drawdowns, subcontract billing, not-to-exceed status
- Revenue recognition: billing-method alignment (T&M vs. fixed-price), fixed-price estimate method (completed contract, percent completion, straight-line, standalone selling price), contract line-based recognition, WIP accrual and reversal, elimination
- Finance integration: Project Operations Integration journal, dual-write map versions, project cost and revenue profile rules, legal-entity parameter settings
- KPIs: resource utilization, project margin, billing accuracy, revenue recognition timing, budget variance

## Safe workflow

1. **Frame scope**
   - Area in scope (contracts / planning / resource / billing / revenue recognition / Finance integration):
   - Deployment type (Project Operations Integrated with ERP / Lite / Core):
   - Required outcome (revenue leakage reduction / resource utilization / billing accuracy / Finance reconciliation):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer exported reports: project transaction journals, billing attainment, resource utilization, budget variance, revenue-recognition estimate runs.
   - Otherwise inspect sanitized user-provided summaries or official Microsoft Learn documentation.
   - Label each finding as `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test risk**
   - Which fixed-price contract lines lack milestones or revenue-recognition profiles?
   - Are time-and-material projects accruing WIP revenue, and is the accrual reversing correctly at invoicing?
   - Do resource bookings reconcile with task assignments, or is the Schedule Board showing over-allocation?
   - Are Finance integration journals posting without errors, and are dual-write map versions current?
   - What evidence is missing that would change the verdict?

4. **Recommend the smallest safe action**
   - Prefer configuration and process fixes over contract or Finance-parameter changes.
   - Production project-contract, revenue-recognition, and Finance integration changes require live-guard escalation with a rollback plan.

## Output contract

Return this structure:

```markdown
# D365 Project Operations Review: <scope>
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
