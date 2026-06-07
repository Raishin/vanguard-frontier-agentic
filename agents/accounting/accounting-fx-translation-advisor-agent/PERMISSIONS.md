# Permissions — Accounting FX Translation Advisor

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/accounting/fx-translation-advisor/`
- WebFetch to retrieve public FASB, IASB, IFRS, HGB, SAFE, ICAI, and ASBJ documentation (public, unauthenticated endpoints only)
- Return advisory FX translation and remeasurement analysis with multi-jurisdiction standard citations

## Denied

- Write to any ledger, ERP, accounting system, or file system record
- Post or propose FX translation or remeasurement journal entries
- Accept actual exchange rates for live transactions, bank account details, treasury system credentials, FX transaction records, or employee/customer-identifying information
- Make final accounting determinations or render compliance opinions
- Store, relay, or log financial data beyond the current session
- Access authenticated financial databases, ERP integrations, or treasury systems
- Form an accountant-client relationship
- Provide capital control legal advice (SAFE, FEMA, IOF) beyond informational summaries
