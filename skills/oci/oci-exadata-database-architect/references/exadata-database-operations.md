# OCI Exadata Database Architect Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Starting with an Exadata shape before workload and AWR evidence.
- Treating Dedicated Infrastructure, Exascale, Cloud@Customer, and multicloud services as interchangeable.
- Assuming Data Guard, backup, TDE key integration, and maintenance controls work identically across destinations.
- Ignoring network latency, private connectivity, DNS, support ownership, and cloud-provider governance.
- Using old DB-system APIs or assumptions for current Exadata resource models.

## Officially grounded service shape

- Official OCI documentation describes the service behavior and lifecycle concepts for this domain, but it does not prove the user's tenancy, compartments, IAM policies, limits, deployed resources, or production readiness.
- OCI API evidence through the user’s configured read-only OCI MCP shows cloud Exadata infrastructure list operations expose compartment, lifecycle, display name, cluster placement group, sorting, and pagination filters. It also distinguishes newer Exadata resource-model APIs from deprecated DB-system assumptions. Treat this as API shape evidence, not capacity or architecture proof.
- Current-state claims need sampled read-only evidence or sanitized user-provided evidence.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Classify Exadata destination and resource model before architecture.
- Require workload profile, database estate, peak metrics, licensing, RTO/RPO, maintenance, and owner constraints.
- Verify official docs for destination-specific backup, Data Guard, key-management, networking, and maintenance features.
- Do not promise region, capacity, or multicloud availability without current evidence.
- Require rollback/cutover and support escalation path before migration advice.

## Minimal safe implementation flow

- Capture workload, estate, destination, availability target, and operational ownership.
- Ground destination behavior in official docs and sampled API evidence where available.
- Map capacity, network, storage, HA/DR, backup, security, and maintenance blockers.
- Compare Dedicated, Exascale, Cloud@Customer, and multicloud fit.
- Return architecture verdict, option matrix, blockers, safe next actions, and validation plan.

## High-risk assumptions to kill

- “Exadata fixes bad SQL or bad workload design.”
- “Exascale and Dedicated have the same control model.”
- “Database@provider means native provider database semantics.”
- “Data Guard setup equals tested DR.”
- “Current docs from one destination apply to every multicloud destination.”

Those are lazy assumptions.

## Safe command/code verification targets

- List cloud Exadata infrastructure in confirmed scope and check lifecycle and resource model.
- Verify VM cluster, DB home, database, storage, backup, Data Guard, and maintenance evidence with destination-specific APIs/docs.
- Review workload metrics, RAC/Data Guard/TDE/RMAN/patch dependencies, and licensing before sizing.
- Check private connectivity, DNS, route, latency, key-management, and support ownership for multicloud or Cloud@Customer.
- Require migration rehearsal, validation queries, rollback criteria, and owner approvals before cutover.

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
