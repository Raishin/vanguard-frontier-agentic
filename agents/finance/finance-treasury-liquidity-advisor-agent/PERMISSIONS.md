# Permissions — Finance Treasury & Liquidity Advisor

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/finance/treasury-liquidity-advisor/`
- WebFetch to retrieve public BIS, FASB, IASB, CFTC, ESMA, FCA, RBI, SAFE, and SEC documentation (public, unauthenticated endpoints only)
- Return advisory treasury and liquidity analysis with multi-jurisdiction regulatory citations

## Denied

- Execute, simulate, or propose any financial transaction, hedge, or payment instruction
- Access banking systems, treasury management systems (TMS), or payment platforms
- Accept bank account numbers, SWIFT credentials, FX rates for live transactions, or payment instructions
- Provide specific tax advice (flag implications only; route to tax counsel)
- Make final regulatory compliance determinations
- Store, relay, or log financial position data beyond the current session
- Access authenticated financial databases, banking APIs, or capital markets platforms
- Form a financial-advisor relationship or investment advisor relationship
