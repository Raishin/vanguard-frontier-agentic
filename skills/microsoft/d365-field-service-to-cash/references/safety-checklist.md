# Safety checklist

Use this reference before any recommendation involving production scheduling-engine changes, Resource Scheduling Optimization parameters, or billing/invoicing configuration in Dynamics 365 Field Service.

## Non-negotiables

- Never ask users to paste credentials, tenant IDs, environment URLs, connection strings, certificates, or customer data into chat.
- Use exported reports or sanitized user-provided evidence for current-state claims; otherwise use documentation and label the evidence level.
- Do not invent utilization rates, first-time-fix percentages, work order counts, or revenue figures.
- Require explicit human approval before recommending any production scheduling-engine, RSO optimization-schedule, or billing-configuration change.
- Use current official Microsoft Learn documentation for Field Service and Universal Resource Scheduling behavior.
- Keep recommendations least-change, reversible, and scoped to the domain in question.

## Stress checks

- What unscheduled or repeatedly rescheduled work orders indicate a scheduling or resourcing gap?
- What drives low first-time-fix (missing parts on truck, skill mismatch, incomplete task templates)?
- What completed bookings have no corresponding invoice (service revenue leakage)?
- Is truck-stock consumption tracked and replenished, or is inventory drifting?
- Is RSO licensed and configured before recommending automated optimization?
- What rollback exists if a scheduling-engine or billing change misbehaves in production?

## Evidence labels

Use `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's actual scheduling utilization, first-time-fix rate, inventory accuracy, or invoicing completeness.

## Live-guard gate

The following actions require explicit human confirmation and are out of scope for automated execution:

- Changing production Resource Scheduling Optimization parameters or optimization schedules
- Modifying production schedule board configuration or scheduling rules at scale
- Changing work-order-to-invoice billing configuration or finance and operations integration mappings
- Executing bulk work order, booking, or inventory updates
- Altering bookable resource working hours, territories, or skills in bulk in production
