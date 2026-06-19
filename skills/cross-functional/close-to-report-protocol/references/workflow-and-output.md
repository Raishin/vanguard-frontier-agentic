# Close-to-Report Protocol — Workflow and Output Contract

## Overview
This document provides the detailed step-by-step workflow, decision tree, and output contract for the close-to-report-protocol skill. It covers the full financial period close sequence from the opening of the close workspace through certified report distribution in Microsoft Fabric and Power BI.

---

## Detailed Step-by-Step Workflow

### Phase 1 — Period Close Workspace Initialization (d365-finance-close-to-report-agent)

**Step 1.1 — Load close template**
- Open the Financial Period Close workspace in Dynamics 365 Finance.
- Select the appropriate close template for the period type: month-end, quarter-end, or year-end.
- Verify that all task assignments, due dates, and dependencies in the template are current.
- Confirm the period end date and legal entities in scope.

**Step 1.2 — Pre-close validation**
- Confirm that all sales orders, purchase orders, and project transactions that should be in the period have been posted.
- Identify any unposted transactions that require attention before subledger close.
- Surface a pre-close readiness summary to the finance owner; do not begin subledger close until the owner confirms readiness.

---

### Phase 2 — Subledger Close (d365-finance-close-to-report-agent)

**Step 2.1 — Accounts Payable close**
- Confirm all vendor invoices for the period are posted.
- Run vendor aging report; confirm no unexpected open items.
- Mark Accounts Payable subledger as closed for the period.

**Step 2.2 — Accounts Receivable close**
- Confirm all customer invoices, credit memos, and cash applications for the period are posted.
- Run customer aging report; confirm AR balance against the general ledger.
- Mark Accounts Receivable subledger as closed for the period.

**Step 2.3 — Fixed Assets close**
- Run depreciation for all fixed asset books for the period.
- Post depreciation journals.
- Reconcile fixed asset net book value to the general ledger balance.

**Step 2.4 — Inventory close**
- Run inventory close or cost adjustment in Dynamics 365 Supply Chain Management.
- Confirm that inventory transaction costs are settled and inventory value agrees to the general ledger.

**Step 2.5 — Project Operations close (if applicable)**
- Post all project costs, revenue recognition, and WIP adjustments for the period.
- Confirm project subledger agrees to the general ledger.

**Subledger close gate:**
- d365-finance-close-to-report-agent must confirm that all applicable subledgers are closed before proceeding to general ledger activities.
- Any open subledger item above the materiality threshold halts the process.

---

### Phase 3 — Reconciliation and Adjustments (d365-finance-close-to-report-agent)

**Step 3.1 — Bank reconciliation**
- Run automated bank statement matching for all bank accounts.
- Investigate and document all unmatched items.
- Escalate any unexplained reconciling item above the materiality threshold to the finance owner.
- Record bank reconciliation sign-off for each account.

**Step 3.2 — Currency revaluation**
- Run foreign currency revaluation for all applicable accounts (AR, AP, bank, intercompany).
- Validate the FX rate source against the approved external reference (e.g., European Central Bank or configured rate provider).
- Post revaluation adjustments; confirm posting with supporting rate documentation.

**Step 3.3 — Ledger-to-subledger reconciliation**
- Run the trial balance and compare general ledger balances to each subledger summary.
- Zero unexplained differences required. Any difference halts the close process.
- Document all reconciling items with business reason and resolution.

**Step 3.4 — Period-end accruals and adjustments**
- Post all approved accrual and deferral journals.
- Every manual journal entry must have: business reason, supporting document, preparer identity, and human approver confirmation.
- Reject any manual journal entry without complete documentation.

---

### Phase 4 — Consolidation (d365-finance-close-to-report-agent, multi-entity only)

**Step 4.1 — Intercompany eliminations**
- Identify all intercompany balances across legal entities.
- Post elimination journals to remove intercompany revenues, costs, receivables, and payables.
- Confirm eliminations net to zero.

