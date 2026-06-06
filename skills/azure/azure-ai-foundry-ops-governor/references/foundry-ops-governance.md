# Microsoft Foundry operations governance

## What people get wrong

- They treat a Foundry project as the whole security boundary. It is only a development boundary; connected services still need their own RBAC, networking, logging, and lifecycle controls.
- They assign broad Azure roles because Foundry-specific roles feel new. That expands blast radius across model deployment, project management, and access assignment.
- They discuss model rollout without proving regional feature support, deployment type, and quota. Documentation is not capacity evidence.
- They enable tool or agent workflows before deciding which identity is allowed to read, write, delete, or assign access.
- They design private access to Foundry but forget Storage, Key Vault, AI Search, registry, logging sinks, and outbound dependencies.

## Officially grounded service shape

Microsoft Learn describes a layered Foundry model: a top-level Foundry resource for governance, networking, security, monitoring, and deployments; projects for team development isolation; and connected Azure resources such as Storage, Key Vault, and AI Search under separate governance boundaries. RBAC is scope-sensitive and uses Foundry-specific built-in roles. Some role names were renamed, so automation should prefer stable role definition IDs where possible.

## Non-negotiable design rules

1. Identify whether each operation is resource-scoped, project-scoped, or connected-resource-scoped.
2. Use least-privilege Foundry roles first; justify any Owner, Contributor, or custom role action.
3. Prefer Microsoft Entra ID and managed identity. Treat keys as full-access secrets unless proven otherwise.
4. Verify regional availability and quota for the exact deployment type before approving rollout.
5. Verify network isolation for Foundry and every connected dependency.
6. Require diagnostic settings or equivalent monitoring for operations, model usage, errors, and access changes.
7. Put mutation approval behind blast-radius, rollback, and evidence review.

## Minimal safe implementation flow

1. Inventory intended teams, projects, model deployments, agent/tool usage, and connected resources.
2. Map every persona and managed identity to the smallest Foundry and connected-resource scope.
3. Validate target regions, features, deployment types, and quotas against Microsoft Learn and sampled configured-environment evidence if available.
4. Validate private access and outbound paths for Foundry plus Storage, Key Vault, AI Search, registry, and monitoring sinks.
5. Confirm diagnostics, retention, alerting, and access-review ownership.
6. Run a preproduction rollout with the same role, network, and quota shape as production.
7. Approve production only with a rollback plan for access, networking, deployments, and tool registration changes.

## High-risk assumptions to kill

- A project boundary is not enough for production isolation unless connected resources, model deployments, networking, diagnostics, and RBAC are separately verified.
- A documented Foundry capability is not proof that the target region, deployment type, quota, or project scope can support the workload.
- Managed identity only reduces key exposure if each identity has narrowly scoped permissions on Foundry and every connected resource.
- Private access to the Foundry resource does not automatically make Storage, Key Vault, AI Search, registry, monitoring, or agent outbound paths private.
- Content safety or guardrail configuration is not production readiness unless intervention points, model deployment scope, logging, and operational ownership are defined.

## Safe command/code verification targets

- Inspect infrastructure definitions for `Microsoft.CognitiveServices/accounts`, `Microsoft.CognitiveServices/accounts/projects`, deployments, private endpoint, diagnostic setting, and role-assignment resources.
- Verify automation distinguishes resource-scope and project-scope operations before creating deployments, projects, agents, evaluations, or connections.
- Check policy-as-code or templates for private DNS, Key Vault, Storage, AI Search, and log destination wiring rather than only the Foundry resource.
- Review scripts for secret material, key-based auth, broad `Owner` or `Contributor` grants, and unguarded delete/update actions.
- Confirm generated evidence labels separate Microsoft Learn documentation from sampled configured-environment evidence.

## Safe verification targets

- Role assignments at Foundry resource and project scopes.
- Managed identity assignments on project and connected resources.
- Model deployment type, region, capacity, and quota.
- Public network access, private endpoint, DNS, and outbound dependency paths.
- Diagnostic settings and log destinations.
- Guardrail and tool-registration configuration where applicable.

## When to push back

Push back on production approval from diagrams only, broad role grants, project-only isolation claims, region-agnostic quota claims, key-based automation, or tool operations without an approval and rollback boundary.
