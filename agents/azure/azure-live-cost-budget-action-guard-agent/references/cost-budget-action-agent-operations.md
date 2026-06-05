# Azure Live Cost Budget Action Guard Agent Operations

> Version note: Azure Cost Management, budget alert behavior, quota capabilities, SKU pricing, and cost-data latency change. Verify exact behavior against Microsoft Learn documentation through the user's configured documentation MCP and any sampled configured-environment evidence before production use. Do not paste credentials, tokens, tenant identifiers, subscription identifiers, billing account identifiers, invoices, customer data, private offers, or unsanitized cost exports into prompts, commands, or reference examples.

## What people get wrong

- Raising budgets or quotas because a deployment is blocked, without proving business approval and forecasted cost.
- Treating budget alerts as spend prevention; Microsoft Learn documents them as notifications, not consumption stops.
- Ignoring cost-data latency: budget evaluations rely on cost data that can lag, and alerts are evaluated periodically.
- Assuming quota increases guarantee capacity; quotas and capacity are separate operational facts.
- Using list price or stale estimates for high-cost compute without region, reservation, commitment, idle, and teardown context.

## Officially grounded service shape

- Azure budgets help plan spending and accountability; thresholds can alert on actual or forecasted cost.
- Budget notifications are triggered when configured thresholds are met; resources are not stopped by budget alerts alone.
- Cost and usage data can be delayed, budgets are evaluated periodically, and forecasted alerts warn about projected overspend.
- Cost alerts include budget, credit, and department spending quota alerts depending on offer type.
- Quota monitoring tracks usage approaching allocated limits, but quota is not the same as real capacity.
- GPU workloads can produce high idle and overprovisioning cost; cost visibility and workload right-sizing are required.

That is the key insight:

> The agent is a financial-risk gate. It must prove budget scope, actual/forecast evidence, quota/capacity distinction, approval authority, and rollback or teardown before allowing cost-impacting changes.

## Non-negotiable design rules

### 1. Never approve a budget, quota, or high-cost SKU change without spend impact, approval authority, monitoring, and rollback or teardown evidence.

### 2. Treat budget alerts as notification controls, not hard stops.

### 3. Block changes when cost data is stale, budget scope is ambiguous, or the approving authority is not explicit.

### 4. Prefer read-only budget, alert, quota, cost analysis, pricing, and resource inventory evidence before mutation.

### 5. Label configured-environment observations as sampled and bounded to the scope, region, service, and time window.

## Minimal safe implementation flow

- Confirm cost scope, service, region, SKU family, requested action, budget owner, approval state, and teardown/rollback owner.
- Ground budget, alert, and quota behavior in Microsoft Learn Cost Management and Quotas guidance.
- Collect read-only evidence for current budget thresholds, actual and forecasted spend, cost alerts, quota usage, SKU count, idle resources, commitment coverage, and teardown controls.
- Decide: monitor, deny, lower, raise, scale, request quota, or block; if action is live, require explicit human approval.
- Verify post-action alert coverage, budget threshold, quota state, resource count, and open financial risks.

## High-risk assumptions to kill

- A budget increase is harmless because alerts exist.
- A quota increase means capacity is available.
- Cost data is real-time.
- GPU or HPC idle capacity is acceptable without teardown policy.
- Documentation proves this account's cost, quota, capacity, or approval posture.

## Safe command/code verification targets

Verify against current docs and safe local or read-only tooling before use:

- Cost scope, budget name, reset period, thresholds, alert recipients or action groups, and actual versus forecasted spend.
- Cost-data freshness, offer/account support, currency constraints, and budget filter behavior.
- Quota usage, quota alerting, requested limit change, service/region/SKU family, and capacity caveat.
- Existing high-cost resources, idle capacity, scheduled teardown, reservations/commitments, and owner justification.
- Approval record, spend cap, rollback/teardown action, and post-change monitoring evidence.

## When to push back

- The cost scope, SKU, region, approval authority, or rollback owner is ambiguous.
- The user treats budget alerts as an enforcement mechanism.
- The user wants to paste invoices, billing identifiers, exports with customer data, or raw environment dumps.
- The requested action would materially increase spend without forecast, quota, monitoring, and explicit approval evidence.
