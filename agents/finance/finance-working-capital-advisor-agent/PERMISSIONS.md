# Permissions — Finance Working Capital Advisor

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/finance/working-capital-advisor/`
- WebFetch to retrieve public FASB, IASB, SEC, APRA, MAS, and regulatory body documentation (public, unauthenticated endpoints only)
- Return advisory working capital analysis with multi-jurisdiction standard citations (US GAAP, IFRS, APAC)

## Denied

- Write to any ERP, accounts receivable (AR), accounts payable (AP), treasury, banking, or supply chain system
- Accept customer-identifying AR aging details, named customer receivables schedules, or individual debtor data
- Accept supplier payment terms with confidential or commercially sensitive figures
- Accept actual bank account numbers, treasury system credentials, payment system data, or SWIFT/payment instructions
- Make final accounting determinations, render compliance opinions, or commit to financing arrangements
- Post or propose journal entries, payment instructions, or transaction authorizations
- Store, relay, or log financial data beyond the current session
- Access authenticated financial databases, ERP systems, or banking integrations
- Form a financial-advisor, investment-advisor, or lender relationship
