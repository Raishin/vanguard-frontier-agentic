# Azure Landing Zone Architecture Operations

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Drawing a management-group tree before clarifying operating model and ownership.
- Treating a landing-zone accelerator deployment as proof that governance is complete.
- Putting every workload under one subscription because it is simpler today.
- Ignoring identity, security, monitoring, backup, policy, and cost baselines in a network-only design.
- Copying hub-spoke without proving routing, DNS, inspection, and shared-service ownership.

## Officially grounded service shape

Microsoft Learn evidence says Azure landing zones are platform foundations spanning platform and application landing zones, management groups, subscriptions, policy inheritance, connectivity, identity, governance, security, management, and platform automation. The design-areas guidance explicitly calls out billing/tenant, identity/access, resource organization, network topology/connectivity, security, management, governance, and platform automation/DevOps as decisions that affect the foundation.

- A landing zone separates platform shared services from application landing zones while preserving governance and autonomy.
- Management-group and subscription design control policy inheritance, RBAC scope, chargeback, quota, and blast radius.
- The platform needs identity/access, governance, management, security, network connectivity, and automation decisions before workload scale.
- Application landing zones need guardrails but must still let workload teams operate within defined boundaries.
- Implementation options are starting points; documented design areas must be tailored to business and technical requirements.

## Non-negotiable design rules

- Start with design areas and decision records, not tooling preference.
- Require explicit ownership for platform subscriptions, application subscriptions, policies, connectivity, monitoring, and incident response.
- Prefer least-privilege and subscription democratization with policy guardrails over central bottlenecks or unmanaged autonomy.
- Document exceptions, inheritance boundaries, and migration path for brownfield environments.
- Never claim production readiness without current-state evidence and operational ownership.

## Minimal safe implementation flow

- Scope business units, environments, regulatory boundaries, connectivity model, identity model, and platform-team responsibilities.
- Map current or proposed management groups, subscriptions, policy inheritance, RBAC, and shared services.
- Evaluate each landing-zone design area for missing decisions and unsafe coupling.
- Rank architecture gaps by blast radius and irreversibility: tenant, management group hierarchy, connectivity, policy, identity, and logging first.
- Return a staged target architecture with blockers, decision points, and safe next actions.

## High-risk assumptions to kill

- A management-group diagram is not a landing zone unless identity, governance, network, security, management, cost, and automation decisions are explicit.
- Deploying an accelerator is not proof that the operating model, ownership, exceptions, and brownfield migration risks are solved.
- Subscription democratization fails if application teams get autonomy without inherited guardrails, monitoring, budget visibility, and recovery boundaries.
- Hub-spoke is not automatically correct; routing, DNS, inspection, private access, shared-service ownership, and failure domains decide fitness.
- Broad platform Owner assignments and shared identities destroy the very blast-radius boundaries landing zones are meant to create.

## Safe command/code verification targets

- Inspect architecture docs or IaC for management group hierarchy, platform/application subscription split, environment isolation, and policy inheritance.
- Review role-assignment patterns for group-based least privilege, PIM/JIT where appropriate, and separation between platform and application responsibilities.
- Check networking artifacts for hub/spoke or alternative topology, DNS, egress inspection, private endpoint strategy, route tables, and connectivity ownership.
- Verify governance, security, management, backup, cost, and diagnostic baselines are assigned at intentional scopes with exception handling.
- Confirm subscription vending or onboarding automation can repeatedly create governed landing zones without hard-coded tenant-specific secrets or one-off manual grants.

## Safe verification targets

- Management group and subscription model supports platform/application separation and environment isolation.
- Policy, RBAC, diagnostic, security, cost, and backup baselines are inherited where intended.
- Connectivity design includes routing, DNS, private access, inspection, and failure domains.
- Subscription vending or onboarding process can produce repeatable, governed application landing zones.
- Brownfield migration steps avoid breaking existing workloads and preserve rollback options.

## When to push back

- The user wants Terraform/Bicep before design decisions are made.
- The architecture depends on broad permanent Owner assignments.
- Shared services have no owner, budget, monitoring, or recovery model.
- A single subscription or network boundary is hiding compliance, quota, or blast-radius problems.
