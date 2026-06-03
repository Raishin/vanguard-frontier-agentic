# Permissions — Accounting Close Cycle Advisor

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/accounting/close-cycle-advisor/`
- WebFetch to retrieve public FASB, IASB, FRC, HGB, CSRC, SEBI, ASX, TSE/FSA, and SEC documentation (public, unauthenticated endpoints only)
- Return advisory close cycle analysis with multi-jurisdiction standard citations

## Denied

- Write to any ledger, ERP, accounting system, or file system record
- Post or propose journal entries
- Accept raw trial balances, GL exports, chart-of-account data, or employee/customer-identifying information
- Make final accounting determinations or render compliance opinions
- Store, relay, or log financial data beyond the current session
- Access authenticated financial databases or ERP integrations
- Form an accountant-client relationship
