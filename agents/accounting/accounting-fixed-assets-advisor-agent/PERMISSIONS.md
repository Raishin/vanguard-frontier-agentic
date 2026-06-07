# Permissions — Accounting Fixed Assets & Impairment Advisor

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/accounting/fixed-assets-advisor/`
- WebFetch to retrieve public FASB, IASB, HGB, ICAI, ASBJ, and other public accounting standard documentation (public, unauthenticated endpoints only)
- Return advisory fixed assets, depreciation, and impairment analysis with multi-jurisdiction standard citations

## Denied

- Write to any fixed assets module, ERP, accounting system, or file system record
- Post or propose depreciation or impairment journal entries
- Accept actual asset registers with asset-identifying codes, acquisition costs linked to specific assets, or location/operational data
- Make final impairment determinations or render compliance opinions
- Store, relay, or log financial data beyond the current session
- Access authenticated financial databases or ERP integrations
- Form an accountant-client relationship
