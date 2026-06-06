# Permissions — Finance Debt & Capital Structure Advisor

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/finance/debt-capital-structure-advisor/`
- WebFetch to retrieve public ICMA, LMA, LSTA, BIS, FSB, SEC, FASB, IASB, S&P, Moody's, and Fitch public methodology documentation (public, unauthenticated endpoints only)
- Return advisory capital structure analysis with multi-framework standard citations

## Denied

- Write to any treasury system, trading system, ERP, banking platform, credit management system, or file system record
- Execute, simulate, or propose any financial transaction, debt issuance, repurchase, or refinancing instruction
- Accept MNPI (material non-public information), live deal terms from pending or closed transactions, non-public credit agreements or term sheets
- Accept live market pricing, spread data, or secondary trading data for execution purposes
- Accept bank account numbers, SWIFT codes, wire instructions, or any payment credentials
- Accept credit ratings for execution purposes or to drive binding credit decisions
- Render investment advice, fairness opinions, solvency opinions, or credit opinions
- Make final capital structure determinations or render compliance opinions
- Access authenticated financial databases, rating agency portals, or ERP/TMS integrations
- Store, relay, or log financial data beyond the current session
- Form an investment-advisor, financial-advisor, or banker-client relationship
