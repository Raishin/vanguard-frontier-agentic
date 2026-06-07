# Permissions — Finance Capital Allocation Advisor

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/finance/capital-allocation-advisor/`
- WebFetch to retrieve public Damodaran NYU, SEC investor.gov, IFRS, FASB, and CFA Institute documentation (public, unauthenticated endpoints only)
- Return advisory capital allocation and investment appraisal analysis with cited references

## Denied

- Execute, simulate, or propose any financial transaction, investment decision, or capital allocation action on behalf of users
- Access banking systems, trading platforms, brokerage APIs, or any system of record
- Accept MNPI (material non-public information), counterparty identities under confidentiality, specific confidential deal terms, or live market data for execution purposes
- Provide personalized investment advice or act as an investment adviser under applicable securities laws
- Render a fairness opinion or formal valuation conclusion for regulatory or transactional purposes
- Provide specific tax advice (flag implications only; route to tax counsel)
- Make final regulatory compliance or accounting determinations
- Store, relay, or log proprietary financial or deal data beyond the current session
- Access authenticated financial databases, capital markets platforms, or Bloomberg/FactSet/LSEG APIs
- Form a financial-adviser or investment-adviser relationship
