# Azure Subscription Resource Organization operations

> Version note: refreshed 2026-06-05 from Microsoft Learn documentation through the user's configured documentation MCP. Documentation-based evidence does not prove any user's deployed Azure state. Do not paste secrets, identifiers, billing exports, or customer data into commands or files.

## What people get wrong

Resource groups are not isolation boundaries for everything. If the governance, policy, cost, network, and ownership boundary is really a subscription or management group, pretending a resource group is enough is self-deception.

## Officially grounded service shape

Microsoft landing-zone guidance treats management groups, subscriptions, resource groups, and resources as distinct organization levels. Management groups scale policy and access across subscriptions; subscriptions are policy, management, isolation, and scale units; resource groups should group resources with shared lifecycle. That is the key insight: organization choices become governance and blast-radius choices.

## Non-negotiable design rules

### 1. Use management groups for scalable policy, compliance, and access boundaries.
### 2. Use subscriptions for environment, workload, isolation, scale, and cost boundaries.
### 3. Use resource groups for shared lifecycle, not as a substitute for environment isolation.
### 4. Require naming, tagging, budget, owner, and policy inheritance evidence.
### 5. Treat subscription moves and hierarchy changes as governance-impacting operations.

## Minimal safe implementation flow

1. Classify operating model, workload type, environment, region, connectivity, and compliance needs.
2. Ground resource organization behavior in Microsoft Learn.
3. Review hierarchy, subscription placement, policy inheritance, RBAC inheritance, naming, tags, budgets, and owners.
4. Identify scale, quota, cost allocation, and blast-radius risks.
5. Return target structure, blockers, migration risks, and safe next actions.

## High-risk assumptions to kill

- One subscription is simpler and therefore better.
- Resource groups provide adequate production isolation.
- Tags can replace policy and ownership.
- Moving subscriptions has no impact on RBAC, policy, or operations.

## Safe command/code verification targets

- Management group hierarchy, subscriptions, resource groups, owners, budgets, policy assignments, and exemptions.
- Environment separation, sandbox rules, production controls, and shared-service boundaries.
- Naming and tagging policy enforcement, resource lifecycle grouping, and quota/region constraints.

## When to push back

- Flat hierarchy hides governance boundaries.
- Production and nonproduction share a subscription without explicit reason.
- Security policy depends on mutable tags at subscription scope.
- Subscription move impact is not assessed.
