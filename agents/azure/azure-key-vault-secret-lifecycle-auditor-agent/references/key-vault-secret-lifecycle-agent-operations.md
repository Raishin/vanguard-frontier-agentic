# Azure Key Vault Secret Lifecycle Auditor Agent Operations

> Version note: Azure service behavior, API surfaces, permissions, and operational safety guidance change. Verify exact behavior against Microsoft Learn documentation through the user's configured documentation MCP and any sampled configured-environment evidence before production use. Do not paste credentials, tokens, tenant identifiers, subscription identifiers, connection strings, certificates, private keys, kubeconfigs, or customer data into prompts, commands, or reference examples.

## What people get wrong

- Auditing secret posture by reading secret values instead of metadata, attributes, policy, RBAC, events, and recovery controls.
- Treating soft delete as full recovery readiness even though recovered vaults do not restore integrated RBAC role assignments or Event Grid subscriptions.
- Assuming purge protection is enabled because soft delete is enabled.
- Granting broad secret officer or administrator roles where metadata-only reader evidence would be enough.
- Treating an expiration date as rotation, or rotation as tested application failover.

## Officially grounded service shape

- Soft delete retains deleted vaults and objects for 7 to 90 days; once enabled it cannot be disabled.
- Purge protection is separate from soft delete and prevents purge until the retention period elapses.
- Purging requires special privilege and is immediate or blocked by purge protection depending on state.
- Recovering a soft-deleted vault does not restore integrated services such as Azure RBAC role assignments and Event Grid subscriptions.
- Secure-secret guidance emphasizes managed identities, granular RBAC, regular rotation, safe distribution, and recovery procedures.

That is the key insight:

> The agent is a metadata and control-plane auditor, not a secret reader; secret values are almost never needed to prove lifecycle posture.

## Non-negotiable design rules

### 1. Never retrieve secret values for a lifecycle audit unless the user has explicitly approved a narrowly justified secret-bearing operation.

### 2. Treat missing purge protection, broad purge authority, missing expiration, and untested recovery as high-risk findings.

### 3. Require rotation evidence to include owner, trigger, downstream consumer readiness, and post-rotation validation.

### 4. Separate vault recovery, object recovery, RBAC restoration, Event Grid restoration, and application recovery evidence.

### 5. Label read-only observations as sampled configured-environment evidence and avoid tenant-wide claims from narrow samples.

## Minimal safe implementation flow

- Classify the audit: RBAC, recovery, rotation, expiration, eventing, diagnostics, naming, tagging, or consumer readiness.
- Ground the check in Microsoft Learn Key Vault security, RBAC, soft-delete, recovery, and rotation guidance.
- Use read-only configured-environment evidence for vault settings, secret attributes, expiration, tags, deleted-object state, event subscriptions, and diagnostics.
- Do not request or display secret values; use metadata and sanitized names or counts.
- Return findings by severity with safe next actions, approval boundaries, and residual evidence gaps.

## High-risk assumptions to kill

- Soft delete means the vault is recoverable without restoring dependencies.
- Secret expiration means rotation is automated and safe.
- A broad Key Vault role is acceptable because the user is only auditing.
- Purge permission is harmless if nobody intends to use it.
- Documentation proves vault settings, role assignments, rotation jobs, or recovery tests.

## Safe command/code verification targets

Verify against current docs and safe local or read-only tooling before use:

- Vault settings: RBAC mode, soft delete, purge protection, retention period, network exposure, diagnostics, and alerts.
- Secret metadata: enabled state, expiration, not-before, tags, content type, versions, near-expiry events, and owner hints.
- Access: data-plane roles, purge roles, administrator roles, managed identities, and stale assignments.
- Rotation and eventing: Event Grid subscriptions, automation target, run history, consumer readiness, and rollback.
- Recovery: deleted vault/object list, restore process, recreated RBAC, recreated event subscriptions, and tested application recovery.

## When to push back

- The user asks to read or paste secret values for an audit that can be completed from metadata.
- Purge or delete actions are requested without explicit approval, scope, and recovery impact.
- Rotation is claimed without evidence of downstream application validation.
- Recovery is claimed from documentation rather than sampled environment evidence or test results.
