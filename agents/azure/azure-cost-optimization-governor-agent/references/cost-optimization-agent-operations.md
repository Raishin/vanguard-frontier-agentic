# Azure Cost Optimization Governor Agent Operations

> Version note: Azure services, pricing, identity, policy, and governance features change. Verify exact behavior against Microsoft Learn documentation through the user's configured documentation MCP and any sampled configured-environment evidence before production use. Do not paste secrets, identifiers, or customer data into prompts, commands, or reference examples.

## What people get wrong

- Promising savings from Advisor or workbooks without validating workload ownership, utilization history, and operational impact.
- Treating budgets as hard enforcement instead of alerting and automation triggers.
- Buying commitments before separating stable baseline usage from bursty, seasonal, dev/test, or soon-to-be-retired usage.
- Publishing cost exports without data classification, storage firewall, retention, access control, and finance-owner review.

## Officially grounded service shape

- Cost Management is an organizational FinOps practice across planning, visibility, accountability, optimization, and iteration.
- Budgets support thresholds and alerts; they can trigger automated actions but do not inherently prevent every cost event.
- Advisor cost recommendations can identify idle, underutilized, reservation, and savings-plan opportunities, but the workbook guidance does not guarantee cost reduction.
- Exports automate cost datasets to storage and can include large datasets, FOCUS format, partitioning, historical reruns, and storage firewall considerations.
- Savings plans and reservations provide billing discounts and do not change runtime state; misuse can lock in waste.

That is the key insight:

> The agent is not a checklist runner. It is an evidence-bound reviewer that separates documented Azure behavior from the user's unproven environment state.

## Non-negotiable design rules

### 1. Do not claim savings until utilization, ownership, dependency, and operational-impact evidence supports the action.

### 2. Pair every recommendation with owner, due date, rollback/undo path, and measurement method.

### 3. Treat billing exports as sensitive operational data with access, retention, and storage controls.

### 4. Distinguish rate optimization from usage optimization and governance guardrails.

### 5. Require alert recipient ownership and stale-recipient review for budgets, anomalies, and commitment utilization.

## Minimal safe implementation flow

- Classify the ask: visibility, accountability, waste removal, rate optimization, budget/alerting, or export/reporting.
- Ground the method in Microsoft Learn Cost Management, Advisor, budget, export, and Well-Architected docs.
- Use sampled cost, usage, tag, Advisor, budget, and export evidence when available.
- Return top actions by evidence strength, blast radius, confidence, and owner readiness.

## High-risk assumptions to kill

- Every Advisor recommendation should be applied immediately.
- A budget alert is a spend control.
- Reservation or savings-plan purchase is safe without stable baseline usage.
- Cost exports are harmless because they are not credentials.
- Unowned resources can be deleted just because they look idle.

## Safe command/code verification targets

Verify against current docs and safe local or read-only tooling before use:

- Budgets, alert thresholds, recipients, anomaly alerts, and action groups.
- Cost analysis scope, tags, management groups, allocation dimensions, and shared-cost rules.
- Advisor recommendation age, lookback window, utilization signal, dependency owner, and rollback.
- Export dataset, cadence, storage firewall, schema version, retention, and downstream consumers.

## When to push back

- The user wants to delete or downsize resources without owner and dependency evidence.
- The user wants to buy commitments based on one short observation window.
- The export target exposes billing data without access controls.
