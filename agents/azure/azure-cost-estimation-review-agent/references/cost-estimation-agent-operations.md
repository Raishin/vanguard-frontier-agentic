# Azure Cost Estimation Review Agent Operations

> Version note: Azure services, pricing, identity, policy, and governance features change. Verify exact behavior against Microsoft Learn documentation through the user's configured documentation MCP and any sampled configured-environment evidence before production use. Do not paste secrets, identifiers, or customer data into prompts, commands, or reference examples.

## What people get wrong

- Treating calculator output as invoice truth instead of a usage-based estimate.
- Using retail price assumptions when negotiated pricing, enterprise agreement, CSP, benefits, or discounts could materially change the result.
- Forgetting support plans, data transfer, backup retention, monitoring ingestion, log retention, private networking, reservations, savings plans, and operational labor.
- Presenting a single monthly number without confidence range, omitted assumptions, and sensitivity drivers.

## Officially grounded service shape

- The Azure pricing calculator builds estimates from product configuration, consumption assumptions, and pricing-plan choices.
- Calculator per-unit prices originate from the Azure Retail Prices API unless the user signs in and selects eligible agreement pricing.
- Azure planning guidance recommends estimated costs before adding services, then budgets, alerts, cost analysis, and invoice/usage reconciliation after deployment.
- Cost data can be retrieved through exports, Cost Details, Query, Retail Prices, and Price Sheet APIs, subject to scope, billing model, refresh, and rate-limit constraints.

That is the key insight:

> The agent is not a checklist runner. It is an evidence-bound reviewer that separates documented Azure behavior from the user's unproven environment state.

## Non-negotiable design rules

### 1. Separate retail, negotiated, estimated, amortized, forecasted, and invoiced cost language.

### 2. Refuse to present exact savings or invoice predictions without pricing scope and usage evidence.

### 3. List omitted cost drivers before giving a confidence verdict.

### 4. Tie every SKU and region to workload assumptions instead of copying defaults.

### 5. Treat future utilization and discount posture as uncertainty unless evidenced.

## Minimal safe implementation flow

- Classify estimate type: new workload, migration, change to existing workload, or optimization scenario.
- Inventory services, regions, SKUs, hours, data volume, retention, support, licensing, and discount assumptions.
- Ground pricing behavior in Microsoft Learn calculator and Cost Management documentation.
- Return an estimate-risk verdict, missing assumptions, sensitivity drivers, and safe next estimation steps.

## High-risk assumptions to kill

- The calculator total is what the invoice will show.
- One region/SKU default is realistic for production.
- Reserved instances or savings plans always save money.
- Monitoring, storage, egress, backup, and support are negligible.
- Negotiated pricing is known without billing-scope evidence.

## Safe command/code verification targets

Verify against current docs and safe local or read-only tooling before use:

- Billing offer type, currency, agreement-pricing availability, and pricing date.
- Per-service SKU, region, tier, reservation/savings-plan choice, usage quantity, and support plan.
- Non-compute drivers: network egress, storage transactions, backup, logs, alerts, and data processing.
- Sensitivity cases for baseline, peak, growth, and failure/DR modes.

## When to push back

- The user wants a single exact number with missing usage assumptions.
- The estimate excludes known production dependencies.
- The requested recommendation assumes discounts or commitments without utilization evidence.
