# Finance Agents

Advisory agents for corporate finance workflows — FP&A variance analysis, management commentary, treasury liquidity monitoring, and capital allocation review — grounded in SEC Regulation S-K, FASB standards, and institutional finance best practices.

## Agents

| Agent | Role | Standard |
|---|---|---|
| [finance-maestro-agent](./finance-maestro-agent/) | Route and coordinate corporate finance tasks | All |
| [finance-variance-analysis-advisor-agent](./finance-variance-analysis-advisor-agent/) | Budget vs. actual variance analysis and MD&A commentary | SEC Reg S-K Item 303, FASB ASC 270 |

## Design Principles

- **Read-only, advisory**: No agent writes to planning systems, ERP, or systems of record.
- **Cited**: MD&A commentary conclusions reference specific SEC Regulation S-K or FASB standard requirements.
- **Conservative**: All outputs are `advisory` — never `authoritative`. Final filings require CFO and legal review.
- **Zero trust**: No agent accepts raw financial statements with company-identifying information unless the user supplies the data as context.
- **Auditable**: Response structure explicitly separates data inputs, applied frameworks, key drivers, and assumptions.

## Scope

These agents address the **corporate finance** function (CFO office, FP&A, Treasury, IR): variance analysis, management commentary, treasury, and capital allocation. For GAAP/IFRS accounting compliance workflows, see [`../accounting/`](../accounting/).

## Not In Scope

- Writing or filing SEC disclosures on behalf of a company
- Making investment decisions or trading recommendations
- Providing tax advice
- Writing to planning systems, ERP, or financial databases
- Providing legal advice or forming a financial-advisor relationship
