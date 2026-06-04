# Azure Key Vault Secret Lifecycle Operations

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Auditing by reading secret values instead of metadata, ownership, and lifecycle policy.
- Treating soft delete alone as enough for production recovery.
- Leaving purge authority with broad operators or automation identities.
- Using one vault as a shared dumping ground across apps, environments, or tenants.
- Equating an expiration date with a tested rotation path.

## Officially grounded service shape

Microsoft Learn evidence says Key Vault should be secured with vault segmentation, network restrictions, managed identities, Azure RBAC for critical workloads, soft delete, purge protection, rotation, logging, Event Grid monitoring, Azure Policy, and tested backup or recovery. Soft delete preserves deleted vaults and objects for a retention period, while purge protection blocks permanent deletion until the retention period elapses.

- Key Vault is a security boundary for keys, secrets, and certificates, not a general configuration database.
- Critical workloads should prefer Azure RBAC and managed identities; legacy access policies are harder to govern safely.
- Soft delete enables recovery; purge protection prevents premature permanent deletion during retention.
- Secret rotation commonly depends on owner, dependency mapping, eventing or alerting, automation, and rollback.
- Event Grid and logs can support lifecycle visibility, but alerts must map to owners and tested runbooks.

## Non-negotiable design rules

- Never ask for or print secret values, connection strings, tokens, or private keys.
- Audit secret names, versions, enabled state, expiration, tags, content type, owners, RBAC, and deleted-object posture instead.
- Require purge permission to be narrowly scoped, JIT where possible, and separate from routine secret writers.
- Treat missing recovery test evidence as a blocker for production-safe claims.
- Require app dependency impact analysis before disabling, deleting, purging, or rotating secrets.

## Minimal safe implementation flow

- Scope vaults by application, environment, region, and tenant boundary.
- Confirm RBAC/access model, network exposure, soft delete, purge protection, logging, and policy coverage.
- Review secret inventory metadata for missing owner, missing expiration, stale versions, ambiguous naming, and unmanaged dependencies.
- Review rotation and recovery runbooks against actual downstream consumers and alert routes.
- Return findings without exposing secret material.

## Safe verification targets

- Vault uses an access model appropriate for critical workloads and has no broad persistent secret or purge roles.
- Soft delete and purge protection are enabled with retention expectations documented.
- Secrets have owner, purpose, expiration or justified exception, and rotation mechanism or compensating control.
- Near-expiry, deletion, and failed-rotation events route to accountable responders.
- Recovery has been tested for both secret object and dependent application behavior.

## When to push back

- The user wants secret contents pasted into chat.
- Purge, delete, recover, or rotation changes are requested without approval and rollback evidence.
- A single vault spans unrelated apps, environments, or tenants without a documented exception.
- No one can name the owner or dependent services for a production secret.
