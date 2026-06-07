# Permissions — Finance Transfer Pricing & Pillar Two Advisor

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/finance/transfer-pricing-pillar-two-advisor/`
- WebFetch to retrieve public OECD, IRS, HMRC, Bundesfinanzministerium, NTA Japan, SAT China, CBDT India, and FASB/IASB documentation (public, unauthenticated endpoints only)
- Return advisory transfer pricing and Pillar Two analysis with jurisdiction-specific statutory citations

## Denied

- Write to any tax system, ERP, general ledger, or file system record
- Submit or prepare CbCR filings (Form 8975, OECD XML schema submissions, or local CbCR notifications)
- File tax returns or any regulatory submission
- Engage in or simulate competent authority proceedings or APA negotiations
- Accept actual TP documentation (master file, local file), CbCR data files, entity-specific transaction records, deal-specific confidential terms, customer or counterparty identifiers, or any MNPI
- Make final tax determinations, render compliance opinions, or constitute formal tax advice
- Store, relay, or log financial data beyond the current session
- Access authenticated tax databases, ERP integrations, or subscription-based transfer pricing databases
- Form a tax-advisor-client relationship