**Step 4.2 — Consolidation run**
- Run financial consolidation in Dynamics 365 Finance.
- Validate that the consolidated trial balance is complete and all entities are included.

---

### Phase 5 — Financial Statement Preparation and Sign-off (d365-finance-close-to-report-agent + human)

**Step 5.1 — Draft financial statements**
- Generate draft balance sheet, income statement, and cash flow statement.
- Run standard analytical checks: significant period-over-period variances, unusual account balances.
- Surface variance explanations to the finance owner for review.

**Step 5.2 — Reconciliation sign-off gate**
- Finance owner reviews the reconciliation package and draft financial statements.
- This gate must be explicitly passed before any report data is released to fabric-power-bi-business-insights-governance-agent.
- Sign-off is recorded with timestamp and owner identity.

---

### Phase 6 — Report Publication (fabric-power-bi-business-insights-governance-agent)

**Step 6.1 — Data refresh governance**
- fabric-power-bi-business-insights-governance-agent receives the certified close confirmation from d365-finance-close-to-report-agent.
- Refresh financial datasets in Microsoft Fabric with certified period data.
- Lock the reporting period to prevent further data changes.

**Step 6.2 — Report certification gate**
- Finance owner certifies the published reports as accurate before distribution.
- Reports are marked with certification timestamp, period, and certifying owner.

**Step 6.3 — Distribution**
- Distribute certified reports to the defined recipient list via Power BI.
- Record distribution event: timestamp, recipients, report version, and certifying owner.

---

## Decision Tree

```
START: Period end date reached
│
├─ All transactions posted (pre-close readiness)?
│   └─ NO → SURFACE to finance owner; do not begin subledger close
│
├─ All subledgers closed with zero unexplained differences?
│   └─ NO → HALT; escalate to subledger owner
│
├─ Bank accounts reconciled?
│   └─ NO (unexplained items above threshold) → HALT; escalate to finance owner
│
├─ Currency revaluation complete with approved FX rates?
│   └─ NO → HOLD; validate rate source
│
├─ Ledger-to-subledger reconciliation: zero differences?
│   └─ NO → HALT; investigate and resolve
│
├─ All manual journals documented and approved?
│   └─ NO → REJECT; require documentation
│
├─ Consolidation and eliminations complete (multi-entity)?
│   └─ NO → HOLD; complete consolidation
│
├─ Reconciliation sign-off from finance owner?
│   └─ NO → BLOCK report data release to Fabric/Power BI
│
├─ Report certification from finance owner?
│   └─ NO → BLOCK distribution
│
└─ Distribution complete?
    └─ YES → CLOSE period; record audit trail
```

---

## Output Contract

Every execution of this protocol produces a structured output capsule:

| Field | Type | Description |
|---|---|---|
| matter_id | string | Unique identifier for this close-to-report instance |
| period_id | string | Fiscal period identifier (e.g., 2026-06) |
| legal_entities | array | Legal entities in scope |
| current_stage | enum | One of: initialization, subledger_close, reconciliation, consolidation, financial_statements, report_publication, closed |
| gate_outcomes | object | Pass/fail/escalated: subledger_close, bank_recon, fx_revaluation, ledger_recon, recon_signoff, report_certification |
| agents_involved | array | Agent IDs that participated |
| escalations | array | Each escalation: {trigger, timestamp, escalated_to, reason} |
| open_questions | array | Unresolved items requiring human input |
| do_not_do_list | array | Actions explicitly prohibited in the current state |
| evidence_quality | enum | high / medium / low |
| privilege_sensitivity | boolean | True if statements contain material non-public financial data |
| last_updated | ISO8601 timestamp | When the capsule was last updated |

---

## Audit Log Fields
- matter_id, skill_id, skill_version, invoked_by, period_id, input_hash, evidence_quality, output_verdict, escalation_fired, recon_signoff_recorded, report_certification_recorded, timestamp
