# Safety checklist for Azure Cost Estimation Review

## Non-negotiable gates

- Never ask for billing account identifiers, enrollment numbers, invoices, tenant identifiers, subscription identifiers, customer names, or screenshots containing sensitive billing data.
- Do not claim exact future spend from calculator or retail API data.
- Do not apply savings plans, reservations, or hybrid benefits unless utilization stability and eligibility are proven.
- Do not ignore data transfer, support, logs, backups, monitoring, security services, private networking, and DR replicas.
- Require explicit uncertainty labels for usage growth, regional price changes, negotiated rates, commitment coverage, and utilization.

## High-risk assumptions to kill

- "The calculator total is the budget." It is an estimate from configuration and usage assumptions.
- "Retail price is our price." Negotiated and agreement-specific rates may differ.
- "Reservations always save money." They can waste money if usage, region, or SKU changes.
- "Nonproduction costs are negligible." Logs, databases, NAT, endpoints, and always-on plans can dominate.
- "One architecture scenario is enough." Estimates need baseline, peak, DR, and growth cases.

## Evidence labels

- `docs_only`: pricing/estimation guidance only.
- `estimate_review`: calculator export or architecture estimate reviewed.
- `usage_informed`: historical usage or measured load informed the estimate.
- `rate_verified`: negotiated or agreement-specific rate evidence was safely verified.
- `commitment_ready`: utilization stability and commitment coverage are proven enough to consider reservations or savings plans.

## Minimum safe evidence

- Architecture bill of materials, regions, SKUs, tiers, quantities, hours, storage, data transfer, logs, backup, and support assumptions.
- Environment split: production, staging, dev/test, DR, and shared services.
- Workload pattern: baseline, peak, growth, seasonality, and shutdown assumptions.
- Pricing basis: retail, calculator, price sheet, negotiated, or unknown.
- Commitment assumptions: reservation, savings plan, hybrid benefit, term, scope, and utilization proof.
