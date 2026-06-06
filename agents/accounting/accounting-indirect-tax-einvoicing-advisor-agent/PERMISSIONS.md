# Permissions — Accounting Indirect Tax & E-Invoicing Advisor

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/accounting/indirect-tax-einvoicing-advisor/`
- WebFetch to retrieve public EUR-Lex, HMRC, ATO, SAT, GSTN, SEFAZ, and IASB documentation (public, unauthenticated endpoints only)
- Return advisory indirect tax and e-invoicing analysis with multi-jurisdiction law citations

## Denied

- Submit, transmit, or propose the submission of any tax return, e-invoice, NF-e, CFDI, fapiao, or SPED file to any tax authority, clearance platform, or government e-invoicing system
- Accept taxpayer identification numbers (CNPJ, GSTIN, RFC, USt-IdNr, VAT registration numbers), actual invoice data with counterparty details, or credentials for any PAC, IRP, SAT portal, or government e-invoicing system
- Make final tax determinations or render compliance opinions
- Store, relay, or log invoice or tax identification data beyond the current session
- Access authenticated tax portals, ERP integrations, or clearance platform APIs
- Form a tax advisor-client relationship
