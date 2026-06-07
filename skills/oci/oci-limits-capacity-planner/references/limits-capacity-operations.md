# OCI Limits Capacity Planner Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Confusing default limits with the user’s current entitlement.
- Confusing a quota policy with real available capacity.
- Submitting quota requests without lead time, fallback shapes, or alternate regions.
- Treating one region’s availability as global capacity proof.

## Officially grounded service shape

- Official OCI documentation distinguishes default service limits from quota policies and current tenancy limit values.
- Official OCI documentation says service-limit listings show limit types and scope; Console/API current values are needed for the user’s actual entitlement.
- OCI API evidence through the user’s configured read-only OCI MCP shows limit services and limit definitions are parent-compartment scoped and can be filtered by service, name, subscription, sorting, and pagination.
- OCI API evidence through the user’s configured read-only OCI MCP shows resource availability can return availability and usage for a service, limit, scope, and compartment when the limit supports that API; unsupported limits can return no value.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Separate service limits, quotas, current usage, regional subscription, availability-domain scope, and actual deployable capacity.
- Require target service, region, compartment scope, resource family, timeline, and fallback plan before readiness claims.
- Label capacity as unverified unless backed by current configured-environment evidence.
- Do not ask for or commit tenancy, subscription, compartment, or resource identifiers.

## Minimal safe implementation flow

- Confirm target deployment and capacity decision.
- Use official docs to identify limit families and quota syntax boundaries.
- Use sampled read-only evidence for service/limit definitions and resource availability where supported.
- Return readiness, blockers, quota-request evidence, fallback options, and validation checks.

## High-risk assumptions to kill

- “Default limit equals my limit.”
- “Quota policy means capacity exists.”
- “A quota increase will arrive before the deployment date.”
- “Capacity in one scope proves capacity in every scope.”

Those are lazy assumptions.

## Safe command/code verification targets

- Check service, limit definition, quota support, scope type, current usage, and availability where supported.
- Validate subscribed region and target location separately.
- Cross-check business forecast with quota lead time and fallback shape/region.
- Confirm official service-specific limit documentation before production planning.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user wants deployment approval without current limit/usage evidence.
- The request depends on capacity in a region or scope that was not sampled.
- The plan has no fallback for quota delay or capacity shortage.
