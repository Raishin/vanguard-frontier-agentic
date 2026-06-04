# Official sources

Use this reference when grounding current Azure behavior for `azure-rbac-review`.

## Microsoft Learn sources

- https://learn.microsoft.com/azure/role-based-access-control/overview
- https://learn.microsoft.com/azure/role-based-access-control/best-practices
- https://learn.microsoft.com/azure/role-based-access-control/scope-overview
- https://learn.microsoft.com/azure/role-based-access-control/built-in-roles
- https://learn.microsoft.com/azure/role-based-access-control/custom-roles
- https://learn.microsoft.com/azure/role-based-access-control/conditions-overview
- https://learn.microsoft.com/entra/id-governance/privileged-identity-management/pim-configure

## Current documentation refresh (2026-06-04)

- Microsoft Learn documentation through the user's configured documentation MCP is the primary source for documented Azure behavior.
- Documentation evidence is not live customer-state evidence. It does not prove the user's tenant, subscription, RBAC, quotas, deployed resources, incident posture, private connectivity, automation state, or production readiness.
- Use sampled read-only Azure evidence only when the user has configured it and the task requires current-state confirmation. Label it as sampled evidence, not broad proof.

## Grounding rule

Docs explain service behavior. Current-state claims require sampled read-only evidence or sanitized user-provided evidence. If current state was not queried or shown, say so.
