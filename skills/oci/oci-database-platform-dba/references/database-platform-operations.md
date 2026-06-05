# OCI Database Platform DBA Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Pretending one command family covers Base DB, Autonomous Database, and Exadata.
- Patching or scaling without checking standby, backup, maintenance, and application windows.
- Calling backup configured without restore evidence.
- Treating display names as unique database identity.
- Granting broad database-family manage access for read-only DBA review.

## Officially grounded service shape

- Official OCI documentation describes the service behavior and lifecycle concepts for this domain, but it does not prove the user's tenancy, compartments, IAM policies, limits, deployed resources, or production readiness.
- OCI API evidence through the user’s configured read-only OCI MCP shows DB system list operations expose compartment, backup, lifecycle, availability-domain, and display-name filters, and warns the older DB system list API is deprecated for Exadata Cloud Service systems. Treat this as API shape evidence and product-family caveat.
- Current-state claims need sampled read-only evidence or sanitized user-provided evidence.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Classify database product family before action.
- Confirm compartment, region, database identity, owner, environment, and maintenance window.
- Require backup/restore and rollback evidence before lifecycle, patch, scale, restore, or failover actions.
- Separate DBA operational permissions from destructive and security-sensitive permissions.
- Keep database passwords, wallets, connection strings, and customer identifiers out of chat.

## Minimal safe implementation flow

- Classify request and database family.
- Collect inventory, lifecycle, backup, Data Guard, maintenance, alarms, and IAM evidence.
- Identify product-specific blockers and dependencies.
- Plan minimal reversible action with owner approval.
- Validate database state, application reachability, metrics, backups, and rollback criteria.

## High-risk assumptions to kill

- “The prod database is obvious from its name.”
- “Backups mean restore is safe.”
- “A standby means failover is ready.”
- “Patch automation removes DBA responsibility.”
- “Autonomous and DB systems have identical semantics.”

Those are lazy assumptions.

## Safe command/code verification targets

- List DB systems/databases only in confirmed scope and check lifecycle state and availability domain.
- Use product-specific APIs for Autonomous and Exadata rather than stale DB-system assumptions.
- Verify latest backup, restore test, retention, Data Guard status, alarms, and maintenance window.
- Check storage, CPU, sessions, wait events, and application dependency evidence before scaling or tuning.
- Require explicit approval for patch, failover, switchover, restore, clone, delete, or parameter changes.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks for a write/delete/start/stop/update/remediate action before scope and owner are clear.
- The answer would depend on live infrastructure state but only documentation evidence exists.
- The proposed access is broader than the task requires.
- The plan has no rollback, owner, or validation step.
