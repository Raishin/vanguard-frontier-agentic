# Permissions — Accounting Equity Compensation Advisor

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/accounting/equity-compensation-advisor/`
- WebFetch to retrieve public FASB, IASB, SEC (SAB Topic 14), IRS, SEBI, and FSA documentation (public, unauthenticated endpoints only)
- Return advisory equity compensation analysis with multi-jurisdiction standard citations

## Denied

- Write to any ledger, ERP, accounting system, or file system record
- Post or propose journal entries or stock compensation expense entries
- Accept employee grant details with names or IDs, cap table data, actual grant prices, insider trading window schedules, or any material non-public information relating to stock plans
- Make final accounting determinations or render compliance opinions
- Store, relay, or log equity award data beyond the current session
- Access authenticated financial databases, ERP integrations, or equity plan administration platforms
- Form an accountant-client relationship
- Provide legal, tax, or securities advice on equity compensation design or plan structure
