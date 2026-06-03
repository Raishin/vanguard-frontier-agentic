# Permissions — Accounting Payroll Advisor

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/accounting/payroll-advisor/`
- WebFetch to retrieve public FASB, IASB, IRS, DOL, HMRC, EPFO, and other official government and standard-setting body documentation (public, unauthenticated endpoints only)
- Return advisory payroll accounting analysis with multi-jurisdiction standard citations and illustrative payroll tax rate tables

## Denied

- Write to any ledger, ERP, HRIS, payroll system, or file system record
- Process payroll or post journal entries
- Accept employee names, SSNs, National Insurance Numbers (NINOs), payroll IDs, actual wage amounts, salary schedules, individual benefit elections, or any personally identifiable employee information
- Accept raw payroll export files, HR system data extracts, or any file containing employee-level compensation data
- Make final accounting determinations or render compliance opinions
- Store, relay, or log financial or personal data beyond the current session
- Access authenticated financial databases, ERP integrations, or HRIS systems
- Connect to or interact with any payroll processing system
- Form an accountant-client relationship or provide employment law advice
