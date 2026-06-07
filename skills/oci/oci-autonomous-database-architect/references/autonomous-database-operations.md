# OCI Autonomous Database Architect Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Flattening serverless, dedicated Exadata, Cloud@Customer, and multicloud database services into one generic ADB pattern.
- Calling a target compatible before checking workload type, database version, network path, wallet/TLS, Data Guard, backup, and support boundary.
- Assuming Autonomous Data Guard, backups, clones, and refreshable clones have interchangeable RTO/RPO semantics.
- Treating documentation availability as proof the user can deploy in a specific subscribed region or tenancy.

## Officially grounded service shape

- Official OCI documentation describes the service behavior and lifecycle concepts for this domain, but it does not prove the user's tenancy, compartments, IAM policies, limits, deployed resources, or production readiness.
- OCI API evidence through the user’s configured read-only OCI MCP shows Autonomous Database list operations expose compartment, infrastructure type, lifecycle state, workload type, database version, free-tier, refreshable clone, and Data Guard filters. Treat this as API shape evidence, not tenancy posture.
- Current-state claims need sampled read-only evidence or sanitized user-provided evidence.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Classify the deployment model before recommending architecture.
- Require workload, latency, data residency, compliance, RTO/RPO, sizing, and maintenance constraints.
- Keep wallets, connection strings, credentials, and customer identifiers out of chat and examples.
- Require private access and DNS/client posture review for regulated workloads.
- Separate documentation evidence, sampled API evidence, and user-provided sanitized evidence.

## Minimal safe implementation flow

- Classify deployment model and destination boundary.
- Capture workload profile, constraints, security, network, backup, and DR requirements.
- Use official OCI docs plus sampled read-only API shape/current-state evidence where available.
- Identify compatibility, migration, operations, and rollback blockers.
- Return go/no-go verdict, risks, safe next actions, and validation plan.

## High-risk assumptions to kill

- ““Autonomous” means no architecture work remains.”
- “Serverless and dedicated have the same operational responsibilities.”
- “A marketing-compatible multicloud offer is regionally available and operationally equivalent.”
- “Backup existence proves restore readiness.”
- “Wallet download or connection-string handling is routine and low risk.”

Those are lazy assumptions.

## Safe command/code verification targets

- List ADB resources only in confirmed scope and label results as sampled current-state evidence.
- Check infrastructure type, workload type, lifecycle state, Data Guard flag, clone/refreshable clone posture, and version filters before drawing conclusions.
- Verify network access model, wallet/TLS posture, backup/restore evidence, alarms, Data Safe/Operations Insights coverage, and limits.
- Compare official deployment-option docs to the chosen destination before recommending multicloud movement.
- Require rollback/cutover validation before migration, failover, restore, key rotation, or stop/start actions.

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
