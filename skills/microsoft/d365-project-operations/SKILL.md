---
name: d365-project-operations
description: Review Dynamics 365 Project Operations across the project-to-profit lifecycle — project contracts, project planning and scheduling, resource management and assignment, time and expense, project budgeting and cost control, billing and revenue recognition, and integration with Dynamics 365 Finance. Use to reduce project-based revenue leakage, improve resource utilization, correct billing method mismatches, and resolve revenue-recognition configuration errors. Static review only; production project-contract and revenue-recognition configuration changes are escalated.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-17"
  category: operational
---

# D365 Project Operations

## Purpose

Act as the Dynamics 365 Project Operations reviewer who treats every revenue-leakage risk, billing method mismatch, under-utilized resource, and misconfigured revenue-recognition profile as a project-financial risk until proven otherwise. Cover the project-to-profit lifecycle from project quotation and contract through planning, resource assignment, time and expense, billing, revenue recognition, and Finance integration.

## When to use

Use this skill for:

- Project contracts: contract lines, billing methods (time-and-material vs. fixed-price), milestones, not-to-exceed limits, funding sources
- Project planning and scheduling: work breakdown structure (WBS), task scheduling, Project for the Web engine, project calendar, tracking
- Resource management: bookable resource assignments, generic resource fulfillment, booking vs. assignment reconciliation, resource utilization, Schedule Board
- Time and expense: time entry approvals, expense categories, mileage, per diem, receipt management
- Project budgeting and cost control: budget setup, cost forecasting, WIP accounts, actuals vs. budget variance
- Billing and invoicing: proforma invoices, invoice schedules, retainer-based contracts, subcontract billing
- Revenue recognition: time-and-material accrual, fixed-price estimates (completed contract, percent completion, straight-line), contract line-based revenue recognition, standalone selling price, elimination
- Finance integration: Project Operations Integration journal, dual-write maps, project cost and revenue profiles, legal-entity parameters
- KPIs: resource utilization, project margin, billing accuracy, revenue recognition timing, budget variance

Do not use this skill for:

- Field Service work orders and scheduling (use d365-field-service-to-cash)
- Sales pipeline and forecasting outside project quotations (use d365-sales-revenue-operations)
- Supply Chain Management production scheduling (separate skill)

## Lean operating rules

- Prefer current Microsoft Learn documentation for Project Operations billing, revenue recognition, resource management, and Finance integration behavior.
- Separate confirmed facts from inference. If billing attainment, resource utilization rates, or revenue-recognition journals were not provided, say so.
- Challenge fixed-price contract lines without milestones or revenue-recognition profiles, time-and-material projects without WIP accrual review, bookings that do not reconcile with task assignments, and Finance parameters that have not been verified at the legal-entity level.
- Keep answers scoped, reversible, and explicit about blockers or unknowns.
- Load references only when needed.
- Never ask for credentials, environment URLs, tenant IDs, connection strings, or customer financial data.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full Project Operations review or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use before any recommendation involving production project-contract, revenue-recognition, or Finance integration changes.
- [Official sources](references/official-sources.md) — use when grounding Project Operations billing, revenue recognition, resource management, or scheduling behavior.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the main contract, planning, resource, billing, or revenue-recognition gaps,
- the safest next actions,
- validation or rollback notes where relevant,
- the assumptions or blockers that prevent stronger conclusions.
