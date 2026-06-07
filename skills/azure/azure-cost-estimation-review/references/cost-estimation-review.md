# Azure cost estimation review

## What people get wrong

- They treat the Azure pricing calculator as an invoice simulator. It is an estimation tool driven by selected configuration and usage quantities.
- They forget support plans, backups, monitoring data, private endpoints, NAT, egress, DR capacity, and nonproduction environments.
- They use 730-hour defaults for workloads that scale, sleep, burst, or run seasonally.
- They apply reservations or savings plans before proving stable utilization.
- They ignore negotiated pricing and agreement-specific data sources while presenting retail math as exact.

## Officially grounded service shape

Microsoft Learn describes the Azure pricing calculator as a way to turn anticipated usage into an estimated cost. Calculator pricing is sourced from Azure Retail Prices API data, while signed-in experiences can reflect negotiated agreement prices where access allows. Cost Management guidance frames estimation as part of a lifecycle: plan, gain visibility, assign accountability, optimize, and iterate.

## Non-negotiable design rules

1. Build estimates from architecture, not wishful monthly totals.
2. Label pricing basis: retail, negotiated, historical actual, or unknown.
3. Include all environments and shared services.
4. Include data transfer, logs, backups, security, support, and operational overhead where applicable.
5. Use scenarios: baseline, peak, failover/DR, and growth.
6. Treat discounts and commitments as conditional until utilization stability is proven.
7. Preserve uncertainty; do not collapse unknowns into false precision.

## Minimal safe implementation flow

1. Inventory every service, SKU, region, tier, quantity, and usage driver.
2. Map workload patterns: hours, requests, storage growth, retention, egress, and peak factors.
3. Add production, nonproduction, DR, shared, and observability cost lines.
4. Identify pricing source and whether negotiated rates are verified.
5. Create low/expected/peak scenarios and a separate commitment-discount scenario.
6. Compare estimate against budget and identify high-cost architectural decisions.
7. Document unknowns and evidence needed to improve confidence.

## High-risk assumptions to kill

- Calculator output is not invoice truth; it depends on selected configuration, usage quantities, pricing basis, and whether negotiated rates were actually available.
- A monthly total without architecture bill of materials hides the services most likely to blow the budget: logs, backups, bandwidth, DR, support, and shared networking.
- Retail pricing is not agreement pricing, and negotiated pricing should not be claimed unless authenticated evidence or sanctioned price sheets support it.
- Reservation or savings-plan scenarios are not savings unless stable utilization and commitment risk are proven.
- Nonproduction and failover environments are not free unless shutdown, scaling, retention, and capacity assumptions are explicitly modeled.

## Safe command/code verification targets

- Inspect estimate exports, calculator links, or IaC-derived bill-of-materials for named service configurations, regions, SKUs, quantities, and usage drivers.
- Check scripts or spreadsheets for hard-coded 730-hour assumptions, missing egress/log/backup/support lines, and unlabeled retail-versus-negotiated pricing.
- Verify scenarios include low, expected, peak, growth, DR/failover, and commitment-discount variants rather than one fake-precise number.
- Review any API-backed pricing lookup for currency, region, meter, date, and source labeling.
- Require estimate notes to list unknowns, owner decisions, and evidence needed before budget approval.

## Safe verification targets

- Calculator export or estimate link with named configurations.
- SKU, region, and quantity assumptions for each service.
- Usage drivers: hours, transactions, GB/month, retention, egress, replica count, and backup frequency.
- Pricing basis and date of rate evidence.
- Support plan, reservations, savings plans, hybrid benefit, and commitment utilization assumptions.
- Historical actuals if estimating an existing workload.

## When to push back

Push back on one-line monthly totals, retail rates presented as negotiated rates, missing DR/nonproduction/logging costs, discount claims with no utilization evidence, or estimates with no architecture bill of materials.
