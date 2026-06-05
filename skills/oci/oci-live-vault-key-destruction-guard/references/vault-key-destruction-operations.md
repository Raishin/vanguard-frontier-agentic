# OCI Live Vault Key Destruction Guard Operations

> Version note: OCI service behavior, catalog entries, and tooling change over time. Verify exact command syntax, permissions, regional availability, feature maturity, and catalog targets before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Deleting a key before proving every encrypted dependency is gone or rekeyed.
- Assuming key deletion is reversible after the waiting period.
- Rotating keys without validating consumers and older key-version dependencies.
- Ignoring vault replication and regional blast radius.

## Officially grounded service shape

- Official OCI documentation says deleting a key is destructive and can make data encrypted with that key inaccessible; OCI requires scheduling deletion with a waiting period.
- Official OCI documentation says the key deletion wait period defaults to 30 days and can be shortened to a minimum of 7 days.
- Official OCI documentation recommends backing up a key before scheduling deletion and using service log data to analyze key usage.
- OCI API evidence through the user’s configured read-only OCI MCP shows key listing is compartment/vault scoped, requires a management endpoint, and can filter by protection mode, algorithm, length, curve, sorting, and pagination.
- OCI API evidence through the user’s configured read-only OCI MCP shows schedule deletion sets lifecycle to pending deletion and cancel deletion restores the key lifecycle state.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Separate disable, rotate, schedule deletion, cancel deletion, vault deletion, and encrypted-data recovery evidence.
- Require explicit owner and security approval before scheduling deletion or destructive key lifecycle actions.
- Prefer disablement, dependency audit, backup, and maximum waiting window before deletion.
- Do not expose key identifiers, vault endpoints, encrypted payloads, secrets, customer data, or sensitive identifiers.

## Minimal safe implementation flow

- Confirm key/vault scope, operation, protection mode, encrypted-resource dependencies, owner, and compliance driver.
- Use official docs for key lifecycle behavior and sampled read-only evidence for API shape/current lifecycle state.
- Audit usage logs, associated resources, backups, replication, and cancellation window.
- Return verdict, blockers, deletion-window stance, cancellation plan, and verification checks.

## High-risk assumptions to kill

- “No recent usage means safe to delete.”
- “Rotation removes dependency on old key versions.”
- “Scheduled deletion can always be cancelled later.”
- “Deleting a replicated vault/key only affects one location.”

Those are lazy assumptions.

## Safe command/code verification targets

- Check key lifecycle, protection mode, vault/replication context, and usage evidence without exposing identifiers.
- Validate encrypted resources have been rekeyed, retired, or proven disposable.
- Confirm backup/restore eligibility and cancellation window.
- Capture explicit approval and post-action monitoring.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations or live-guard dispatch have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks to delete a key without dependency and backup evidence.
- The proposed deletion window is shorter than needed for audit and cancellation.
- The evidence includes vault endpoints, key identifiers, secrets, or customer data.
