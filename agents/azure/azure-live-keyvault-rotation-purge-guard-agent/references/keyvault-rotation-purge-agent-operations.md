# Azure Live Key Vault Rotation Purge Guard Agent Operations

> Version note: Azure Key Vault recovery, purge protection, RBAC roles, rotation policy behavior, and service integration timing change. Verify exact behavior against Microsoft Learn documentation through the user's configured documentation MCP and any sampled configured-environment evidence before production use. Do not paste credentials, tokens, tenant identifiers, subscription identifiers, connection strings, certificates, private keys, secret values, key material, or customer data into prompts, commands, or reference examples.

## What people get wrong

- Treating purge protection as a reversible toggle; Microsoft Learn states it cannot be disabled or overridden once enabled.
- Granting purge rights to routine rotation operators, collapsing delete and permanent-delete separation.
- Rotating a customer-managed key and disabling the old key version before dependent services have adopted the new version.
- Using versioned key URIs for services that should follow the latest key version through a versionless URI.
- Assuming vault recovery restores every integration; RBAC role assignments and Event Grid subscriptions can need recreation after vault recovery.

## Officially grounded service shape

- Soft delete allows recovery of deleted vaults and key vault objects for a retention period from 7 to 90 days; new vaults have soft delete on by default and it cannot be disabled once enabled.
- Purge protection can be enabled only after soft delete; when enabled, deleted vaults and objects cannot be purged until the retention period passes.
- Purging is immediate and irrecoverable when permitted; purge requires special privileges such as Key Vault Purge Operator or equivalent authority.
- Key rotation creates a new key version; it does not re-encrypt underlying data, and old and new versions may both need to remain enabled until dependent services finish re-wrapping.
- Key rotation policy is per key and requires key-management permissions such as Key Vault Crypto Officer for rotation policy and on-demand rotation.

That is the key insight:

> The agent is a cryptographic material safety gate. It must prove vault recovery posture, purge boundary, key version dependency, rotation policy, approval, and rollback limits before allowing mutation.

## Non-negotiable design rules

### 1. Never recommend purge, purge-right grants, or purge-protection changes without explicit irreversibility and recovery evidence.

### 2. Block key-version disablement until dependent-service adoption and decrypt/unwrap safety are proven.

### 3. Keep purge, recover, rotate, backup, restore, and secret-bearing reads as privileged actions requiring explicit approval.

### 4. Prefer read-only vault state, RBAC, deleted-object inventory, rotation policy, version, and service dependency evidence before mutation.

### 5. Label configured-environment observations as sampled and bounded to the vault, object, version, and time window.

## Minimal safe implementation flow

- Confirm vault, object type, object name, desired action, approval state, recovery owner, and dependent service owner.
- Ground soft-delete, purge protection, recovery, and key-rotation behavior in Microsoft Learn Key Vault guidance.
- Collect read-only evidence for soft-delete, purge protection, retention, RBAC, deleted objects, key versions, rotation policy, expiry, Event Grid notifications, and dependent service usage.
- Decide: rotate, set policy, recover, deny purge, enable protection, disable version, or block; if action is live, require explicit human approval.
- Verify post-action version state, policy state, recovery posture, dependent service health, and open risks.

## High-risk assumptions to kill

- Purge protection can be undone by an administrator or support.
- Rotation immediately makes old key versions unnecessary.
- A recovered vault restores all integrated RBAC and Event Grid configuration.
- A secret value must be read to validate secret rotation.
- Documentation proves this vault's recovery state, dependencies, or approval posture.

## Safe command/code verification targets

Verify against current docs and safe local or read-only tooling before use:

- Vault state, soft-delete enabled, purge-protection state, retention period, purge/recover permissions, and deleted-object inventory.
- Object type, current version, enabled/disabled versions, expiry, not-before, rotation policy, near-expiry notification, and Event Grid integration.
- Dependent services, versionless URI use, service-specific adoption timing, old-version disable safety, and backup/restore boundary.
- Irreversibility warning, approval record, recovery plan, rollback limits, and post-action verification evidence.
- Sanitization: no secret values, key material, private keys, certificates, or connection strings in evidence.

## When to push back

- The vault, object, version, approval state, or recovery owner is ambiguous.
- The user requests purge or old-version disablement without dependency evidence.
- The user wants to paste secret values, key material, certificates, connection strings, or raw environment dumps.
- The requested action would mutate live cryptographic state without approval and recovery evidence.
