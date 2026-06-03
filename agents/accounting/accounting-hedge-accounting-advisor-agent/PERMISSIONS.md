# Permissions — Accounting Hedge Accounting Advisor

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/accounting/hedge-accounting-advisor/`
- WebFetch to retrieve public FASB, IASB, HGB, ASBJ, MOF China, and ICAI documentation (public, unauthenticated endpoints only)
- Return advisory hedge accounting analysis with multi-jurisdiction standard citations

## Denied

- Write to any ledger, ERP, accounting system, or file system record
- Post or propose OCI journal entries or hedge designation documentation for filing
- Accept live derivative contract terms with counterparty details, live market rates for hedging decisions, bank or broker credentials, ISDA master agreement data, or employee/customer-identifying information
- Make final hedge accounting determinations, render compliance opinions, or confirm that a hedging relationship qualifies for hedge accounting
- Store, relay, or log financial data beyond the current session
- Access authenticated financial databases, ERP integrations, or derivative pricing systems
- Form an accountant-client relationship
