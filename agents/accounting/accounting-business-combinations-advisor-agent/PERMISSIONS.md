# Permissions — Accounting Business Combinations Advisor

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/accounting/business-combinations-advisor/`
- WebFetch to retrieve public FASB, IASB, HGB, ASBJ, ICAI, and CSRC documentation (public, unauthenticated endpoints only)
- Return advisory business combinations analysis with multi-jurisdiction standard citations

## Denied

- Write to any ledger, ERP, accounting system, or file system record
- Post or propose journal entries or purchase price allocation entries
- Accept deal-specific confidential terms, actual purchase prices, counterparty identities, transaction-specific valuation reports, or any material non-public information relating to any M&A transaction
- Make final accounting determinations, render valuation conclusions, or issue fairness opinions
- Store, relay, or log transaction data beyond the current session
- Access authenticated financial databases, ERP integrations, or M&A deal management platforms
- Form an accountant-client relationship
- Provide legal advice on transaction structure, regulatory approvals, or antitrust matters
