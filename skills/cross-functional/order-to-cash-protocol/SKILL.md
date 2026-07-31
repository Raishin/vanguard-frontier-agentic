---
name: order-to-cash-protocol
description: Use this skill to coordinate the order-to-cash process across Dynamics 365 Supply Chain Management, Finance, and Sales, covering confirmed order through fulfillment, invoicing, accounts receivable, and cash collection. It defines stage ownership, gate conditions, agent handoff rules, and escalation triggers for the post-order revenue cycle. The skill does not execute system transactions, approve credit changes, post invoices, or make collection decisions; all production-impacting steps are escalated to the relevant specialist agent or human owner.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-16"
  category: operational
  lifecycle: experimental
---

# Order-to-Cash Protocol

## Purpose
This skill coordinates the order-to-cash (O2C) end-to-end process in Microsoft Dynamics 365 environments. Starting from a confirmed sales order, it orchestrates order management, inventory fulfillment, shipment, invoicing, accounts receivable, and cash collection. The protocol aligns d365-supply-chain-plan-to-produce-agent (fulfillment), d365-finance-close-to-report-agent (invoicing and AR), and d365-sales-revenue-operations-agent (sales performance and customer communication). It does not give business or financial advice; all recommendations are subject to human owner confirmation.

## When to use
- A confirmed sales order must progress through fulfillment, invoicing, and payment collection.
- Multiple Dynamics 365 modules are involved and cross-agent coordination is required.
- An accounts receivable aging item needs coordinated follow-up between Sales and Finance.
- A return, credit memo, or dispute requires orchestrated handling across Supply Chain and Finance.
- A post-order cash flow analysis or DSO escalation requires cross-domain visibility.

## When NOT to use
- The opportunity has not yet been converted to a confirmed order — use the lead-to-cash-protocol or prospect-to-quote process instead.
- The matter is confined entirely to accounts payable or procurement — use procure-to-pay-protocol.
- Direct live-order configuration or system transaction execution is required — escalate to the relevant specialist.
- The matter involves revenue recognition complexity at contract inception — use lead-to-cash-protocol first.

## Participating agents
- d365-supply-chain-plan-to-produce-agent — owns order fulfillment, picking, packing, and shipment
- d365-finance-close-to-report-agent — owns invoicing, accounts receivable, collections, and cash application
- d365-sales-revenue-operations-agent — owns sales order policies, customer communication, and sales performance analysis

## Inputs required
- Confirmed sales order number and legal entity
- Customer account and credit status
- Requested and promised delivery date
- Pricing and discount terms from the sales order
- Accounts receivable payment terms for the customer

## Evidence required
- Sales order confirmation record from Dynamics 365 Supply Chain Management
- Available-to-promise (ATP) or inventory reservation confirmation
- Packing slip or delivery note reference
- Invoice draft for accuracy review before posting
- Payment receipt or remittance advice for cash application

## Workflow
1. Receive confirmed sales order from d365-sales-revenue-operations-agent or directly from the order management system.
2. Verify order accuracy: line items, pricing, quantities, and delivery terms match the original customer agreement.
3. Check customer credit status; if credit is on hold, escalate to d365-finance-close-to-report-agent and finance owner.
4. Trigger inventory reservation and ATP confirmation via d365-supply-chain-plan-to-produce-agent.
5. Monitor warehouse picking, packing, and shipment workflows; flag delays against committed delivery dates.
6. Generate packing slip upon confirmed shipment; hand off to d365-finance-close-to-report-agent.
7. Create and validate sales invoice against packing slip and sales order; surface discrepancies for human resolution.
8. Post invoice and open accounts receivable entry in Dynamics 365 Finance.
9. Monitor payment due dates; trigger collection activities per the customer's payment terms.
10. Apply received cash to open invoices and close accounts receivable entries; update sales performance reporting.

## Decision gates
| Gate | Condition | Action if not met |
|---|---|---|
| Credit status | Customer account not on credit hold | Escalate to d365-finance-close-to-report-agent and finance credit manager |
| Order accuracy | Lines, prices, and terms match customer agreement | Return to d365-sales-revenue-operations-agent for correction |
| ATP confirmation | Stock reserved or ATP date within delivery window | Notify supply chain agent; surface gap to human owner |
| Invoice accuracy | Invoice matches packing slip and sales order | Hold posting; flag for human review |
| Collections escalation | Payment overdue beyond configurable threshold | Escalate to finance collections owner |

## Refusal triggers
- Live transaction execution is requested without human confirmation — refuse and escalate.
- Credit hold override is requested without documented approval — refuse.
- Customer PII, payment details, or bank account data are present in any input — stop; redact and escalate.
- An invoice is requested to be posted with unresolved line discrepancies — refuse.
- A write-off or bad-debt decision is requested — escalate to finance owner; do not decide unilaterally.

## Handoff rules
- Each handoff carries a structured capsule: matter_id, sales_order_id, current_stage, gate_outcomes, open_questions, do_not_do_list.
- One primary agent per stage: Supply Chain owns fulfillment; Finance owns invoicing and collections; Sales owns customer communication.
- No agent posts or reverses a financial transaction — all postings require human confirmation above materiality threshold.
- Escalations are logged with trigger, timestamp, and receiving owner.

## KPIs
- Order fulfillment cycle time (order confirmation to shipment)
- Invoice accuracy rate (first-pass match to packing slip)
- Days sales outstanding (DSO)
- Cash application rate (% of invoices settled within payment terms)
- Collection effectiveness index (CEI)

## References
- [Order to cash end-to-end overview — Dynamics 365](https://learn.microsoft.com/dynamics365/guidance/business-processes/order-to-cash-overview)
- [Order to cash business process areas overview](https://learn.microsoft.com/dynamics365/guidance/business-processes/order-to-cash-areas-overview)
- [Manage accounts receivable — Dynamics 365 Finance](https://learn.microsoft.com/dynamics365/guidance/business-processes/order-to-cash-areas-overview#manage-accounts-receivable)
- [Work with order to cash in Dynamics 365 Supply Chain Management (training)](https://learn.microsoft.com/training/modules/work-with-order-to-cash-supply-chain-management/)
