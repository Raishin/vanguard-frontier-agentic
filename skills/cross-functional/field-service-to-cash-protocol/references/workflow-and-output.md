# Field Service to Cash Protocol — Workflow and Output Contract

## Overview
This document provides the detailed step-by-step workflow, decision tree, and output contract for the field-service-to-cash-protocol skill. It covers the complete service-to-deliver cycle from work order creation through cash application in Dynamics 365 Field Service and Finance.

---

## Detailed Step-by-Step Workflow

### Phase 1 — Work Order Initialization (d365-field-service-to-cash-agent)

**Step 1.1 — Work order creation and validation**
- Receive the service request and create a work order in Dynamics 365 Field Service.
- Confirm that the following are populated: customer account, service type, scope of work, billing method (time and materials, fixed price, or agreement invoice), and applicable service agreement reference.
- If any required field is missing, surface the gap to the human owner before proceeding.

**Step 1.2 — Parts and resource requirements**
- Identify required parts, materials, and specialist skills for the work order.
- Check inventory availability at the nearest warehouse or the technician's truck stock.
- If required parts are not available, trigger a parts transfer or purchase order request; do not schedule the work order for a date when parts will not be available.

**Step 1.3 — Service agreement check**
- If the work order is covered by a service agreement or maintenance plan, validate that the agreement is active and the work type is within scope.
- For agreement-based billing, confirm the invoice generation schedule and pricing from the agreement record.

---

### Phase 2 — Scheduling and Dispatch (d365-field-service-to-cash-agent)

**Step 2.1 — Technician scheduling**
- Use the Dynamics 365 Field Service scheduling board to assign a qualified technician with the required skills.
- Confirm the scheduled date, estimated duration, and travel requirements.
- Record the assignment in the work order.

**Step 2.2 — Work order communication**
- The scheduling details are available to the technician via the Field Service mobile application.
- Do not communicate customer site addresses or personal contact details outside of the configured Field Service channel.

---

### Phase 3 — Service Delivery (d365-field-service-to-cash-agent)

**Step 3.1 — Service execution monitoring**
- Monitor work order status as the technician performs the service.
- Flag work orders that exceed the estimated duration or have not been updated past the scheduled completion time.

**Step 3.2 — Labor and parts recording**
- Technician records all labor time (including travel if billable), parts consumed, and any additional work performed on the work order in Dynamics 365 Field Service.
- Parts consumed are posted as inventory usage; inventory levels in the assigned location are updated automatically.

**Step 3.3 — Work order completion**
- Technician marks the work order as "Complete" in Dynamics 365 Field Service.
- If customer sign-off is required by the service contract, confirm that the customer signature or approval is recorded before closing the work order.

**Work order verification gate:**
- d365-field-service-to-cash-agent confirms the work order is in "Complete" status with all labor, parts, and customer sign-off (if required) recorded.
- If any item is missing, the work order remains open; invoicing does not proceed.

---

### Phase 4 — Inventory Reconciliation (d365-field-service-to-cash-agent)

**Step 4.1 — Inventory adjustment confirmation**
- Confirm that parts consumed on the work order have been reflected as inventory usage transactions in the system.
- Reconcile the parts list on the work order against the inventory adjustment records.
- Any discrepancy between work order parts and inventory records must be resolved before invoicing.

**Step 4.2 — Return of unused parts**
- If parts were issued for the work order but not consumed, confirm that return-to-warehouse transactions are posted.
- Unused parts must not remain on the work order as consumed.

---

### Phase 5 — Invoice Creation and Posting (d365-finance-close-to-report-agent)

**Step 5.1 — Work order actuals handoff**
- d365-field-service-to-cash-agent provides a structured work order actuals summary to d365-finance-close-to-report-agent:
  - Labor hours (regular, overtime, travel)
  - Parts consumed with unit prices
  - Any additional charges
  - Customer and work order references

**Step 5.2 — Invoice draft creation**
- d365-finance-close-to-report-agent creates a draft invoice in Dynamics 365 Finance.
- Invoice lines are validated against the work order actuals and the applicable service contract pricing.
- Tax codes, payment terms, and customer billing address are confirmed.

**Invoice accuracy gate:**
- Any discrepancy between the invoice draft and work order actuals, or between invoice pricing and contract rates, triggers a hold.
- The discrepancy is surfaced to d365-field-service-to-cash-agent and the human owner for resolution.
- Do not post the invoice until the accuracy gate is passed.

