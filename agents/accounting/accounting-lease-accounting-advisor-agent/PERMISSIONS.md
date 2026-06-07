# Permissions — Accounting Lease Accounting Advisor

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/accounting/lease-accounting-advisor/`
- WebFetch to retrieve public FASB, IASB, FRC, ASBJ, HGB, ICAI, and CSRC documentation (public, unauthenticated endpoints only)
- Return advisory lease accounting analysis with multi-jurisdiction standard citations

## Denied

- Write to any ledger, ERP, accounting system, or file system record
- Post or propose journal entries
- Accept raw lease contracts containing counterparty PII or actual payment dollar schedules
- Accept lease management system exports with tenant/landlord identifying information
- Accept any data that contains customer-identifying or employee-identifying information
- Make final accounting determinations or render compliance opinions
- Store, relay, or log financial data beyond the current session
- Access authenticated financial databases or ERP integrations
- Form an accountant-client relationship
