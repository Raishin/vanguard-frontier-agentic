# Azure WAF Cost Optimization Review operations

> Version note: refreshed 2026-06-05 from Microsoft Learn documentation through the user's configured documentation MCP. Documentation-based evidence does not prove any user's deployed Azure state. Do not paste secrets, identifiers, billing exports, or customer data into commands or files.

## What people get wrong

A savings percentage without baseline, utilization, and trade-off evidence is finance theater.

## Officially grounded service shape

Microsoft Well-Architected cost guidance emphasizes cost discipline, cost-efficient design, usage optimization, rate optimization, and continuous monitoring. Azure Advisor and the Cost Optimization workbook surface recommendations for idle resources, Reservations, Savings Plans, Hybrid Benefit, compute, storage, and networking. That is the key insight: cost optimization is recurring evidence review, not one cleanup sprint.

## Non-negotiable design rules

### 1. Require cost baseline, owner, time window, and workload scope before accepting savings claims.
### 2. Separate usage optimization from rate optimization.
### 3. Do not recommend reservations or savings plans without utilization and commitment-risk evidence.
### 4. Treat untagged spend as an allocation blocker.
### 5. Do not delete, cancel, resize, or change billing constructs without explicit approval.

## Minimal safe implementation flow

1. Classify workload scope, cost owner, business criticality, and optimization objective.
2. Ground WAF cost behavior in Microsoft Learn.
3. Review sanitized cost trends, Advisor recommendations, tags, budgets, utilization, commitments, and idle inventory if available.
4. Separate quick wins from risk-bearing changes and commitment decisions.
5. Return prioritized savings actions with evidence level and validation targets.

## High-risk assumptions to kill

- Advisor recommendation equals safe action.
- Reservation coverage is always good.
- Rightsizing cannot hurt reliability.
- Untagged shared costs can be allocated later without governance.

## Safe command/code verification targets

- Cost analysis, budgets, alerts, tags, Advisor recommendations, workbook outputs, utilization, and idle-resource cadence.
- Reservations, Savings Plans, Hybrid Benefit, spot tolerance, and commitment utilization evidence.
- Storage lifecycle policy, transition history, and deletion/retention constraints.

## When to push back

- Savings are asserted without baseline or utilization.
- The action can reduce resiliency or performance without owner approval.
- Commitment purchase is proposed for spiky or uncertain usage.
- Billing-impacting identifiers or private discount sheets are requested.
