# Official sources

Use this reference when grounding current Azure behavior for `azure-waf-security-review`.

## Microsoft Learn sources

- https://learn.microsoft.com/azure/well-architected/security/principles
- https://learn.microsoft.com/azure/well-architected/security/checklist
- https://learn.microsoft.com/azure/well-architected/security/establish-baseline
- https://learn.microsoft.com/azure/well-architected/security/secure-development-lifecycle
- https://learn.microsoft.com/azure/well-architected/security/threat-model
- https://learn.microsoft.com/azure/well-architected/security/data-classification
- https://learn.microsoft.com/azure/well-architected/security/segmentation
- https://learn.microsoft.com/azure/well-architected/security/identity-access
- https://learn.microsoft.com/azure/well-architected/security/networking
- https://learn.microsoft.com/azure/well-architected/security/encryption
- https://learn.microsoft.com/azure/well-architected/security/harden-resources
- https://learn.microsoft.com/azure/well-architected/security/application-secrets
- https://learn.microsoft.com/azure/well-architected/security/monitor-threats
- https://learn.microsoft.com/azure/well-architected/security/test
- https://learn.microsoft.com/azure/well-architected/security/incident-response
- https://learn.microsoft.com/security/benchmark/azure/introduction
- https://learn.microsoft.com/azure/defender-for-cloud/concept-regulatory-compliance
- https://learn.microsoft.com/azure/defender-for-cloud/secure-score-security-controls
- https://learn.microsoft.com/azure/defender-for-cloud/review-security-recommendations

## Current documentation refresh (2026-06-05)

- Microsoft Learn documentation through the user's configured documentation MCP is the primary source for documented Azure behavior.
- Documentation evidence is not live customer-state evidence. It does not prove the user's tenant, subscription, RBAC, quotas, deployed resources, billing state, security posture, or production readiness.
- Use sampled read-only Azure evidence only when the user has configured it and the task requires current-state confirmation. Label it as sampled evidence, not broad proof.
- Secure score, regulatory compliance, and MCSB mappings help prioritize controls, but they do not replace workload-specific threat modeling, ownership, testing, and incident response evidence.

## Grounding rule

Docs explain service behavior. Current-state claims require sampled read-only evidence or sanitized user-provided evidence. If current state was not queried or shown, say so.
