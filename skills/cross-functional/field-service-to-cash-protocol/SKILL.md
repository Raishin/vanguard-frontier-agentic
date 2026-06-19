---
name: field-service-to-cash-protocol
description: Use this skill to orchestrate the field service to cash process (service to deliver) across Dynamics 365 Field Service and Dynamics 365 Finance, covering work order creation through service delivery, parts and inventory consumption, invoicing, and accounts receivable settlement. The skill defines stage gates, agent handoff rules, and escalation triggers so that service work is verified before invoicing, inventory is accurately consumed, and revenue is recognized correctly. It does not create or modify work orders, post invoices, approve service completions, or access field engineer credentials; all production-impacting steps are escalated to the relevant specialist or human owner.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-16"
  category: operational
  lifecycle: experimental
---

# Field Service to Cash Protocol

## Purpose
This skill orchestrates the field service to cash process (referred to as "service to deliver" in the Microsoft Dynamics 365 business process catalog) in Dynamics 365 environments. It coordinates Dynamics 365 Field Service (work order management, scheduling, service delivery, and inventory consumption) with Dynamics 365 Finance (invoicing, accounts receivable, and cash application). The protocol ensures that service is verified as delivered before invoicing, that parts consumed are accurately recorded against inventory, and that revenue is recognized in the correct period. It does not give field service configuration or financial accounting advice; all recommendations are subject to human owner confirmation.

## When to use
- A field service work order must progress from creation through scheduling, delivery, parts consumption, invoicing, and payment.
- Inventory consumed during field service must be reconciled with accounts in Dynamics 365 Finance.
- A service agreement with recurring invoicing requires coordination between Field Service and Finance.
- A work order completion must be verified before an invoice is generated or posted.
- A cross-module discrepancy between field service actuals and financial postings needs investigation.

## When NOT to use
- The matter is confined entirely to work order scheduling with no financial impact — route to the Field Service specialist.
- The matter involves project-based billing from Dynamics 365 Project Operations — use the appropriate project-to-profit process.
- The matter is a warranty claim or service contract renewal without an active work order — escalate to the Field Service and finance owners.
- Direct Field Service or Finance configuration is required — route to the relevant specialist agent.

## Participating agents
- d365-field-service-to-cash-agent — owns work order creation, scheduling, service delivery confirmation, and inventory consumption in Dynamics 365 Field Service
- d365-finance-close-to-report-agent — owns invoicing, accounts receivable, revenue recognition, and cash application in Dynamics 365 Finance

## Inputs required
- Work order number and customer account
- Service type and scope of work
- Assigned field technician and scheduled date
- Parts and inventory items required for the work order
- Applicable service agreement or contract reference
- Billing method (time and materials, fixed price, or agreement invoice)

## Evidence required
- Work order completion confirmation from the field technician in Dynamics 365 Field Service
- Parts consumed and labor time recorded on the work order
- Customer sign-off or work order closure approval (if required by contract)
- Inventory adjustment confirmation for parts consumed
- Invoice draft validated against work order actuals before posting

## Workflow
1. Receive work order creation trigger; validate that customer account, service scope, and billing method are defined.
2. Confirm that required parts are available in the assigned inventory location (warehouse or technician's truck stock).
3. Schedule and dispatch field technician via Dynamics 365 Field Service scheduling board; record assignment.
4. Monitor work order progress; flag overdue work orders to the field service owner.
5. Upon service completion, verify that the technician has recorded all labor time and parts consumed on the work order.
6. Work order verification gate: confirm the work order is marked as "Complete" in Dynamics 365 Field Service; if customer sign-off is required, confirm it is recorded.
7. Trigger inventory adjustment for consumed parts; confirm inventory levels are updated in the system.
8. Hand work order actuals (labor, parts, travel) to d365-finance-close-to-report-agent for invoice creation.
9. Create invoice draft in Dynamics 365 Finance; validate against work order actuals and applicable service contract pricing.
10. Invoice accuracy gate: surface discrepancies between invoice draft and work order actuals for human resolution before posting.
11. Post the invoice; open an accounts receivable entry with the correct payment terms.
12. Monitor payment due date; surface collection activities to the finance owner as needed.
13. Apply cash receipt to the open invoice; close accounts receivable entry.

## Decision gates
| Gate | Condition | Action if not met |
|---|---|---|
| Parts availability | Required parts in stock at assigned location | Notify d365-field-service-to-cash-agent; arrange parts transfer or order |
| Work order completion | Work order marked complete in Dynamics 365 Field Service | Do not proceed to invoicing; return to field service agent |
| Customer sign-off | Sign-off recorded if required by contract | Hold invoice; await confirmation |
| Inventory adjustment | Consumed parts reflected in inventory system | Hold invoice; resolve inventory discrepancy first |
| Invoice accuracy | Invoice matches work order actuals and contract pricing | Hold posting; flag for human resolution |
| Cash application | Payment matched to invoice | Do not auto-close AR entry on partial payment without finance owner approval |

## Refusal triggers
- Invoice is requested for a work order that is not yet marked as "Complete" — refuse.
- Inventory adjustment is requested without a corresponding work order consumption record — refuse.
- Customer PII, field technician credentials, or service agreement financial terms are included in any input — stop; redact and escalate.
- A live invoice or AR entry modification is requested without human authorization — refuse.
- A backdated work order completion or invoice date is requested — refuse; escalate to the field service and finance owners.

## Handoff rules
- d365-field-service-to-cash-agent provides a work order actuals summary (labor, parts, travel) to d365-finance-close-to-report-agent before any invoice is created.
- Every handoff carries a capsule: matter_id, work_order_id, current_stage, gate_outcomes, open_questions, do_not_do_list.
- No invoice is posted without a verified work order completion record.
- All escalations are logged with trigger, timestamp, and receiving owner.

## KPIs
- Work order to invoice cycle time
- First-time fix rate (work orders completed without return visits)
- Invoice accuracy rate (first-pass match to work order actuals)
- Days sales outstanding for field service invoices
- Inventory accuracy rate for field service consumption

## References
- [Service to deliver end-to-end overview — Dynamics 365](https://learn.microsoft.com/dynamics365/guidance/business-processes/service-to-cash-areas-overview)
- [Overview of Dynamics 365 Field Service](https://learn.microsoft.com/dynamics365/field-service/overview)
- [Set up agreements to automatically generate invoices — Dynamics 365 Field Service](https://learn.microsoft.com/dynamics365/field-service/set-up-agreements-invoices)
- [Streamline service delivery and invoicing — service to deliver end-to-end](https://learn.microsoft.com/dynamics365/guidance/business-processes/service-to-cash-introduction)
