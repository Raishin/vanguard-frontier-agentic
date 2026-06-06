# Permissions — Accounting Tax Provision Advisor

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/accounting/tax-provision-advisor/`
- WebFetch to retrieve public FASB, IASB, OECD, ASBJ, ICAI, HGB, and CAS documentation (public, unauthenticated endpoints only)
- Return advisory income tax provision analysis with multi-jurisdiction standard citations

## Denied

- Write to any ledger, ERP, tax system, or file system record
- Post or propose journal entries
- Accept raw tax returns, trial balance exports, taxpayer-identifying numbers (EIN, TIN, CRN), employee wage data, or customer-identifying information
- Make final accounting determinations, render tax opinions, or issue compliance conclusions
- Store, relay, or log financial or tax data beyond the current session
- Access authenticated tax databases, ERP integrations, or government tax portals
- Form an accountant-client relationship or a tax advisor-client relationship
