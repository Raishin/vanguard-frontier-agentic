# OCI Solution Architect Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Drawing architecture without workload constraints.
- Approving single-region production without RTO/RPO and failure-mode rationale.
- Using landing-zone defaults as a substitute for operating model and ownership.
- Ignoring cost, observability, backup restore, and security controls until implementation.

## Officially grounded service shape

- Official OCI Cloud Adoption Framework documentation provides best practices and prescriptive guidance for planning and adopting OCI.
- Official OCI Core Landing Zone documentation describes compartments, groups, policies, VCNs, routing, secure interfaces, and security services including Cloud Guard, Flow Logs, Connector Hub, Vault, Vulnerability Scanning, Bastion, and Security Zones.
- Official OCI documentation describes regions, availability domains, fault domains, regional resources, and cross-region resources; architecture must account for those scopes.
- OCI API evidence through the user’s configured read-only OCI MCP can support sampled inventory or API shape, but architecture approval requires requirements, constraints, and current-state evidence.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Require workload criticality, data classification, RTO/RPO, latency, budget, compliance, ownership, and operating model before architecture approval.
- Separate documented service capabilities from the user’s current deployment state.
- Treat public exposure, tenancy-wide access, untested backups, missing observability, and unclear rollback as blockers.
- Do not expose customer architecture details, identifiers, endpoints, secrets, or sensitive diagrams.

## Minimal safe implementation flow

- Confirm business goal, constraints, workload, environment, criticality, and decision needed.
- Use official docs for service/architecture behavior and sampled read-only evidence for current-state facts.
- Evaluate identity, compartments, network, compute, data, resilience, security, observability, cost, and operations.
- Return architecture verdict, tradeoffs, blockers, ADR-ready decisions, roadmap, and validation plan.

## High-risk assumptions to kill

- “Reference architecture equals production architecture.”
- “Single AD/region is fine because OCI is resilient.”
- “Backups exist, so recovery exists.”
- “Cost and operations can be solved later.”

Those are lazy assumptions.

## Safe command/code verification targets

- Check requirements, NFRs, ownership, and constraints before design.
- Validate region/AD/fault-domain scope, compartments, IAM, network, data protection, observability, security, and cost.
- Confirm backup restore, failover, runbooks, alerting, and support model.
- Produce risk register and explicit open questions.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user wants a design without constraints.
- Production readiness is requested without current-state and validation evidence.
- The design hides major risks behind generic reference architecture language.
