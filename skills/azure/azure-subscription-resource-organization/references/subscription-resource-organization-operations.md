# Azure Subscription Resource Organization Operations

> Version note: Azure service behavior and tooling change over time. Verify exact command syntax, permissions, and feature availability against Microsoft Learn documentation through the user's configured documentation MCP before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Using resource groups as the main isolation boundary for production environments.
- Building a flat subscription estate then hoping tags fix governance later.
- Moving subscriptions without understanding inherited policy and RBAC impact.
- Mixing platform shared services and workload resources without owner boundaries.
- Ignoring quotas, regions, naming, tagging, and lifecycle when designing hierarchy.

## Officially grounded service shape

- Microsoft Learn evidence says Azure resources can be organized at management group, subscription, resource group, and resource levels.
- Resource organization decisions are foundational for naming, tagging, subscription design, and management group design.
- Management groups manage policies, access, and compliance across subscriptions at scale; subscriptions act as policy and management boundaries and scale units.
- Landing-zone guidance separates platform/management deployments from workload or application resources and recommends planning for scale, multiple regions, governance, and ownership.

Documentation evidence proves documented Azure service behavior. It does not prove the user's tenant, subscription, RBAC, quotas, deployed resources, billing state, security posture, or production readiness.

## Non-negotiable design rules

- Choose hierarchy from governance, ownership, compliance, network, cost, and scale requirements, not aesthetics.
- Use subscriptions as isolation, policy, management, and scale boundaries where environment or workload separation is needed.
- Use resource groups for shared lifecycle, not as a substitute for subscription isolation.
- Document policy/RBAC inheritance and exemption strategy before moving subscriptions.
- Require naming, tagging, budget, and owner model for every proposed boundary.

## Minimal safe implementation flow

- Scope organization goal, current hierarchy, workloads, environments, regions, owners, compliance, and platform services.
- Map management groups, subscriptions, resource groups, policies, RBAC, budgets, tags, and quotas.
- Identify weak boundaries, inheritance surprises, and migration risks.
- Propose minimal target-state changes with staged moves and rollback/communication plan.
- Return target hierarchy, rationale, blockers, verification checks, and governance debt.

## High-risk assumptions to kill

- Tags can replace hierarchy; tags help reporting and policy targeting, but they are not first-class isolation boundaries.
- Resource groups can isolate production from nonproduction; they share subscription-level policy, quota, billing, and many blast-radius concerns.
- A flat hierarchy is simpler; it usually defers governance debt until policy, RBAC, and compliance inheritance become painful.
- Subscription moves are administrative cleanup; they can change inherited policy, RBAC, budgets, compliance reporting, and operational ownership.
- A landing-zone diagram proves suitability; current quotas, regions, ownership, and workload lifecycle still need evidence.

## Safe command/code verification targets

- Inventory management groups, subscriptions, resource groups, inherited policies, role assignments, tags, budgets, and quotas with read-only queries.
- Compare proposed boundaries to workload lifecycle, owner, environment, compliance, network, and cost-accountability requirements.
- Check management-group depth, parentage, and subscription placement constraints before proposing hierarchy moves.
- Validate naming and tagging standards against Microsoft Learn guidance and existing policy enforcement.
- Label hierarchy inventory as sampled current-state evidence; documentation alone does not prove the estate is organized correctly.

## Safe verification targets

- Management group purpose and inherited controls are documented.
- Subscription placement matches workload/environment/platform boundary.
- Resource groups contain resources with shared lifecycle and ownership.
- Policy, RBAC, budget, tag, naming, and quota implications are known.
- Subscription moves or hierarchy changes have validation and rollback/communication plan.

## When to push back

- The user wants one subscription or flat hierarchy for convenience.
- A resource group is used to isolate production from non-production without policy/RBAC proof.
- The proposed move ignores inherited deny policies or RBAC.
- Ownership and cost accountability are undefined.
