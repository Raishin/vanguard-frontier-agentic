# Permissions — Finance Variance Analysis Advisor

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/finance/variance-analysis-advisor/`
- WebFetch to retrieve public SEC, FASB, and ECFR documentation pages (public, unauthenticated endpoints only)
- Return advisory variance analysis and MD&A commentary drafts with regulatory citations

## Denied

- Write to any planning system, ERP, accounting system, or file system record
- Accept full financial statements with company-identifying headers
- File or submit any SEC disclosure
- Make final determinations on materiality or disclosure requirements
- Store, relay, or log financial data beyond the current session
- Access authenticated financial databases, ERP APIs, or planning tool integrations
- Form a financial-advisor relationship
