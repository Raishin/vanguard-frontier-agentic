# OCI Migration Cutover Architect Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Cutting over because replication exists while dependencies, DNS, IAM, and rollback are unproven.
- Treating migration tooling as application readiness.
- Ignoring freeze windows, data reconciliation, monitoring, and support bridge ownership.
- Preserving bad network, IAM, or database design because lift-and-shift is faster.

## Officially grounded service shape

- Official OCI documentation describes Oracle Cloud Migrations as a service for discovering external VMware virtual machines and AWS EC2 instances, organizing workloads, replicating data, planning, and launching OCI compute.
- Official OCI documentation describes the migration workflow as phases for managing assets, planning migrations, and launching migrated resources.
- OCI API evidence through the user’s configured read-only OCI MCP shows migration listing can filter by compartment, lifecycle state, display name, migration identifier, sorting, and pagination. Treat this as API-shape evidence, not cutover readiness.
- Resource Search command-help evidence can support inventory-style discovery across viewable compartments, but it proves only the searchable API surface and caller-visible scope.

Documentation evidence proves documented service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Microsoft Learn documentation through the user's configured documentation MCP can prove documented Azure behavior. None of these prove broad tenancy/subscription posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Separate asset discovery, migration plan, replication, launch, validation, DNS, IAM, network, data, rollback, and support evidence.
- Require explicit go/no-go criteria, abort thresholds, owner approvals, and rollback path before cutover.
- Label readiness as unverified unless backed by current state or sanitized runbook evidence.
- Do not expose customer systems, hostnames, IPs, database names, migration identifiers, or sensitive identifiers.

## Minimal safe implementation flow

- Confirm source, target, wave, owner, business criticality, and cutover decision.
- Use official docs for Cloud Migrations behavior and sampled read-only evidence for API shape/current migration state.
- Map dependencies, data sync, DNS, identity, network, monitoring, backup, support, and rollback.
- Return go/no-go verdict, blockers, cutover sequence, rollback triggers, and validation checks.

## High-risk assumptions to kill

- “Replication complete means cutover ready.”
- “A migrated VM means the application works.”
- “DNS rollback is quick.”
- “Support can be called after problems start.”

Those are lazy assumptions.

## Safe command/code verification targets

- Check migration lifecycle and asset inventory without exposing identifiers.
- Validate dependency map, replication status, data reconciliation, DNS TTLs, IAM, routes, security rules, and monitoring.
- Confirm freeze window, owner approvals, support bridge, rollback procedure, and abort criteria.
- Run post-cutover application, data, security, and observability checks.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks to accelerate cutover without dependency and rollback proof.
- Data validation or DNS rollback is missing.
- The evidence includes unsanitized customer inventory or sensitive identifiers.
