# Packet A: Role Selection, Security Posture, Resource Organization

## Targets

- `skills/azure/azure-role-selector`
- `skills/azure/azure-security-posture-hardening`
- `skills/azure/azure-subscription-resource-organization`

## Evidence used

Microsoft Learn documentation through the user's configured documentation MCP. No live Azure environment state was sampled.

## Findings addressed

- Role selection now separates control-plane and data-plane permissions, built-in roles, custom role constraints, wildcard risk, and assignment scope.
- Security posture hardening now covers Key Vault Zero Trust guidance, managed identities, RBAC over legacy access policies, network restrictions, soft delete, purge protection, rotation, diagnostics, Defender, and policy.
- Subscription/resource organization now covers management groups, subscriptions as policy/management/scale boundaries, resource groups as lifecycle boundaries, policy inheritance, ownership, tagging, naming, regions, and quotas.
