# OCI Live Network Security Rule Guard Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Adding open ingress for speed and promising to clean it later.
- Changing a Security List without realizing every associated subnet/VNIC is affected.
- Skipping current-state capture and calling that rollback.
- Ignoring stateful/stateless behavior and path validation.

## Officially grounded service shape

- Official OCI documentation describes Security Lists and Network Security Groups as virtual firewall features using security rules at packet level.
- Official OCI documentation recommends NSGs for finer scope where supported and explains that Security Lists apply to all VNICs in associated subnets.
- Official OCI documentation explains security rules, stateful/stateless behavior, and comparisons between Security Lists and NSGs.
- OCI API evidence through the user’s configured read-only OCI MCP shows Security List get requires the target security-list identifier and NSG rules listing can filter by direction, sorting, and pagination. Treat this as API-shape evidence, not approval to change rules.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Capture current rule state before any mutation.
- Flag open internet ingress, all-protocol egress, database ports, management ports, and database subnets as high risk.
- Prefer narrow source/destination, NSG-based scope where supported, explicit protocol/port, owner approval, and timed rollback.
- Do not expose network identifiers, private endpoints, customer CIDRs, credentials, or sensitive topology.

## Minimal safe implementation flow

- Confirm target VCN component, rule delta, business justification, owner, criticality, and rollback plan.
- Use official docs for rule semantics and sampled read-only evidence for API shape/current state.
- Classify rule risk by direction, source/destination, protocol, port, statefulness, and subnet criticality.
- Return verdict, required approvals, rollback baseline, safer alternative, and post-change verification.

## High-risk assumptions to kill

- “One rule affects only one workload.”
- “Open ingress is temporary, so it is safe.”
- “Rollback exists because we can edit it again.”
- “Connectivity success proves security acceptability.”

Those are lazy assumptions.

## Safe command/code verification targets

- Capture current Security List or NSG rules before mutation.
- Check direction, source/destination, protocol, port, statefulness, subnet attachment, and workload criticality.
- Use path analysis or equivalent connectivity validation where appropriate.
- Validate logs, reachability, and rollback after change.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks to open broad ingress or egress without justification and expiry.
- Current rules were not captured before mutation.
- The target component or blast radius is ambiguous.
