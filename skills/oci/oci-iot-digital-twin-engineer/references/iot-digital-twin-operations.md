# OCI IoT Digital Twin Engineer Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Changing a model or relationship without checking downstream telemetry, command paths, and consumers.
- Assuming a twin topology matches physical assets just because the model compiles.
- Ignoring data quality, device identity, command authorization, and rollback.
- Treating API shape as proof that devices, adapters, and instances exist or are healthy.

## Officially grounded service shape

- Official OCI documentation describes the Internet of Things Platform as bidirectional communication and data collection between OCI and devices or external applications, with domain groups, domains, work requests, digital twin models, adapters, instances, and relationships.
- Official OCI documentation lists device connectivity, REST APIs, Data API, database schema, CLI, SDKs, and data consumption paths such as APEX, REST Data Services, database connections, and analytics.
- OCI API evidence through the user’s configured read-only OCI MCP shows the IoT CLI surface includes domain groups, domains, work requests, digital twin adapters, models, instances, and relationships.
- OCI API evidence through the user’s configured read-only OCI MCP shows digital twin model and instance listing are domain-scoped and can filter by display name, lifecycle state, model linkage, model spec URI, sorting, and pagination. Treat this as API-shape evidence, not deployed-device truth.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Separate domain, model, adapter, instance, relationship, telemetry, command, and consumer evidence.
- Require versioning, compatibility, dependency, and rollback checks before model or relationship changes.
- Treat device commands, adapter changes, and relationship deletions as high risk.
- Keep device identifiers, customer data, payload samples, credentials, and endpoint details sanitized.

## Minimal safe implementation flow

- Confirm target domain, digital twin asset type, topology decision, and whether commands or telemetry are affected.
- Use official docs for service behavior and sampled read-only evidence for API shape or sanitized current state.
- Map dependencies across models, instances, adapters, relationships, consumers, and command paths.
- Return topology risks, compatibility blockers, safe validation checks, and rollback requirements.

## High-risk assumptions to kill

- “The digital twin graph is the physical truth.”
- “A model update is non-breaking.”
- “Telemetry arriving means device identity and command authorization are correct.”
- “Deleted relationships can be recreated without operational impact.”

Those are lazy assumptions.

## Safe command/code verification targets

- List domain-scoped models and instances without exposing identifiers.
- Check lifecycle state, model spec URI, adapter bindings, relationship dependencies, consumers, and command paths.
- Validate telemetry schema compatibility, versioning, and rollback before topology changes.
- Confirm audit, authorization, and safety controls for command-capable devices.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks to update or delete models, adapters, instances, or relationships without dependency and rollback evidence.
- The requested action could affect physical device commands or safety-critical telemetry.
- The evidence includes sensitive device identifiers, payloads, endpoint details, or customer data.
