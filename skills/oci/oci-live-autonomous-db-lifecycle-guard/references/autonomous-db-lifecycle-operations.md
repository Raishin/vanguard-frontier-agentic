# OCI Live Autonomous DB Lifecycle Guard Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Stopping or scaling production without connection, batch, backup, and dependency evidence.
- Treating AVAILABLE as proof the application can tolerate a lifecycle action.
- Assuming storage scale, clone, restore, or termination has a clean rollback.
- Ignoring wallet, connection-string, Data Guard, and backup-recency impact.

## Officially grounded service shape

- Official OCI documentation describes Autonomous Database lifecycle operations including scaling, start/stop, cloning, backup, and recovery, but documentation does not prove a target database is safe to mutate.
- OCI API evidence through the user’s configured read-only OCI MCP shows Autonomous Database listing is compartment-scoped and can filter by infrastructure type, lifecycle state, workload, version, free tier, display name, refreshable clone, Data Guard, and resource-pool fields. Treat this as API-shape evidence, not mutation approval.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Separate lifecycle state, backup state, application dependency, wallet/connectivity, Data Guard, and protection evidence.
- Require explicit approval before scale, start, stop, clone, restore, refresh, or termination.
- Treat termination, restore, and irreversible capacity changes as blocked until owner, backup, rollback, and validation are proven.
- Do not expose database names, endpoints, wallets, credentials, customer data, or sensitive identifiers.

## Minimal safe implementation flow

- Confirm target database, operation, criticality, owner, maintenance window, and rollback expectation using sanitized references.
- Use official docs for lifecycle semantics and sampled read-only evidence for current lifecycle/API shape.
- Check dependencies, backup/restore, connection impact, and protection controls before approval.
- Return verdict, blockers, approval state, rollback posture, and post-change checks.

## High-risk assumptions to kill

- “AVAILABLE means safe to change.”
- “A backup exists and is restorable.”
- “Clone or restore has no downstream impact.”
- “Stopping a database only affects the database team.”

Those are lazy assumptions.

## Safe command/code verification targets

- List target database lifecycle and configuration without exposing identifiers.
- Check backup recency, restore path, Data Guard state, wallet/connection impact, and dependent workload windows.
- Capture pre-change state and owner approval before mutation.
- Validate application connectivity, performance, backups, and alerts after change.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks to terminate, restore, stop, or scale without owner approval and rollback evidence.
- The backup or dependency evidence is missing.
- The request includes wallet, endpoint, credential, or customer data.
