# OCI Multi-Cloud Architect Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Assuming connected clouds are resilient clouds.
- Ignoring overlapping CIDRs, asymmetric routing, DNS split brain, identity drift, and egress cost.
- Designing Azure–OCI interconnect without region/peering-location support and BGP ownership.
- Letting one cloud’s IAM, logging, or network model silently govern the other.

## Officially grounded service shape

- Official OCI documentation describes FastConnect as dedicated private connectivity and describes DRG route advertisements, Site-to-Site VPN, public/private peering, and path-preference concerns.
- Microsoft Learn documentation through the user’s configured documentation MCP describes Azure–OCI direct interconnection using ExpressRoute and FastConnect, region/peering-location prerequisites, matching circuit bandwidth, BGP address planning, and route verification.
- OCI API evidence through the user’s configured read-only OCI MCP shows DRG, VCN, and route-table listing surfaces are compartment-scoped and expose lifecycle, display name, sorting, and pagination filters where applicable. Treat API shape as inventory evidence, not design approval.

Documentation evidence proves documented service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Microsoft Learn documentation through the user's configured documentation MCP can prove documented Azure behavior. None of these prove broad tenancy/subscription posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Separate routing, DNS, identity, encryption, observability, cost, data residency, operations, and failure-mode evidence by cloud.
- For Azure–OCI, validate ExpressRoute/FastConnect region pair, bandwidth, BGP, route tables, and non-overlapping address space.
- Require owner model, incident path, rollback, and cost guardrails before production interconnect.
- Do not expose PSKs, route tables with customer CIDRs, tenant identifiers, endpoints, or sensitive diagrams.

## Minimal safe implementation flow

- Confirm clouds, regions, workloads, latency, bandwidth, routing, identity, DNS, and compliance goals.
- Use OCI docs for OCI networking and Microsoft Learn documentation through the user’s configured documentation MCP for Azure interconnect behavior.
- Use sampled read-only evidence for OCI API shape or sanitized current-state inventory.
- Return architecture verdict, failure modes, route/DNS/IAM risks, cost risks, and validation plan.

## High-risk assumptions to kill

- “Multi-cloud improves availability by default.”
- “Private connectivity removes all security risk.”
- “BGP advertised means reachable and approved.”
- “One observability stack sees the full failure path.”

Those are lazy assumptions.

## Safe command/code verification targets

- Check non-overlapping CIDRs, DRG/VCN/route tables, ExpressRoute/FastConnect readiness, BGP sessions, DNS, security rules, and route preference.
- Validate identity federation, secrets flow, logging, monitoring, and incident ownership.
- Estimate egress, latency, bandwidth, support, and operational cost.
- Test failover and rollback, not just connectivity.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user wants quick cloud interconnect without CIDR, BGP, DNS, and owner evidence.
- The design relies on unsupported region/peering-location assumptions.
- The plan hides one provider’s control gaps behind another provider’s tools.
