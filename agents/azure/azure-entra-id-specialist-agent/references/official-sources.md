# Official sources

Use this reference when grounding current Microsoft Entra ID identity security and governance behavior.

## Microsoft Learn sources refreshed on 2026-06-04

- https://learn.microsoft.com/entra/identity/conditional-access/overview
- https://learn.microsoft.com/entra/identity/role-based-access-control/best-practices
- https://learn.microsoft.com/entra/id-governance/best-practices-secure-id-governance
- https://learn.microsoft.com/entra/architecture/authorize-applications-resources-workloads
- https://learn.microsoft.com/entra/workload-id/workload-identities-overview
- https://learn.microsoft.com/entra/identity/conditional-access/workload-identity
- https://learn.microsoft.com/entra/id-protection/overview-identity-protection
- https://learn.microsoft.com/security/zero-trust/sfi/higher-security-microsoft-entra-id-apps

## Current documentation refresh notes

- Microsoft Learn documentation through the user's configured documentation MCP proves documented Azure service behavior only.
- It does not prove the user's tenant, subscription, RBAC, quota, deployed resources, production readiness, cost posture, or incident status.
- If documentation and sampled configured-environment evidence conflict, report both and explain the narrower scope of the sample.

## Evidence handling

- `documentation-based`: cite Microsoft Learn URLs and state what the docs prove.
- `sampled evidence`: read-only configured-environment observation with scope and time window.
- `user-provided sanitized evidence`: user input after redaction; validate before relying on it.
- `inference`: a cautious conclusion that still needs proof.

## Grounding rule

Docs explain service behavior. They do not prove the user's licensing, live configuration, permissions, usage, data, resources, or business readiness.
