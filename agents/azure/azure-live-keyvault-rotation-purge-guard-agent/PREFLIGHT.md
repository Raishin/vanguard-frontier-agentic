# Preflight

Before Azure Live Key Vault Rotation Purge Guard live action:

1. Confirm target scope, resource, desired action, expected impact, approval state, and rollback owner.
2. Review `references/keyvault-rotation-purge-agent-operations.md`, `references/official-sources.md`, and `references/safety-checklist.md`.
3. Collect read-only evidence first and label it as sampled configured-environment evidence.
4. Verify rollback or disablement path before any mutation.
5. Block if target, approval, evidence, or rollback posture is ambiguous.
