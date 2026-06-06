# OCI Fusion Apps Environment Operator Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Treating an ACTIVE lifecycle as proof users can transact safely.
- Planning production changes without maintenance, refresh, support, and dependency evidence.
- Assuming every environment type supports the same refresh or termination behavior.
- Confusing environment family consistency with business-process readiness.

## Officially grounded service shape

- Official OCI documentation describes Fusion Applications Environment Management as self-service management for environment families and environments, including lifecycle, maintenance, metrics, integrations, refresh, and some configurable options.
- Official OCI documentation says an environment family groups environments so applications are maintained, upgraded, and patched at the same levels.
- OCI API evidence through the user’s configured read-only OCI MCP shows fusion environment listing is compartment-scoped and can filter by environment family, display name, lifecycle state, sorting, and pagination. Treat lifecycle as control-plane evidence, not application readiness.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Separate environment family, environment, subscription, maintenance, refresh, access, integration, and support evidence.
- Do not claim tenant readiness without current environment, maintenance, availability, and owner evidence.
- Treat production refresh, termination, and go-live claims as high risk unless official docs and support evidence allow them.
- Never request tenant identifiers, subscription identifiers, customer records, user lists, or support artifacts containing sensitive data.

## Minimal safe implementation flow

- Confirm target environment family, environment type, and decision needed.
- Use official docs to classify lifecycle, maintenance, refresh, and support constraints.
- Use sampled read-only evidence only to confirm API shape or sanitized current state.
- Return readiness blockers, maintenance/change risks, safe next actions, and support/escalation evidence needed.

## High-risk assumptions to kill

- “ACTIVE means the application is ready for go-live.”
- “Test, development, demo, and production environments support the same operations.”
- “Environment family planning proves integrations and business processes are validated.”
- “Support evidence can be skipped because this is self-service.”

Those are lazy assumptions.

## Safe command/code verification targets

- List environments and environment families without exposing sensitive identifiers.
- Check lifecycle state, environment type, maintenance schedule, refresh eligibility, integrations, and support status evidence.
- Validate planned changes against business calendar, rollback options, and owner approvals.
- Confirm production claims with sampled current-state or sanitized support evidence.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks for production go-live, refresh, termination, or change approval based only on docs.
- The environment type is unclear or the requested operation may be unsupported.
- Evidence includes unsanitized customer, subscription, user, or support data.
