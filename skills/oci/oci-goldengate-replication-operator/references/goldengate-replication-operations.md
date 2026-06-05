# OCI GoldenGate Replication Operator Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Starting or stopping replication without checkpoint, trail, lag, and reconciliation evidence.
- Treating low lag as proof of transaction consistency or cutover readiness.
- Ignoring credential, network endpoint, vault, and source-target blast radius.
- Assuming a deployment backup covers every data-loss or rollback scenario.

## Officially grounded service shape

- Official OCI documentation describes OCI GoldenGate as a fully managed cloud service for real-time data movement and processing, with concepts including compartments, connections, deployments, deployment backups, pipelines, capture, and apply.
- Official OCI documentation describes GoldenGate connections as storing source or target connectivity information and notes encryption and network endpoint choices.
- OCI API evidence through the user’s configured read-only OCI MCP shows deployment listing is compartment-scoped and can filter by supported connection type, assigned connection, lifecycle state, lifecycle sub-state, deployment type, display name, endpoint name, sorting, and pagination.
- OCI API evidence through the user’s configured read-only OCI MCP shows connection listing is compartment-scoped and can filter by technology type, connection type, deployment assignment, lifecycle state, display name, sorting, and pagination.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Separate control-plane deployment health from data-plane replication correctness.
- Require source, target, schema, trail, checkpoint, lag, conflict, and reconciliation evidence before cutover claims.
- Treat credential changes, connection reassignment, extract/replicat changes, and deployment lifecycle actions as high risk.
- Keep secrets, connection strings, endpoint details, and customer data out of prompts and docs.

## Minimal safe implementation flow

- Confirm source, target, replication mode, deployment type, business criticality, and decision needed.
- Use official docs for GoldenGate concepts, connection behavior, and service responsibilities.
- Use sampled read-only evidence for deployment and connection API shape or sanitized current-state observations.
- Return health verdict, data-consistency risks, unsafe assumptions, and least-risk recovery or cutover steps.

## High-risk assumptions to kill

- “Lag is zero, so cutover is safe.”
- “Deployment ACTIVE means extracts and replicats are correct.”
- “A backup guarantees rollback of source and target data.”
- “Network connectivity is safe because the control plane accepted the connection.”

Those are lazy assumptions.

## Safe command/code verification targets

- Inventory deployments and connections without exposing identifiers or secrets.
- Check lifecycle states, lifecycle sub-states, deployment type, connection assignment, endpoint mode, and vault/encryption evidence.
- Validate extract/replicat status, trail/checkpoint continuity, lag, error logs, reconciliation, and rollback plan.
- Require application freeze or dual-write handling evidence before cutover.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks to start, stop, reassign, delete, restore, or cut over replication without reconciliation evidence.
- The plan relies on control-plane state as proof of data correctness.
- The evidence includes secrets, connection strings, private endpoints, or customer data.
