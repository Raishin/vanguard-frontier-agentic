# Accounting Agents

Advisory agents for corporate accounting workflows — revenue recognition, financial close, reconciliation, and audit evidence — grounded in GAAP (ASC) and IFRS standards.

## Agents

| Agent | Role | Standard |
|---|---|---|
| [accounting-maestro-agent](./accounting-maestro-agent/) | Route and coordinate accounting tasks | All |
| [accounting-revenue-recognition-advisor-agent](./accounting-revenue-recognition-advisor-agent/) | ASC 606 / IFRS 15 revenue recognition review | ASC 606, IFRS 15 |

## Design Principles

- **Read-only, advisory**: No agent writes to any ledger, ERP, or system of record.
- **Cited**: Every conclusion references the specific GAAP paragraph or IFRS section it relies on.
- **Conservative**: All outputs are `advisory`, not `authoritative`. Material transactions require external auditor review.
- **Zero trust**: No agent accepts financial data beyond what is necessary for the specific question asked.
- **Auditable**: Response structure explicitly separates confirmed facts, applied standards, key judgments, and open unknowns.

## Scope

These agents address the **accounting** function (Controller, Chief Accounting Officer): revenue recognition, financial reporting, close cycle, and GAAP/IFRS compliance. For FP&A, treasury, and corporate finance workflows, see [`../finance/`](../finance/).

## Not In Scope

- Posting journal entries or modifying accounting records
- Making final accounting determinations for material transactions
- Replacing external auditor or Big 4 review
- Providing tax advice (federal, state, international)
- Providing legal advice or forming an accountant-client relationship
