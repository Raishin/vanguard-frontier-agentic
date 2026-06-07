# Official Sources

Use these sources to ground the skill. Microsoft Learn documentation proves documented Azure behavior; it does not prove the user's tenant, RBAC, quotas, deployed resources, or production readiness.

## Primary Microsoft Learn sources

- https://learn.microsoft.com/azure/key-vault/secrets/secure-secrets
- https://learn.microsoft.com/azure/key-vault/general/secure-key-vault
- https://learn.microsoft.com/azure/key-vault/general/rbac-guide
- https://learn.microsoft.com/azure/key-vault/general/soft-delete-overview
- https://learn.microsoft.com/azure/key-vault/general/key-vault-recovery
- https://learn.microsoft.com/azure/key-vault/secrets/tutorial-rotation
- https://learn.microsoft.com/azure/key-vault/general/event-grid-overview
- https://learn.microsoft.com/azure/key-vault/policy-reference

## Grounding notes

- Documentation-based claim: Microsoft Learn evidence says Key Vault should be secured with vault segmentation, network restrictions, managed identities, Azure RBAC for critical workloads, soft delete, purge protection, rotation, logging, Event Grid monitoring, Azure Policy, and tested backup or recovery. Soft delete preserves deleted vaults and objects for a retention period, while purge protection blocks permanent deletion until the retention period elapses.
- Current-state claim: requires sampled read-only Azure evidence or sanitized user-provided evidence.
- Inference: allowed only when labeled and tied to observed fields or documented behavior.
- Do not include sensitive internal identifiers or secret material in findings.

## Source use rules

- Prefer Microsoft Learn documentation through the user's configured documentation MCP for current Azure service behavior.
- Use sampled read-only Azure evidence only to validate current configured-environment observations.
- If documentation and sampled evidence appear to conflict, report both and stop short of a production-ready verdict.
- Re-check official sources before changing high-risk guidance, because cloud behavior and feature availability can change.
