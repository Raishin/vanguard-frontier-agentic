# Official Sources

Use these sources to ground the skill. Microsoft Learn documentation proves documented Azure behavior; it does not prove the user's tenant, subscription, RBAC, quota, deployed resources, current cost, vault state, app health, or production readiness.

## Primary Microsoft Learn sources

- https://learn.microsoft.com/azure/key-vault/general/key-vault-recovery
- https://learn.microsoft.com/azure/key-vault/general/soft-delete-overview
- https://learn.microsoft.com/azure/key-vault/general/secure-key-vault
- https://learn.microsoft.com/azure/key-vault/keys/how-to-configure-key-rotation
- https://learn.microsoft.com/azure/key-vault/keys/secure-keys
- https://learn.microsoft.com/azure/key-vault/policy-reference

## Grounding notes

- Documentation-based claim: Microsoft Learn evidence says soft delete retains deleted vaults and objects for a configurable period, cannot be disabled once enabled, and purge protection prevents permanent deletion until retention elapses. Purge protection cannot be disabled or overridden after enablement. Key rotation should use versioning and rotation policy with dependent-service planning; purge operations require elevated permissions and can be irreversible.
- Current-state claim: requires sampled read-only Azure evidence or sanitized user-provided evidence.
- Live-operation claim: requires target, principal, approval, preflight evidence, rollback constraints, and post-action verification.
- Inference: allowed only when labeled and tied to observed fields or documented behavior.
- Do not include sensitive internal identifiers or secret material in findings.

## Source use rules

- Prefer Microsoft Learn documentation through the user's configured documentation MCP for current Azure service behavior.
- Use sampled read-only Azure evidence only to validate current configured-environment observations.
- If documentation and sampled evidence appear to conflict, report both and stop short of a production-ready verdict.
- Re-check official sources before changing high-risk guidance, because cloud behavior and feature availability can change.

## Current Microsoft Learn deltas checked on 2026-06-05

- Soft delete is enabled by default for new vaults and cannot be disabled after enablement.
- Purge protection is not enabled by default, but once enabled it cannot be disabled or bypassed during the retention period.
- Key rotation creates a new key version; it does not re-encrypt dependent data by itself, so old and new versions may both be needed during rewrap/migration.
- Recovering a soft-deleted vault does not automatically restore every integrated artifact such as role assignments or event subscriptions.

