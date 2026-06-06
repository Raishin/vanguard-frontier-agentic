# Well-Architected Cost Optimization Review Guide

Use this reference for AWS Well-Architected Framework Cost Optimization Pillar reviews. In this repository, `aws-waf-cost-optimization-review` means Well-Architected Framework review, not AWS Web Application Firewall configuration.

## What people get wrong

The lazy story is:

> Cost optimization means find idle resources and buy commitments.

Wrong. Good cost review preserves business outcomes. Savings that degrade reliability, security, observability, or delivery speed are not automatically good recommendations.

Common bad assumptions:

- Current month spend alone proves waste.
- Low utilization means safe rightsizing.
- Reserved Instances or Savings Plans are safe if the discount is large.
- Untagged resources are minor accounting issues.
- Spot is always a cost win.
- Deleting logs, backups, or redundancy is acceptable savings.

## Cost-specific failure modes

- No owner, cost allocation tags, unit-cost metric, or budget accountability.
- Recommendations ignore seasonality, migration, incident recovery, test load, or business growth.
- Commitment purchase locks in wrong family/Region/account or hides architecture change plans.
- Rightsizing ignores burst, memory, I/O, latency, failover, or compliance requirements.
- Observability, backup, security, or resilience costs are cut without risk acceptance.
- Savings estimate uses list pricing or aggregates without amortized/unblended context.

## Minimum safe workflow

1. Confirm workload, payer/member account scope, monthly spend, business owner, unit economics, and cost target.
2. Break down cost by service, account, Region, usage type, operation, tag, and resource where supported.
3. Evaluate governance: tagging, budgets, anomaly detection, chargeback/showback, and owner accountability.
4. Separate quick wins from structural optimization, commitment strategy, and risky workload changes.
5. Estimate savings with assumptions, confidence, operational risk, and pillar tradeoffs.
6. Require owner approval before deletion, downscaling, commitment purchase, retention reduction, or architecture change.
7. Track recommendations as experiments with rollback criteria and post-change cost validation.

## Verification targets

- Cost Explorer/Billing data grouped by service, account, Region, usage type, operation, tag, and resource where available
- budget, anomaly detection, cost category, and tag coverage evidence
- utilization metrics, performance headroom, workload seasonality, and business-critical periods
- Savings Plans/RI coverage, utilization, expiration, and purchase constraints
- idle/underused resources with owner and purpose confirmation
- reliability/security/observability impact for any proposed savings action

## When to push back

Push back if the user asks to:

- recommend commitments without usage stability and roadmap evidence
- delete backups/logs/redundancy for savings without risk acceptance
- rights-size production from average utilization only
- blame teams from untagged or payer-level aggregates
- ignore security or reliability tradeoffs
- confuse this Well-Architected Framework review with AWS Web Application Firewall cost tuning
