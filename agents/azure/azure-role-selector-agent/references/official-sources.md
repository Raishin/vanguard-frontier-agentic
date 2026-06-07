# Official sources

Use this reference when grounding current Azure behavior for `azure-role-selector`.

Last refreshed: 2026-06-05

## Microsoft Learn sources

- https://learn.microsoft.com/en-us/azure/cloud-adoption-framework/ready/azure-setup-guide/manage-access
- https://learn.microsoft.com/en-us/azure/role-based-access-control/role-assignments-steps
- https://learn.microsoft.com/en-us/azure/role-based-access-control/built-in-roles
- https://learn.microsoft.com/en-us/azure/role-based-access-control/best-practices

## Grounding rules

- Use Microsoft Learn documentation through the user's configured documentation MCP for Azure service behavior.
- Treat documentation as documentation-based evidence only. It does not prove the user's tenant, subscription, RBAC, quota, deployed resources, costs, logs, incidents, or production readiness.
- Use read-only configured-environment evidence only when available and label it as sampled evidence.
- If documentation and sampled evidence conflict, report both and prefer the narrower conclusion.
- Do not mention private tool labels, workstation aliases, connection handles, or environment-specific identifiers in committed docs or final answers.
