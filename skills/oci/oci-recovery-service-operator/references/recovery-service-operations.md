# OCI Recovery Service Operator Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Treating a protection policy as proof of recoverability.
- Ignoring redo status, last successful backup, subnet/network dependency, and restore-test evidence.
- Assuming retention lock fixes bad backup scope or failed backups.
- Claiming RPO/RTO compliance without restore validation.

## Officially grounded service shape

- Official OCI documentation says Recovery Service uses protection policies to control backup retention, backup storage location, and backup protection for protected databases.
- Official OCI documentation says each protected database must be associated with one protection policy, with Oracle-defined and user-defined policy options.
- Official OCI documentation says user-defined protection policies support a backup retention period from 14 to 95 days, and retention lock can restrict modification or deletion until retention ends.
- OCI API evidence through the user’s configured read-only OCI MCP shows protected database listing is compartment-scoped and can filter by lifecycle state, display name, protected database, protection policy, recovery service subnet, sorting, and pagination. Treat that as API-shape evidence, not restore readiness.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Separate protected database state, policy, retention, redo, backup health, subnet, storage location, and restore-test evidence.
- Require current backup and restore validation before resilience claims.
- Treat delete, policy change, retention change, or subnet changes as high risk.
- Do not expose database names, backup identifiers, subnet identifiers, customer data, or sensitive identifiers.

## Minimal safe implementation flow

- Confirm database scope, policy, RPO/RTO, retention, owner, and restore-readiness decision.
- Use official docs for Recovery Service behavior and sampled read-only evidence for API shape/current state.
- Check backup health, redo, recovery window, network dependencies, policy drift, and restore-test evidence.
- Return verdict, blockers, restore-readiness gaps, safe next actions, and validation plan.

## High-risk assumptions to kill

- “Backups configured means restore works.”
- “Policy attached means RPO is met.”
- “Retention lock is a substitute for restore testing.”
- “No failed backup alarm means healthy.”

Those are lazy assumptions.

## Safe command/code verification targets

- List protected databases and policies without exposing identifiers.
- Check lifecycle, redo status, last backup, recovery window, subnet health, and policy mapping.
- Validate restore drill evidence and business RPO/RTO.
- Confirm monitoring, alerts, and escalation ownership.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks for restore-readiness approval without restore-test evidence.
- Backup health or redo status is missing.
- The request would delete/change policy or retention without explicit approval.
