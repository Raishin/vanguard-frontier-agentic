# Safety checklist

Use this reference before any recommendation involving production project-contract configuration, revenue-recognition setup, Finance integration parameters, or billing changes in Dynamics 365 Project Operations.

## Non-negotiables

- Never ask users to paste credentials, tenant IDs, environment URLs, connection strings, certificates, or customer financial data into chat.
- Use exported project transaction reports or sanitized user-provided evidence for current-state claims; otherwise use documentation and label the evidence level.
- Do not invent billing attainment percentages, resource utilization rates, revenue-recognition amounts, or Finance journal balances.
- Require explicit human approval before recommending any production project-contract, revenue-recognition profile, Finance parameter, or billing-method change.
- Use current official Microsoft Learn documentation for Project Operations billing, revenue recognition, resource management, and Finance integration behavior.
- Keep recommendations least-change, reversible, and scoped to the domain in question.

## Stress checks

- Which fixed-price contract lines lack milestones, revenue-recognition profiles, or correct cost-and-revenue profile assignments?
- Are time-and-material projects accruing WIP revenue correctly, and does the accrual reverse at invoicing without leaving residual balances?
- Do resource bookings reconcile with task assignments on the Resource Reconciliation tab, or are over-allocated resources masked?
- Are Finance integration (dual-write) maps at the required minimum versions, and are Integration journals posting without error?
- What rollback exists if a revenue-recognition setup or Finance parameter change creates incorrect posting in a live legal entity?

## Evidence labels

Use `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's actual billing accuracy, resource utilization, revenue-recognition journal correctness, or Finance integration health.

## Live-guard gate

The following actions require explicit human confirmation and are out of scope for automated execution:

- Modifying production project-contract billing methods, milestone schedules, or not-to-exceed limits
- Changing production revenue-recognition profiles, cost-and-revenue profile rules, or project management and accounting parameters
- Enabling or disabling contract line-based revenue recognition or standalone selling price features in a live legal entity
- Adjusting or eliminating fixed-price revenue estimate projects in production
- Modifying Finance integration (dual-write) map versions or Project Operations Integration journal configuration in a production environment
- Executing bulk time-entry or expense adjustments, or performing mass billable-status updates
