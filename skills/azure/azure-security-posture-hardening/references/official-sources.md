# Official sources

Use this reference when grounding current Azure behavior for `azure-security-posture-hardening`.

## Microsoft Learn sources

- https://learn.microsoft.com/azure/key-vault/general/secure-key-vault
- https://learn.microsoft.com/security/benchmark/azure/baselines/key-vault-security-baseline
- https://learn.microsoft.com/security/benchmark/azure/baselines/microsoft-defender-for-cloud-security-baseline
- https://learn.microsoft.com/azure/defender-for-cloud/recommendations-reference-identity-access
- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-area/security
- https://learn.microsoft.com/azure/governance/policy/overview
- https://learn.microsoft.com/azure/role-based-access-control/best-practices
- https://learn.microsoft.com/azure/defender-for-cloud/secure-score-security-controls
- https://learn.microsoft.com/azure/defender-for-cloud/concept-cloud-security-posture-management
- https://learn.microsoft.com/azure/defender-for-cloud/review-security-recommendations

## Current documentation refresh (2026-06-05)

- Microsoft Learn documentation through the user's configured documentation MCP is the primary source for documented Azure behavior.
- Documentation evidence is not live customer-state evidence. It does not prove the user's tenant, subscription, RBAC, quotas, deployed resources, billing state, security posture, or production readiness.
- Use sampled read-only Azure evidence only when the user has configured it and the task requires current-state confirmation. Label it as sampled evidence, not broad proof.

## Grounding rule

Docs explain service behavior. Current-state claims require sampled read-only evidence or sanitized user-provided evidence. If current state was not queried or shown, say so.
