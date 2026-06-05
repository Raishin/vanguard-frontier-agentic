# Azure WAF Cost Optimization Operations

> Version note: Azure service behavior and tooling change over time. Verify exact command syntax, permissions, and feature availability against Microsoft Learn documentation through the user's configured documentation MCP before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Starting with reservations or savings plans before rightsizing and shutdown analysis.
- Treating Advisor savings as guaranteed savings.
- Ignoring reliability, security, observability, backup, and data-transfer costs in the cost model.
- Deleting idle-looking resources without owner and dependency confirmation.
- Optimizing unit price while breaking SLOs or compliance requirements.

## Officially grounded service shape

- Microsoft Learn evidence says a cost model estimates initial cost, run rates, and ongoing costs; it is foundational for forecasting and budget planning.
- Cost modeling must account for workload components, requirements, supporting services, billing model, licensing, reliability costs, operational costs, business metrics, budgets, forecasts, and model maintenance.
- Microsoft Cost Management, Azure Advisor, reservations, savings plans, Azure Hybrid Benefit, budgets, forecasts, pricing calculator, and tag inheritance are Azure facilitation mechanisms.
- Advisor recommendations should be sequenced: rightsizing/shutdown first, then reservations, then savings plans; forecasted savings can vary and are not guaranteed.

Documentation evidence proves documented Azure service behavior. It does not prove the user's tenant, subscription, RBAC, quotas, deployed resources, billing state, security posture, or production readiness.

## Non-negotiable design rules

- Build or validate the cost model before optimization claims.
- Separate usage optimization from rate optimization.
- Confirm owner, dependency, and business criticality before deleting or shutting down resources.
- Rightsize and remove waste before buying commitments.
- Label savings as forecast, estimate, or sampled evidence, not guaranteed outcome.

## Minimal safe implementation flow

- Scope workload, business metric, billing boundary, time window, environments, and optimization goal.
- Collect cost model, budgets, Advisor recommendations, utilization, tags, commitments, licensing, and owner evidence.
- Classify opportunities by idle waste, rightsizing, storage lifecycle, data transfer, reservations, savings plans, hybrid benefit, and architecture tradeoffs.
- Prioritize reversible changes and quantify forecast confidence.
- Return savings candidates, risk, approval requirements, verification checks, and follow-up measurements.

## High-risk assumptions to kill

- Lowest cost is the goal; Well-Architected cost optimization requires business-aligned tradeoffs, not reckless cheapness.
- Advisor savings are bankable; Microsoft positions them as recommendations and forecast opportunities, not guaranteed outcomes.
- Reservations or savings plans fix waste; commitments can lock in bad sizing if rightsizing and shutdown analysis happen later.
- Idle-looking resources are safe to delete; owner, dependency, recovery, compliance, and business-calendar evidence must come first.
- Cost reports prove optimization; they need utilization, commitment, tagging, and workload-criticality context.

## Safe command/code verification targets

- Pull read-only Cost Management, budget, Advisor, tag, and utilization evidence for the scoped workload and time window.
- Separate usage cleanup candidates from rate optimization candidates before proposing commitments.
- Validate each deletion, shutdown, SKU change, or scaling change against owner confirmation, dependency mapping, and rollback.
- Check whether reliability, security, backup, logging, data transfer, and support costs are included in the cost model.
- Label savings as estimate, forecast, sampled evidence, or unverified; never call them guaranteed.

## Safe verification targets

- Cost model includes direct, supporting, reliability, operational, licensing, and data transfer costs.
- Budgets and alerts exist for the workload boundary.
- Advisor or sampled utilization evidence supports rightsizing/shutdown candidates.
- Reservations/savings plans are considered only after usage cleanup.
- Owners approve any resource deletion, commitment purchase, or billing change.

## When to push back

- The user wants to delete resources based only on cost list output.
- Savings are presented as guaranteed without usage and rate evidence.
- A commitment purchase is proposed before rightsizing.
- Cost reduction would violate reliability, security, or compliance requirements.
