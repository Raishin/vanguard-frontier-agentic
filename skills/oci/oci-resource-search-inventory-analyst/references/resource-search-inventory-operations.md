# OCI Resource Search Inventory Analyst Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Calling a Resource Search result “all resources” without permission and region caveats.
- Trusting names instead of tags, compartments, lifecycle, and dependency evidence.
- Using inventory as authorization to delete resources.
- Ignoring cross-region and availability-domain scoping.

## Officially grounded service shape

- Official OCI documentation says Search can find resources within a tenancy, Console pages, and documentation.
- Official OCI CLI documentation says structured search queries resources matching query criteria and results include resources the caller has permission to view.
- Official OCI documentation explains that resources can be cross-region, regional, or availability-domain scoped, so inventory completeness depends on scope and resource type.
- OCI API evidence through the user’s configured read-only OCI MCP shows structured search accepts query text, pagination, matching context, and optional cross-tenancy tenant parameter. Treat search as caller-visible inventory evidence, not full posture proof.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- State query text, scope, permission caveat, region/resource-type caveat, and timestamp for every inventory.
- Separate inventory, ownership, dependency, cost, security, and cleanup recommendations.
- Require owner, dependency, backup, and business criticality before cleanup.
- Do not expose resource identifiers, private endpoints, customer names, or sensitive tags.

## Minimal safe implementation flow

- Confirm inventory purpose, scope, regions, compartments, resource types, and output format.
- Use official docs for Search behavior and sampled read-only evidence for query/API shape.
- Classify resources by type, lifecycle, compartment, tag, owner, exposure, dependency, and confidence.
- Return inventory summary, blind spots, risky resources, safe next actions, and follow-up queries.

## High-risk assumptions to kill

- “Search returns everything.”
- “Untagged means unused.”
- “Stopped means safe to delete.”
- “One compartment proves the tenancy.”

Those are lazy assumptions.

## Safe command/code verification targets

- Check query scope, permissions, region/resource-type coverage, lifecycle states, tags, and timestamps.
- Cross-check critical resources with service-specific read-only evidence where decisions are high impact.
- Validate owner and dependency before cleanup recommendations.
- Label all inventory conclusions by evidence level.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks for deletion from search output alone.
- Scope or permissions are unclear.
- The inventory contains sensitive identifiers or customer tags.
