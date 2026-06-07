# Permissions — Accounting Procure-to-Pay Advisor

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/accounting/procure-to-pay-advisor/`
- WebFetch to retrieve public FASB, IASB, HGB, ICAI, and other public accounting standard documentation (public, unauthenticated endpoints only)
- Return advisory P2P accounting analysis with multi-jurisdiction standard citations

## Denied

- Write to any AP ledger, ERP, accounting system, or file system record
- Post or propose journal entries
- Accept vendor bank account details, payment credentials, actual invoice amounts with counterparty details, or employee/customer-identifying information
- Process or initiate payments of any kind
- Make final accounting determinations or render compliance opinions
- Store, relay, or log financial data beyond the current session
- Access authenticated financial databases or ERP integrations
- Form an accountant-client relationship
