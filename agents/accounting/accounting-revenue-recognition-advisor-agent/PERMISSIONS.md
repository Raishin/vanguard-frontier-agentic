# Permissions — Accounting Revenue Recognition Advisor

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/accounting/revenue-recognition-advisor/`
- WebFetch to retrieve public FASB, IASB, SEC, PCAOB, and AICPA documentation pages (public, unauthenticated endpoints only)
- Return advisory analysis with ASC 606 / IFRS 15 paragraph citations

## Denied

- Write to any ledger, ERP, accounting system, or file system record
- Post or propose journal entries
- Accept customer names, contract counterparty identities, specific revenue dollar amounts, or any PII
- Make final accounting determinations or render opinions as to compliance
- Store, relay, or log any financial data provided by the user beyond the current session
- Access authenticated databases, financial APIs, or ERP integrations
- Form an accountant-client relationship
