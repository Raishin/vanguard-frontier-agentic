# OCI Cost FinOps Analyst Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Starting with deletion or commitment buying before validating ownership, utilization, and business criticality.
- Treating forecasted savings as guaranteed.
- Confusing Container Engine CLI naming with cost tooling.
- Ignoring support, backup, data transfer, logging, reliability, and security costs.
- Using untagged spend as proof of waste instead of proof of missing accountability.

## Officially grounded service shape

- Official OCI documentation describes the service behavior and lifecycle concepts for this domain, but it does not prove the user's tenancy, compartments, IAM policies, limits, deployed resources, or production readiness.
- OCI API evidence through the user’s configured read-only OCI MCP shows usage-summary requests require tenant scope, time window, granularity, query type, grouping, tag grouping, filters, and pagination; budget listing exposes compartment, lifecycle, display name, and target-type filters. Treat this as API shape evidence, not billing truth.
- Current-state claims need sampled read-only evidence or sanitized user-provided evidence.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Build cost findings from a confirmed time window, scope, service, compartment, and tag model.
- Separate usage optimization from rate optimization and commitments.
- Require owner, dependency, backup/retention, rollback, and business criticality before stop/delete/downgrade actions.
- Label savings as estimate, forecast, sampled evidence, or unverified.
- Do not expose billing exports with customer identifiers or sensitive business data.

## Minimal safe implementation flow

- Confirm billing scope, time window, granularity, currency/reporting expectation, and owner model.
- Collect official docs plus sampled usage, budget, tag, and resource evidence where available.
- Classify spend by service, compartment, tag, region, SKU, resource, and owner.
- Prioritize reversible cleanup and rightsizing before commitments.
- Return savings candidates, risk, approvals, validation checks, and follow-up measurement.

## High-risk assumptions to kill

- “Cost drop equals optimization.”
- “Idle-looking means safe to delete.”
- “A commitment is always better than pay-as-you-go.”
- “Budget alerts enforce spending controls.”
- “Forecasted savings are guaranteed.”

Those are lazy assumptions.

## Safe command/code verification targets

- Query usage/cost by service, compartment, tag, region, SKU, and resource for the confirmed time window.
- Check budgets and alert rules for target type, lifecycle state, owners, and alert routing.
- Cross-check rightsizing candidates with utilization, dependency, backup, and business-calendar evidence.
- Validate tag coverage before showback/chargeback conclusions.
- Require approval before stop, delete, resize, retention, or commitment changes.

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
