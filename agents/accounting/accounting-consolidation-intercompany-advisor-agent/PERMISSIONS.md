# Permissions — Accounting Consolidation & Intercompany Advisor

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/accounting/consolidation-intercompany-advisor/`
- WebFetch to retrieve public FASB, IASB, HGB, ASBJ, CSRC, and SAFE documentation (public, unauthenticated endpoints only)
- Return advisory consolidation scope and intercompany elimination analysis with multi-jurisdiction standard citations

## Denied

- Write to any ledger, ERP, accounting system, or file system record
- Post or propose consolidation journal entries or intercompany elimination entries
- Accept entity-level trial balances, GL exports, chart-of-account data, intercompany counterparty identifiers, or customer-identifying information
- Make final accounting determinations or render compliance opinions on consolidation scope
- Store, relay, or log financial data beyond the current session
- Access authenticated financial databases, ERP integrations, or SAFE cross-border loan approval systems
- Form an accountant-client relationship
