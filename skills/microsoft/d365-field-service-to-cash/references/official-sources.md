# Official sources

Use this reference only when you need source grounding for Dynamics 365 Field Service behavior or the service-to-deliver business process.

## Microsoft Learn documentation

Use these as starting points, not as proof of the user's live Field Service configuration or operational metrics:

- https://learn.microsoft.com/dynamics365/field-service/overview — Field Service capabilities: work orders, scheduling/dispatch, mobile app, asset management, preventive maintenance, inventory/purchasing/returns, billing, time tracking, analytics. Supports the scope and KPI workflow steps.
- https://learn.microsoft.com/dynamics365/field-service/universal-resource-scheduling-for-field-service — Universal Resource Scheduling: requirements generated from work orders, bookable resource bookings, scheduling personas (dispatcher, scheduling analyst), manual/semi-automated/automated scheduling. Supports the scheduling review step.
- https://learn.microsoft.com/dynamics365/field-service/field-service-architecture — Work order architecture: requirement→booking lifecycle, booking timestamps and journals, inventory consumption, and invoice creation on work order close. Supports the service-to-deliver flow and billing steps.
- https://learn.microsoft.com/dynamics365/field-service/rso-overview — Resource Scheduling Optimization add-in (separate license) for automated scheduling and utilization optimization. Supports the automated-scheduling and licensing pushback steps.
- https://learn.microsoft.com/dynamics365/guidance/business-processes/service-to-cash-create-process-service-work — "Manage service work" process area within the Service to deliver (formerly service to cash) end-to-end scenario: create → schedule → perform → review/close → invoice → record payment. Supports the end-to-end lifecycle workflow.

## Terminology note

Microsoft renamed the "service to cash" end-to-end scenario to "service to deliver" beginning with the February 2025 Business Process Catalog. Some Learn articles are not yet fully updated. Verify current terminology before presenting process names.

## Grounding rule

Official documentation explains Field Service behavior. It does not prove the user's actual work order backlog, scheduling utilization, first-time-fix rate, inventory accuracy, or invoicing completeness. Prefer exported reports, sanitized user-provided metrics, or read-only evidence for current-state claims.
