---
name: procure-to-pay-protocol
description: Use this skill to orchestrate the procure-to-pay (source-to-pay) process across Dynamics 365 Supply Chain Management, Finance, and compliance-aware separation-of-duties governance. It coordinates the journey from purchase requisition through purchase order approval, goods or services receipt, vendor invoice processing, and payment settlement. The skill enforces segregation-of-duties gates on purchase order approval, defines agent handoff rules, and escalates SoD conflicts to d365-security-sod-governance-agent. It does not approve purchase orders, release vendor payments, override procurement policies, or access vendor credentials; all production-impacting steps are escalated to the relevant specialist or human owner.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-16"
  category: operational
  lifecycle: experimental
---

# Procure-to-Pay Protocol

## Purpose
This skill orchestrates the end-to-end procure-to-pay (P2P) / source-to-pay (S2P) business process in Microsoft Dynamics 365 environments. It coordinates Dynamics 365 Supply Chain Management (requisitions, purchase orders, and goods receipt), Dynamics 365 Finance (vendor invoice processing and payment), and Dynamics 365 security and compliance governance (segregation of duties on PO approval). The protocol ensures that no single agent approves, receives, and pays for the same purchase, and that SoD violations are identified and escalated before they reach the general ledger.

## When to use
- A purchase requisition must be evaluated, converted to a PO, approved, and tracked through receipt and payment.
- A vendor invoice must be matched to a PO and goods receipt before payment is released.
- A segregation-of-duties concern exists on a PO approval workflow.
- Multiple Dynamics 365 modules are involved and cross-agent coordination is required.
- A procurement compliance review is needed before a vendor payment run.

## When NOT to use
- The matter is confined to sales order fulfillment — use order-to-cash-protocol.
- Direct configuration of Dynamics 365 procurement workflows is required — route to the supply chain specialist.
- The matter is a vendor master data change — escalate to the supply chain specialist and finance owner.
- The ask is for strategic sourcing or vendor selection advice without an active procurement request.

## Participating agents
- d365-supply-chain-plan-to-produce-agent — owns purchase requisitions, purchase orders, and goods/services receipt
- d365-finance-close-to-report-agent — owns vendor invoice processing, accounts payable, and payment settlement
- d365-security-sod-governance-agent — reviews and flags segregation-of-duties violations on PO approval workflows

## Inputs required
- Purchase requisition or purchase order number
- Vendor account and legal entity
- Goods or services description, quantities, and agreed pricing
- Applicable procurement policy and approval workflow configuration
- Budget availability confirmation (if applicable)

## Evidence required
- Approved purchase requisition with budget check result
- Purchase order confirmation from vendor
- Goods receipt note or service entry sheet
- Vendor invoice with three-way match result (PO, receipt, invoice)
- SoD clearance from d365-security-sod-governance-agent for PO approval

## Workflow
1. Receive purchase requisition; validate against procurement catalog and purchasing policy.
2. Perform budget availability check; if budget is insufficient, pause and escalate to finance owner.
3. Convert approved requisition to a purchase order; confirm vendor, pricing, and delivery terms.
4. Route purchase order through the configured approval workflow.
5. SoD gate: invoke d365-security-sod-governance-agent to verify that the PO approver is not the same person who raised the requisition or will receive the goods.
6. If SoD conflict is detected, halt PO approval; escalate to compliance owner and document the conflict.
7. Send confirmed purchase order to vendor; record confirmation reference.
8. Monitor delivery against PO delivery date; flag overdue deliveries.
9. Record goods receipt or service entry sheet upon delivery; perform quantity and quality check against PO.
10. Receive vendor invoice in Dynamics 365 Finance; perform three-way match (PO, receipt, invoice).
11. If match discrepancy exists, hold invoice and route to d365-supply-chain-plan-to-produce-agent and vendor for resolution.
12. Approved and matched invoices are queued for payment; d365-finance-close-to-report-agent manages payment run per payment terms.
13. Release payment; apply to vendor open transactions and close accounts payable entries.

## Decision gates
| Gate | Condition | Action if not met |
|---|---|---|
| Budget availability | Sufficient budget in the appropriate cost center | Pause; escalate to finance owner for budget approval |
| Procurement policy | Request conforms to catalog and purchasing policy | Reject and return to requester |
| SoD on PO approval | Approver, requisitioner, and receiver are different people | Halt; escalate to d365-security-sod-governance-agent and compliance owner |
| Three-way match | PO, goods receipt, and invoice quantities and prices align | Hold invoice; route to supply chain agent and vendor for resolution |
| Payment authorization | Payment run approved by authorized finance signatory | Do not release payment without documented authorization |

## Refusal triggers
- PO approval is requested for a person who also raised the requisition — refuse; escalate SoD violation.
- Vendor payment is requested without a matched invoice and goods receipt — refuse.
- Vendor credentials, banking details, or PII are included in any input — stop; redact and escalate.
- A purchase order is requested to be back-dated — refuse; escalate to compliance owner.
- An invoice is requested to be paid without three-way match and no documented exception approval — refuse.

## Handoff rules
- Every handoff carries a structured capsule: matter_id, po_id, current_stage, gate_outcomes, sod_status, open_questions, do_not_do_list.
- Supply Chain owns requisition through goods receipt; Finance owns invoice processing through payment.
- d365-security-sod-governance-agent is invoked at PO approval and again at payment authorization if personnel assignments have changed.
- No agent releases a vendor payment — all payments require a human-authorized payment run.

## KPIs
- Purchase order cycle time (requisition to PO confirmation)
- Three-way match rate (first-pass match without manual intervention)
- Invoice processing time (receipt to posting)
- Payment on-time rate (payments within agreed vendor terms)
- SoD violation rate (conflicts detected per period)

## References
- [Source to pay end-to-end overview — Dynamics 365](https://learn.microsoft.com/dynamics365/guidance/business-processes/source-to-pay-overview)
- [TechTalk: Procure to pay in Dynamics 365 Supply Chain Management](https://learn.microsoft.com/dynamics365/guidance/techtalks/supply-chain-procure-to-pay-overview)
- [TechTalk: Source to pay in Dynamics 365 Finance and Supply Chain Management](https://learn.microsoft.com/dynamics365/guidance/techtalks/dynamics-365-finance-supply-chain-management-source-to-pay)
- [Procurement and sourcing overview — Dynamics 365 Supply Chain Management](https://learn.microsoft.com/dynamics365/supply-chain/procurement/procurement-sourcing-overview)
