# OCI Network Architect Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Fixing connectivity by opening broad ingress.
- Ignoring overlapping CIDRs and asymmetric routing.
- Changing default route tables or security lists without knowing attached subnets and workloads.
- Treating NSGs and security lists as interchangeable blast-radius tools.

## Officially grounded service shape

- Official OCI documentation describes VCNs, subnets, route tables, gateways, DRGs, security lists, NSGs, and related networking components as separate controls with distinct blast radius.
- Official OCI documentation describes DRG route advertisements and path preferences when multiple private/public paths exist.
- Official OCI documentation explains security lists and NSGs as virtual firewall features; security lists apply at subnet association scope while NSGs can scope to supported VNIC resources.
- OCI API evidence through the user’s configured read-only OCI MCP shows VCN, DRG, and route-table listing are compartment-scoped with lifecycle/display-name/sorting/pagination filters where applicable.

Documentation evidence proves documented service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Microsoft Learn documentation through the user's configured documentation MCP can prove documented Azure behavior. None of these prove broad tenancy/subscription posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Separate CIDR, subnet, route, gateway, DRG, peering, DNS, security rule, load balancer, and workload evidence.
- Require current-state capture and rollback before route or security mutations.
- Flag public exposure, management ports, database ports, all-protocol egress, and default table changes as high risk.
- Do not expose customer CIDRs, private endpoints, topology diagrams with identifiers, credentials, or sensitive identifiers.

## Minimal safe implementation flow

- Confirm connectivity question, source, destination, protocol, port, VCN/subnet path, and desired decision.
- Use official docs for networking behavior and sampled read-only evidence for API shape/current state.
- Map routes, security controls, DNS, gateways, DRG attachments, and asymmetric paths.
- Return verdict, likely breakpoints, safe tests, rollback requirements, and least-risk fix.

## High-risk assumptions to kill

- “Open firewall proves the fix.”
- “A route exists, so return path works.”
- “Default security list is safe for production.”
- “Private subnet means private workload.”

Those are lazy assumptions.

## Safe command/code verification targets

- Check VCNs, subnets, route tables, DRGs, gateways, NSGs, security lists, DNS, and path analysis without exposing identifiers.
- Validate both forward and return path.
- Review source/destination CIDR, protocol, port, statefulness, and workload criticality.
- Capture current route/security state before mutation.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks to open broad access without path evidence.
- CIDR ownership or route preference is unclear.
- The change affects default tables, database subnets, or shared hub routing without rollback.
