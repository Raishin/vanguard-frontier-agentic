# Cost Anomaly Triage Guide

Use this reference when reviewing AWS spend spikes, Cost Anomaly Detection alerts, Budgets variance, Cost Explorer deltas, usage surges, commitment mismatch, or tagging gaps.

## What people get wrong

The lazy story is:

> Find the expensive service and tell someone to turn it off.

Wrong. Cost anomalies need evidence, attribution, business context, and safe containment. Stopping workloads from a FinOps watch role can become an outage.

Common bad assumptions:

- The largest service delta is the root cause.
- Cost Explorer data is real-time enough for incident response.
- Budget alerts prove waste.
- Anomaly root-cause fields prove ownership.
- Tag gaps are minor reporting issues.
- Savings Plans or RI changes are safe short-term fixes.

## Cost-specific failure modes

- Spend increase is legitimate business demand, data backfill, migration, or incident mitigation.
- Usage type, operation, linked account, Region, or tag dimension points to a different owner than the service name suggests.
- Data transfer, NAT Gateway, CloudWatch Logs, KMS requests, Bedrock tokens, or support charges are missed because the review stops at top-level service.
- Forecast/budget period does not align with anomaly window.
- Commitment coverage/utilization is interpreted without amortized vs unblended cost context.
- Proposed containment would stop production, delete data, or break compliance evidence.

## Minimum safe workflow

1. Define anomaly window, baseline period, account scope, payer/member visibility, and data freshness.
2. Break down spend by service, linked account, Region, usage type, operation, tag, and resource where available.
3. Separate confirmed cost drivers from hypotheses and missing dimensions.
4. Check business/change context: releases, migrations, tests, incidents, backfills, traffic growth, or new services.
5. Recommend non-destructive containment first: owner escalation, budget/anomaly subscription tuning, log retention review, quota check, tagging correction, or deeper domain review.
6. Escalate destructive containment only to the correct service owner with approval and rollback/impact notes.
7. State uncertainty caused by billing latency, missing tags, unsupported resource granularity, or limited account visibility.

## Verification targets

- Cost Anomaly Detection monitor, alert, root-cause dimensions, and subscription recipients
- Cost Explorer grouped by service, linked account, Region, usage type, operation, tag, and resource where supported
- Budgets actual/forecast threshold, time period, alert recipients, and action settings
- pricing/usage context for the suspected service
- recent deployments, batch jobs, migrations, incidents, or traffic changes
- tag coverage, cost category mapping, account owner, and commitment coverage/utilization
- safe follow-up owner and impact assessment before any workload change

## When to push back

Push back if the user asks to:

- stop, delete, downscale, or revoke resources from cost evidence alone
- buy or modify commitments as an emergency reaction
- blame an owner from incomplete tags or payer-only aggregates
- ignore billing latency or missing account scope
- publish customer/resource-sensitive billing details broadly
- treat a cost anomaly as waste without business validation
