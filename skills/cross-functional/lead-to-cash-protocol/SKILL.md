---
name: lead-to-cash-protocol
description: Use this skill to orchestrate the end-to-end lead-to-cash business process across Dynamics 365 Sales, Supply Chain Management, and Finance. It coordinates the journey from a qualified sales opportunity through order creation, fulfillment, invoicing, and final revenue recognition. The skill defines stage gates, agent handoff rules, escalation triggers, and decision logic for cross-functional coordination. It does not make business decisions, approve credit limits, authorize revenue recognition treatments, or execute any system transactions; all production-impacting steps are escalated to the relevant specialist or human owner.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-16"
  category: operational
  lifecycle: experimental
---

# Lead-to-Cash Protocol

## Purpose
This skill orchestrates the cross-functional lead-to-cash process in Microsoft Dynamics 365 environments. It bridges Dynamics 365 Sales (opportunity and quote management), Dynamics 365 Supply Chain Management (order fulfillment), and Dynamics 365 Finance (invoicing, accounts receivable, and revenue recognition). The protocol defines stage ownership, gate conditions, escalation paths, and refusal triggers so that no single agent works in isolation on a revenue-impacting flow.

## When to use
- A qualified sales opportunity must progress to a confirmed sales order.
- A sales order requires coordination with supply chain fulfillment and finance invoicing.
- A credit check, order accuracy review, or revenue recognition event is required.
- Multiple Dynamics 365 modules are involved and agent handoffs must be auditable.
- A revenue-impacting matter crosses Sales, Supply Chain, and Finance domains simultaneously.

## When NOT to use
- The matter is contained within a single Dynamics 365 module and no cross-domain coordination is needed.
- The ask is for direct configuration advice, live system transactions, or field-level setup guidance — route to the relevant specialist agent instead.
- The opportunity has not yet been qualified — use the prospect-to-quote process first.
- The ask involves collection disputes or credit limit adjustments without an upstream opportunity — route directly to the finance agent.

## Participating agents
- d365-sales-revenue-operations-agent — owns opportunity qualification, quote approval, and order entry
- d365-supply-chain-plan-to-produce-agent — owns order fulfillment, inventory reservation, and delivery
- d365-finance-close-to-report-agent — owns invoicing, accounts receivable, revenue recognition, and cash application

## Inputs required
- Opportunity or sales order identifier
- Legal entity and currency
- Requested delivery date or fulfillment window
- Revenue recognition method (ASC 606 / IFRS 15 classification if applicable)
- Credit limit status for the customer account
- Applicable pricing agreement or trade agreement reference

## Evidence required
- Opportunity stage confirmation from d365-sales-revenue-operations-agent
- Credit check outcome from finance or credit management system
- Inventory availability signal from d365-supply-chain-plan-to-produce-agent
- Revenue recognition classification from d365-finance-close-to-report-agent
- Approval chain documentation for discount exceptions above threshold

## Workflow
1. Receive opportunity handoff from d365-sales-revenue-operations-agent; verify stage is "Qualified" or "Proposal Accepted".
2. Validate that a signed quote or accepted purchase order exists before advancing to order creation.
3. Invoke credit check gate: if customer exceeds credit limit, pause and escalate to finance owner; do not proceed.
4. Confirm order accuracy: line items, pricing, delivery terms, and tax treatment match the accepted quote.
5. Trigger order confirmation in Dynamics 365 Supply Chain Management via d365-supply-chain-plan-to-produce-agent.
6. Monitor fulfillment: picking, packing, and shipping status against promised delivery date.
7. Generate packing slip / delivery note upon shipment; hand off to d365-finance-close-to-report-agent for invoice generation.
8. Validate invoice against sales order and packing slip before posting; flag discrepancies for human review.
9. Post invoice and initiate accounts receivable tracking; apply revenue recognition rule per contract classification.
10. Confirm cash receipt and settlement; close the revenue cycle and update opportunity as "Closed Won".

## Decision gates
| Gate | Condition | Action if not met |
|---|---|---|
| Credit check | Customer credit within approved limit | Pause; escalate to finance credit manager |
| Order accuracy | Line items and pricing match accepted quote | Return to d365-sales-revenue-operations-agent for correction |
| Inventory availability | Stock confirmed before order confirmation | Notify d365-supply-chain-plan-to-produce-agent; assess ATP date |
| Revenue recognition | Contract classification confirmed | Escalate to d365-finance-close-to-report-agent and finance owner |
| Invoice match | Invoice matches order and packing slip | Hold posting; flag discrepancy for human review |

## Refusal triggers
- A production system transaction is requested without human confirmation — refuse and escalate.
- Credit limit override is requested without documented approval — refuse.
- Revenue recognition treatment is ambiguous and jurisdiction is unknown — pause and escalate to finance owner.
- Customer PII or payment credentials are included in any input — stop immediately; redact and escalate.
- A live pricing or discount change is requested outside an approved workflow — refuse.

## Handoff rules
- Every handoff uses a structured context capsule containing: matter_id, stage, evidence quality, open questions, do_not_do_list, and privilege sensitivity flag.
- Primary agent per stage: Sales (opportunity → quote), Supply Chain (order → fulfillment), Finance (invoice → cash).
- No parallel edits to the same sales order from multiple agents without explicit sequencing.
- All escalations to human owners are logged with timestamp, reason, and agent that triggered them.

## KPIs
- Days from opportunity close to order confirmation
- Order accuracy rate (lines matching quote on first pass)
- Days sales outstanding (DSO)
- Invoice-to-cash cycle time
- Revenue recognition compliance rate (no late adjustments)

## References
- [Dynamics 365 order-to-cash overview](https://learn.microsoft.com/dynamics365/guidance/business-processes/order-to-cash-overview)
- [Prospect to quote end-to-end overview](https://learn.microsoft.com/dynamics365/guidance/business-processes/prospect-to-quote-overview)
- [Manage accounts receivable — Dynamics 365 Finance](https://learn.microsoft.com/dynamics365/guidance/business-processes/order-to-cash-areas-overview#manage-accounts-receivable)
- [Revenue recognition in Dynamics 365 Finance](https://learn.microsoft.com/dynamics365/finance/accounts-receivable/revenue-recognition-overview)
