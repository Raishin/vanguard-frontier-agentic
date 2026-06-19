# Order-to-Cash Protocol — Workflow and Output Contract

## Overview
This document provides the detailed step-by-step workflow, decision tree, and output contract for the order-to-cash-protocol skill. It is the authoritative reference for agent implementations and human reviewers tracing execution through the post-order revenue cycle.

---

## Detailed Step-by-Step Workflow

### Phase 1 — Order Validation (d365-sales-revenue-operations-agent + d365-supply-chain-plan-to-produce-agent)

**Step 1.1 — Receive confirmed sales order**
- Accept the confirmed sales order number from the upstream system or lead-to-cash-protocol handoff.
- Verify the order status is "Confirmed" in Dynamics 365 Supply Chain Management.
- Confirm that legal entity, currency, and customer account are correctly set.

**Step 1.2 — Order accuracy review**
- Compare all line items: item numbers, quantities, unit prices, discount rates, delivery addresses, and requested delivery dates against the customer's accepted quote or purchase order.
- Any discrepancy must be resolved by d365-sales-revenue-operations-agent and the customer before fulfillment proceeds.

**Step 1.3 — Credit status check**
- Query the customer's credit limit and current accounts receivable exposure in Dynamics 365 Finance.
- If the customer account is on credit hold, halt fulfillment; escalate to d365-finance-close-to-report-agent and the finance credit manager.
- Do not release from credit hold without documented human approval.

---

### Phase 2 — Fulfillment (d365-supply-chain-plan-to-produce-agent)

**Step 2.1 — Inventory reservation and ATP check**
- Reserve available inventory against each order line.
- Calculate the available-to-promise (ATP) date for any line where stock is insufficient.
- If the ATP date exceeds the promised delivery date, surface the gap to d365-sales-revenue-operations-agent and the human owner immediately; do not silently delay shipment.

**Step 2.2 — Warehouse picking and packing**
- Trigger picking work in the warehouse management system.
- Monitor picking completion; flag pick shortages for supply chain owner resolution.
- Confirm packing and generate a packing slip with shipment reference.

**Step 2.3 — Shipment and delivery confirmation**
- Record the shipment date and carrier details against the sales order.
- Generate the delivery note or bill of lading reference.
- Hand the packing slip and shipment confirmation to d365-finance-close-to-report-agent.

---

### Phase 3 — Invoicing (d365-finance-close-to-report-agent)

**Step 3.1 — Invoice creation**
- Generate a draft sales order invoice in Dynamics 365 Finance using the confirmed packing slip.
- Validate that invoice lines match packing slip quantities and original sales order pricing.
- Apply correct tax codes and customer-specific payment terms.

**Step 3.2 — Invoice accuracy gate**
- Compare draft invoice against sales order and packing slip line by line.
- Any discrepancy (price, quantity, tax) triggers a hold; route to d365-sales-revenue-operations-agent for resolution.
- Invoices above the configured materiality threshold require human approval before posting.

**Step 3.3 — Invoice posting and AR entry**
- Post the validated invoice in Dynamics 365 Finance.
- Open an accounts receivable entry with the customer's due date based on payment terms.
- Deliver the invoice to the customer via the configured channel (email, portal, EDI).

---

### Phase 4 — Collections and Cash Application (d365-finance-close-to-report-agent)

**Step 4.1 — Payment monitoring**
- Monitor the accounts receivable aging report for the invoice due date.
- At configurable intervals before and after due date, surface collection activities to the finance owner.
- Do not contact the customer directly; surface recommended actions to the human owner.

**Step 4.2 — Cash application**
- Upon receipt of customer payment or remittance advice, apply cash to the matching open invoice.
- If the payment is short (partial payment or deduction), flag for finance owner; do not auto-close the AR entry.

**Step 4.3 — Settlement and close**
- Fully settled invoices are closed in Dynamics 365 Finance.
- Update the sales performance report in d365-sales-revenue-operations-agent.
- Confirm completion to the upstream lead-to-cash-protocol if this order-to-cash cycle was initiated from it.

---

## Decision Tree

```
START: Confirmed sales order received
│
├─ Order status confirmed in Supply Chain Management?
│   └─ NO → STOP; return to order management
│
├─ Order accuracy verified?
│   └─ NO → RETURN to d365-sales-revenue-operations-agent for correction
│
├─ Customer credit status: OK?
│   └─ NO → HALT; escalate to finance credit manager
│
├─ Inventory ATP within delivery window?
│   └─ NO → SURFACE gap to human owner; do not commit silently
│
├─ Picking and packing complete?
│   └─ NO → MONITOR; alert supply chain owner on delay
│
├─ Invoice accurate (matches packing slip + order)?
│   └─ NO → HOLD; flag for human resolution
│
├─ Invoice above materiality threshold?
│   └─ YES → REQUIRE human approval before posting
│
├─ Payment received in full?
│   └─ NO (partial) → FLAG for finance owner; do not auto-close
│
└─ Invoice settled?
    └─ YES → CLOSE AR; update sales reporting
```

---

## Output Contract

Every execution of this protocol produces a structured output capsule with the following mandatory fields:

| Field | Type | Description |
|---|---|---|
| matter_id | string | Unique identifier for this O2C instance |
| sales_order_id | string | Confirmed sales order number |
| current_stage | enum | One of: order_validated, credit_check, fulfillment, shipped, invoiced, collections, closed |
| gate_outcomes | object | Pass/fail/escalated for each gate: credit, order_accuracy, atp, invoice_match, collections |
| agents_involved | array | Agent IDs that participated |
| escalations | array | Each escalation: {trigger, timestamp, escalated_to, reason} |
| open_questions | array | Unresolved items requiring human input |
| do_not_do_list | array | Actions explicitly prohibited in the current state |
| evidence_quality | enum | high / medium / low |
| privilege_sensitivity | boolean | True if commercially sensitive terms are involved |
| last_updated | ISO8601 timestamp | When the capsule was last updated |

---

## Audit Log Fields
- matter_id, skill_id, skill_version, invoked_by, input_hash, evidence_quality, output_verdict, escalation_fired, timestamp
