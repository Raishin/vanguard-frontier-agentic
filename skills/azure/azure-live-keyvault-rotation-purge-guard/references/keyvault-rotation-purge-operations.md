# Azure Key Vault Rotation and Purge Operations

Use this reference for current, source-grounded service behavior and the hard live-operation gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Treating rotation as safe because Key Vault creates a new version.
- Purging soft-deleted material before proving no dependent service needs recovery.
- Granting purge rights to the same operators who rotate keys or secrets.
- Enabling purge protection without warning that it cannot be disabled.
- Disabling old key versions before re-encryption or dependent services are migrated.

## Officially grounded service shape

Microsoft Learn evidence says soft delete retains deleted vaults and objects for a configurable period, cannot be disabled once enabled, and purge protection prevents permanent deletion until retention elapses. Purge protection cannot be disabled or overridden after enablement. Key rotation should use versioning and rotation policy with dependent-service planning; purge operations require elevated permissions and can be irreversible.

- Soft delete and purge protection are recovery controls with retention windows.
- Purge protection depends on soft delete and prevents purge until retention passes.
- Key rotation creates new versions; consumers must either follow latest version safely or be updated deliberately.
- Secret rotation requires dependent application validation and rollback.
- Azure Policy can audit or deny missing soft delete and purge protection.

## Non-negotiable design rules

- Never request or print key material, secret values, connection strings, or private keys.
- Separate rotation authority from purge authority.
- List dependencies before rotate, disable, delete, recover, or purge.
- Warn explicitly when an operation is irreversible or not fully rollbackable.
- Prefer recovery and quarantine over purge unless destruction is approved and evidenced.

## Minimal safe implementation flow

- Scope vault, object type, object name, environment, owner, and dependent services.
- Collect read-only evidence for soft delete, purge protection, retention, RBAC, object versions, rotation policy, and deleted-object state.
- Classify requested action as rotation, policy update, enable protection, recover, delete, disable, or purge.
- Gate mutation on dependency impact, backup/recovery posture, and explicit approval.
- Verify new version, consumer health, recovery posture, audit events, and remaining risk.

## Safe verification targets

- Soft delete and purge protection state are known and documented.
- Purge role assignments are narrow, JIT where possible, and not routine automation.
- Rotation policy aligns with compliance and consumer behavior.
- Dependent services are tested against new key/secret versions.
- Recovery or purge decision has owner approval and retention/irreversibility caveat.

## When to push back

- The user asks to purge without dependency and approval evidence.
- The user asks to paste or export secret/key material.
- Rotation would break pinned-version consumers with no migration plan.
- Purge protection or retention implications are not understood by the approver.