**Step 5.3 — Invoice posting**
- Human owner confirms the invoice for amounts above the configured materiality threshold.
- Invoice is posted in Dynamics 365 Finance; accounts receivable entry is opened.
- Invoice is delivered to the customer via the configured channel (email, portal, or printed copy).

---

### Phase 6 — Collections and Cash Application (d365-finance-close-to-report-agent)

**Step 6.1 — Payment monitoring**
- Monitor the accounts receivable aging for the field service invoice.
- At configured intervals, surface recommended collection activities to the finance owner.
- Do not contact the customer directly.

**Step 6.2 — Cash application**
- Upon payment receipt, apply cash to the matching invoice in Dynamics 365 Finance.
- Partial payments are flagged to the finance owner; do not auto-close the AR entry.

**Step 6.3 — Settlement and close**
- Fully settled invoices are closed; work order revenue is recorded.
- Update the field service performance metrics in d365-field-service-to-cash-agent.

---

## Decision Tree

```
START: Service request received
│
├─ Work order fields complete (account, scope, billing method)?
│   └─ NO → SURFACE to human owner; do not proceed
│
├─ Required parts available at assignment location?
│   └─ NO → TRIGGER parts transfer or PO; do not schedule before parts confirmed
│
├─ Service agreement active and work type in scope?
│   └─ NO → ESCALATE to field service and finance owners
│
├─ Work order marked "Complete" in Field Service?
│   └─ NO → DO NOT proceed to invoicing
│
├─ Customer sign-off recorded (if required)?
│   └─ NO → HOLD; await sign-off
│
├─ Inventory adjustment confirmed for consumed parts?
│   └─ NO → RESOLVE discrepancy before invoicing
│
├─ Invoice accurate (matches actuals and contract pricing)?
│   └─ NO → HOLD; route to field service agent and human owner
│
├─ Invoice above materiality threshold?
│   └─ YES → REQUIRE human approval before posting
│
├─ Payment received in full?
│   └─ NO (partial) → FLAG to finance owner; do not auto-close AR
│
└─ Invoice settled?
    └─ YES → CLOSE AR; update field service performance metrics
```

---

## Output Contract

Every execution of this protocol produces a structured output capsule:

| Field | Type | Description |
|---|---|---|
| matter_id | string | Unique identifier for this field service to cash instance |
| work_order_id | string | Dynamics 365 Field Service work order number |
| current_stage | enum | One of: work_order_created, scheduled, in_progress, completed, inventory_reconciled, invoice_created, invoiced, collections, closed |
| gate_outcomes | object | Pass/fail/escalated: parts_availability, work_order_completion, customer_signoff, inventory_adjustment, invoice_accuracy, cash_application |
| agents_involved | array | Agent IDs that participated |
| escalations | array | Each escalation: {trigger, timestamp, escalated_to, reason} |
| open_questions | array | Unresolved items requiring human input |
| do_not_do_list | array | Actions explicitly prohibited in the current state |
| evidence_quality | enum | high / medium / low |
| privilege_sensitivity | boolean | True if service contract contains commercially sensitive pricing |
| last_updated | ISO8601 timestamp | When the capsule was last updated |

### Example output capsule (partial)
```json
{
  "matter_id": "FSC-2026-005678",
  "work_order_id": "WO-20260345",
  "current_stage": "invoiced",
  "gate_outcomes": {
    "parts_availability": "pass",
    "work_order_completion": "pass",
    "customer_signoff": "pass",
    "inventory_adjustment": "pass",
    "invoice_accuracy": "pass",
    "cash_application": "pending"
  },
  "agents_involved": [
    "d365-field-service-to-cash-agent",
    "d365-finance-close-to-report-agent"
  ],
  "escalations": [],
  "open_questions": ["Payment due 2026-06-30; monitoring AR aging"],
  "do_not_do_list": [
    "Do not auto-close AR entry on partial payment",
    "Do not modify posted invoice without human authorization"
  ],
  "evidence_quality": "high",
  "privilege_sensitivity": false,
  "last_updated": "2026-06-16T14:45:00Z"
}
```

---

## Audit Log Fields
- matter_id, skill_id, skill_version, invoked_by, work_order_id, input_hash, evidence_quality, output_verdict, escalation_fired, timestamp
