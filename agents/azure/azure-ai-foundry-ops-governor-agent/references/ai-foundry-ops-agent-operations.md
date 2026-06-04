# Azure AI Foundry Ops Governor Agent Operations

> Version note: Microsoft Foundry and Azure AI Foundry role names, model deployment capabilities, quota surfaces, and private networking requirements change. Verify exact behavior against Microsoft Learn documentation through the user's configured documentation MCP and any sampled configured-environment evidence before production use. Do not paste secrets, identifiers, prompts containing customer data, keys, or environment-specific IDs into prompts, commands, or reference examples.

## What people get wrong

- Treating subscription Owner or Contributor as acceptable day-to-day Foundry access instead of separating project work from account administration.
- Assuming renamed Foundry roles are cosmetic only and then using stale role names in implementation docs, policy, or runbooks.
- Treating key-based authentication as equivalent to Entra ID; Microsoft Learn states keys grant full access without role restrictions.
- Claiming a model, agent, or project rollout is ready without quota, capacity, rate-limit, private networking, managed-identity, and diagnostics evidence.
- Confusing Foundry resource scope, Foundry project scope, model deployment control plane, and runtime invocation paths.

## Officially grounded service shape

- Foundry RBAC has resource and project scopes; project work should use Foundry-scoped built-in roles rather than generic broad Azure roles.
- The minimum project build path starts from Foundry User for both user principal and project managed identity; publishing agents requires stronger project-management permissions.
- Foundry roles were renamed from older Azure AI names; role IDs and core permissions are unchanged, but docs and portals may show old names during rollout.
- Key-based authentication grants full access without role restrictions; Entra ID supports granular RBAC and is the safer default.
- Managed compute and deployment operations have separate read, write, delete, capacity, and usage operations; quota and capacity reads are not proof that a deployment is safe to create.
- Managed private endpoints require the Foundry managed identity to have scoped permission to create or approve private endpoint connections to target resources.

That is the key insight:

> The agent is not a model-deployment cheerleader. It is a least-privilege release governor that must prove identity scope, quota, network path, diagnostics, and rollback before saying Foundry operations are safe.

## Non-negotiable design rules

### 1. Prefer Foundry-scoped roles and Entra ID over generic Owner/Contributor access and key-based runtime access.

### 2. Separate evidence for project build, model deployment management, agent publishing, runtime invocation, and dependent-resource access.

### 3. Treat quota, capacity, and rate-limit evidence as deployment blockers, not post-deployment tuning trivia.

### 4. Do not approve public endpoints, unbounded keys, or unmanaged dependent-service access without explicit risk acceptance.

### 5. Label every live observation as sampled configured-environment evidence and keep documentation-based claims separate.

## Minimal safe implementation flow

- Classify the requested operation: project access, model deployment, managed compute, agent publishing, network setup, diagnostics, or rollback.
- Ground the operation in Microsoft Learn RBAC, authentication, quota, networking, and model-deployment docs.
- Check whether the required identity can perform only the operation needed at the narrowest practical scope.
- Use read-only configured-environment evidence, when available, for role assignments, quotas, deployments, network state, and diagnostics routing.
- Return blockers before any mutation plan; include approval boundary, rollback path, and evidence gaps.

## High-risk assumptions to kill

- Contributor is harmless because the team only needs to deploy a model.
- A visible model in the portal proves quota, capacity, data boundary, or runtime readiness.
- A key is acceptable for routine agent or application runtime because it is easier than Entra ID.
- A private endpoint exists for the Foundry resource, therefore every dependency and egress path is private.
- Read access to deployments means the user can safely create, update, delete, or publish.

## Safe command/code verification targets

Verify against current docs and safe local or read-only tooling before use:

- Foundry resource and project scopes, role IDs, role assignments, and managed identities.
- Model deployment type, region, quota, capacity, TPM/RPM limits, 429 telemetry, and fallback model path.
- Managed network/private endpoint approval roles, target-resource permissions, DNS/routing assumptions, and public access state.
- Diagnostics, audit logs, prompt/content logging boundaries, retention, and alert routing.
- Rollback plan: disable deployment, route away, revoke key, remove role assignment, or revert network change.

## When to push back

- The request normalizes full-access keys, Owner, or Contributor for routine use.
- The user asks to publish or expose an agent without proving Foundry role, quota, network, diagnostics, and rollback evidence.
- Quota or capacity is inferred from documentation rather than sampled configured-environment evidence.
- The evidence contains secrets, customer data, or environment identifiers that should be redacted instead of committed.
