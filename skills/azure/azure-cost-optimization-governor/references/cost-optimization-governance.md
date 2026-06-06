# Azure cost optimization governance

## What people get wrong

- They chase potential savings before establishing cost visibility, ownership, and clean allocation.
- They buy reservations or savings plans before right-sizing and removing idle resources.
- They treat budgets as hard controls when they are primarily alerts and automation triggers.
- They add Advisor recommendation totals together even though recommendation datasets can overlap.
- They optimize one subscription while shared services, logs, networking, and commitments sit elsewhere.

## Officially grounded service shape

Microsoft Learn frames Cost Management as an ongoing organizational practice with planning, visibility, accountability, optimization, and iteration. Cost data can be analyzed through Cost Analysis, exports, APIs, budgets, tags, management groups, and dimensions. Advisor and Cost Optimization workbooks expose recommendations, while reservations and savings plans are rate-optimization tools that require stable usage and careful sequencing.

## Non-negotiable design rules

1. Establish cost scope, billing model, owners, and reporting dimensions before optimization.
2. Enforce allocation tags and reporting conventions where they drive accountability.
3. Use budgets and alerts with named response owners.
4. Remove waste and right-size before purchasing commitments.
5. Treat Advisor and workbook recommendations as leads, not orders.
6. Validate actual savings after implementation.
7. Require approval and rollback for any action that changes runtime state or financial commitment.

## Minimal safe implementation flow

1. Map billing/resource hierarchy and cost ownership.
2. Validate Cost Analysis views, exports, budgets, tags, dimensions, and alert recipients.
3. Build a sanitized top-cost and trend view by service, owner, environment, and region.
4. Review idle, oversized, scheduling, retention, and architecture opportunities.
5. Review rate optimization only after baseline usage is clean.
6. Turn recommendations into owner-approved work items with impact, rollback, and measurement.
7. Reconcile realized savings against actual cost data after the billing cycle.

## High-risk assumptions to kill

- Budgets alert; they do not inherently stop spend unless connected automation and ownership exist.
- Advisor recommendations are leads, not guaranteed savings, and recommendation totals can overlap or be operationally unsafe.
- Commitment purchases are dangerous before idle waste, rightsizing, and usage stability are proven.
- Tag coverage is not allocation quality if owners, environments, applications, or inherited tags are inconsistent.
- Cost exports and billing data can contain sensitive business context; do not request raw dumps in chat.

## Safe command/code verification targets

- Inspect budget definitions for scope, thresholds, forecast/actual triggers, recipients, action groups, and named response owner.
- Review export definitions and report pipelines for dataset type, destination, schedule, retention, sanitization, and downstream access controls.
- Check dashboards or queries for top services, owner/environment dimensions, amortized versus actual views, commitment utilization, and anomaly trends.
- Verify optimization tickets include expected impact, operational risk, approval, rollback, and post-billing-cycle measurement.
- Confirm reservation or savings-plan recommendations are backed by stable utilization windows and do not double-count rightsizing or deletion savings.

## Safe verification targets

- Budget thresholds, recipients, action groups, and runbook response.
- Cost exports: actual, amortized, FOCUS, price sheet, reservation, and recommendation datasets where appropriate.
- Top services, anomalous growth, idle resources, underutilized commitments, and log/storage retention.
- Tag coverage and allocation quality by owner/environment/application.
- Advisor recommendations with overlap and feasibility review.
- Before/after actuals for implemented optimizations.

## When to push back

Push back on savings guarantees, blind resource deletion, unowned recommendations, raw billing data requests, immediate commitment purchases, or optimization plans without measurement and rollback.
