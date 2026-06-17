# Lead-to-Cash Protocol — Workflow and Output Contract

## Overview
This document provides the detailed step-by-step workflow, decision tree, and output contract for the lead-to-cash-protocol skill. It is the authoritative reference for agent implementations and human reviewers who need to trace execution through the protocol.

---

## Detailed Step-by-Step Workflow

### Phase 1 — Opportunity Qualification (d365-sales-revenue-operations-agent)

**Step 1.1 — Receive and validate opportunity signal**
- Confirm the opportunity record exists in Dynamics 365 Sales with a stage of "Qualified" or "Proposal Accepted".
- Verify that an estimated revenue, close date, and primary contact are populated.
- If the opportunity stage is earlier than "Qualified", stop and return to the prospect-to-quote process; do not advance.

**Step 1.2 — Quote or purchase order confirmation**
- Confirm that a signed customer quote (in Dynamics 365 Sales) or an accepted customer purchase order is on file.
- Validate that pricing, discount bands, and delivery terms on the quote match the approved trade agreement or contract.
- Any discount that exceeds the configured approval threshold must have a documented manager approval before advancing.

**Step 1.3 — Revenue recognition pre-classification**
- Identify whether the contract contains multiple performance obligations (ASC 606 / IFRS 15).
- If contract complexity is detected (e.g., bundled products and services, milestone billing, variable consideration), flag for d365-finance-close-to-report-agent before order creation.
- Document classification decision and evidence quality in the handoff capsule.

---

### Phase 2 — Credit and Order Gate (d365-finance-close-to-report-agent + human)

**Step 2.1 — Credit check**
- Query the customer's credit limit and current exposure in Dynamics 365 Finance.
- If the order value plus existing open AR balance exceeds the credit limit, the protocol halts.
- Produce a credit risk summary for the finance credit manager; do not override or assume approval.
- Record the outcome (approved / denied / escalated) before proceeding.

**Step 2.2 — Order accuracy review**
- Compare line items, quantities, unit prices, delivery addresses, and tax codes on the draft sales order against the accepted quote.
- Flag any discrepancy for human correction. Do not post an inaccurate order.
- Confirm that the legal entity, currency, and applicable price list are correct.

---

### Phase 3 — Order Fulfillment (d365-supply-chain-plan-to-produce-agent)

**Step 3.1 — Inventory availability check**
- Query available-to-promise (ATP) data for each order line.
- If ATP date is after the requested delivery date, surface the gap to the sales agent and human owner; propose alternatives but do not commit a delivery date unilaterally.

**Step 3.2 — Order confirmation**
- Confirm the sales order in Dynamics 365 Supply Chain Management once credit and order accuracy gates are passed.
- Trigger warehouse picking and packing workflows.

**Step 3.3 — Shipment and delivery note**
- Monitor picking and packing completion.
- Upon shipment, generate a packing slip / delivery note in the system.
- Hand the packing slip reference and shipment date to d365-finance-close-to-report-agent.

---

### Phase 4 — Invoicing and Revenue Recognition (d365-finance-close-to-report-agent)

**Step 4.1 — Invoice generation**
- Create a sales order invoice in Dynamics 365 Finance based on the confirmed packing slip.
- Validate that invoice lines match the packing slip quantities and the original sales order pricing.
- Apply the correct revenue recognition schedule per the pre-classification in Step 1.3.

**Step 4.2 — Invoice posting**
- Human owner confirms invoice accuracy before posting if the invoice exceeds a configurable materiality threshold.
- Post the invoice; accounts receivable entry is created.

**Step 4.3 — Cash application and close**
- Monitor customer payment receipt.
- Apply cash to the open AR entry and settle the invoice.
- Update opportunity status in Dynamics 365 Sales to "Closed Won" and record the final revenue amount.

---

## Decision Tree

```
START: Opportunity signal received
│
├─ Opportunity stage < Qualified?
│   └─ YES → STOP; return to prospect-to-quote process
│
├─ Signed quote or accepted PO on file?
│   └─ NO → STOP; escalate to d365-sales-revenue-operations-agent
│
├─ Discount within approval threshold?
│   └─ NO → PAUSE; require documented manager approval
│
├─ Revenue recognition complexity detected?
│   └─ YES → FLAG to d365-finance-close-to-report-agent before order creation
│
├─ Credit check passed?
│   └─ NO → HALT; escalate to finance credit manager; do not proceed
│
├─ Order accuracy confirmed?
│   └─ NO → RETURN to d365-sales-revenue-operations-agent for correction
│
├─ ATP date within requested delivery window?
│   └─ NO → SURFACE gap to human owner; do not commit delivery date
│
├─ Invoice matches packing slip and order?
│   └─ NO → HOLD posting; flag for human review
│
└─ Cash received and applied?
    └─ YES → CLOSE revenue cycle; update opportunity to Closed Won
```

---

## Output Contract

Every execution of this protocol produces a structured output capsule with the following mandatory fields:

| Field | Type | Description |
|---|---|---|
| matter_id | string | Unique identifier for this lead-to-cash instance |
| opportunity_id | string | Dynamics 365 Sales opportunity record ID |
| sales_order_id | string | Confirmed sales order number |
| current_stage | enum | One of: qualification, credit_check, order_confirmed, fulfillment, invoiced, closed |
| gate_outcomes | object | Pass/fail/escalated for each gate: credit, order_accuracy, atp, invoice_match |
| agents_involved | array | List of agent IDs that participated in this execution |
| escalations | array | Each escalation: {trigger, timestamp, escalated_to, reason} |
| open_questions | array | Any unresolved items requiring human input |
| do_not_do_list | array | Actions explicitly prohibited in the current state |
| evidence_quality | enum | high / medium / low — reflecting confidence in input data |
| privilege_sensitivity | boolean | True if the matter contains legally sensitive commercial terms |
| last_updated | ISO8601 timestamp | When the capsule was last updated |

### Example output capsule (partial)
```json
{
  "matter_id": "L2C-2026-001234",
  "opportunity_id": "OPP-98765",
  "sales_order_id": "SO-20260001",
  "current_stage": "invoiced",
  "gate_outcomes": {
    "credit": "pass",
    "order_accuracy": "pass",
    "atp": "pass",
    "invoice_match": "pass"
  },
  "agents_involved": [
    "d365-sales-revenue-operations-agent",
    "d365-supply-chain-plan-to-produce-agent",
    "d365-finance-close-to-report-agent"
  ],
  "escalations": [],
  "open_questions": [],
  "do_not_do_list": [
    "Do not adjust invoice amounts without human approval",
    "Do not override credit limit without documented approval"
  ],
  "evidence_quality": "high",
  "privilege_sensitivity": false,
  "last_updated": "2026-06-16T10:30:00Z"
}
```

---

## Audit Log Fields
- matter_id, skill_id, skill_version, invoked_by, input_hash, evidence_quality, output_verdict, escalation_fired, timestamp
