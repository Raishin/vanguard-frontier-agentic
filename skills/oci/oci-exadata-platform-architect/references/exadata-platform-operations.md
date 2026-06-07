# OCI Exadata Platform Architect Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Designing the VM cluster first and discovering quota, network, backup, or maintenance constraints later.
- Treating Exascale, Dedicated Infrastructure, Cloud@Customer, and multicloud database services as interchangeable.
- Assuming Data Guard configured means failover, application routing, and restore validation are proven.
- Using documentation to claim customer entitlement, available capacity, or production readiness.

## Officially grounded service shape

- Official OCI documentation separates Exadata Database Service on Dedicated Infrastructure from Exadata Database Service on Exascale Infrastructure; do not collapse their capacity and operational models.
- Official OCI documentation describes dedicated infrastructure as Exadata racks in OCI with one or more VM clusters and control through Console, CLI, and REST APIs.
- OCI API evidence through the user’s configured read-only OCI MCP shows cloud Exadata infrastructure listing is compartment-scoped and exposes lifecycle-state, display-name, cluster-placement, sorting, and pagination filters. Treat that as API-shape evidence, not capacity proof.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Separate infrastructure, VM cluster, database, backup, key management, network, and DR decisions.
- Verify licensing, support model, maintenance ownership, and database version support before design sign-off.
- Require connectivity, DNS, route, security-list, key, backup, and restore evidence before production readiness claims.
- Label Exadata capacity and quota statements as unverified unless backed by current configured-environment evidence.

## Minimal safe implementation flow

- Confirm target platform: Dedicated Infrastructure, Exascale, Cloud@Customer, or multicloud database service.
- Map databases, VM clusters, network paths, keys, backups, patch windows, RAC/Data Guard topology, and operational owners.
- Use official docs for service behavior and sampled read-only evidence for API shape or current inventory.
- Return platform decision, blockers, risk trade-offs, validation checks, and rollback/DR proof required.

## High-risk assumptions to kill

- “Dedicated and Exascale have the same operational constraints.”
- “Available docs mean capacity is available in the target environment.”
- “A standby database proves the DR runbook works.”
- “Oracle-managed infrastructure removes customer patching and database responsibilities.”

Those are lazy assumptions.

## Safe command/code verification targets

- List cloud Exadata infrastructure and VM cluster inventory without exposing identifiers.
- Check lifecycle states, maintenance windows, shape/capacity evidence, and backup/DR configuration.
- Validate network paths, DNS, security controls, key management, and restore/failover test evidence.
- Confirm official service limits and support requirements before deployment or migration.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks for a target shape or migration cutover without capacity, network, backup, and DR evidence.
- The plan mixes Exascale, Dedicated Infrastructure, Cloud@Customer, or multicloud assumptions.
- The requested action would scale, patch, terminate, restore, or fail over production without explicit approval.
