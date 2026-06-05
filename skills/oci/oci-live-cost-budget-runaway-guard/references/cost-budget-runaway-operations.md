# OCI Live Cost Budget Runaway Guard Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Raising budget thresholds to silence alerts instead of investigating spend.
- Treating budgets as hard quota enforcement.
- Approving expensive shapes without owner, duration, quota, and shutdown evidence.
- Stopping resources blindly without business criticality and rollback checks.

## Officially grounded service shape

- Official OCI documentation describes budgets as soft spending controls with alert rules; they are not hard enforcement of spend.
- Official OCI documentation says budget alerts can be based on percentage or absolute amount and actual or forecast spending.
- OCI API evidence through the user’s configured read-only OCI MCP shows budget listing is compartment-scoped and can filter by lifecycle state, display name, target type, sorting, and pagination.
- OCI API evidence through the user’s configured read-only OCI MCP shows summarized usage requests require tenant scope, time window, granularity, query type, grouping, tags, filters, and pagination. Treat that as API-shape evidence, not spend truth unless current state is sampled.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Separate budget alerts, quotas, usage, forecasts, resource inventory, and owner approval.
- Require financial authority before raising thresholds, increasing quotas, or provisioning high-cost resources.
- For emergency containment, prefer reversible stop/scale actions and preserve evidence before deletion.
- Do not expose billing exports, resource names, customer data, or sensitive business identifiers.

## Minimal safe implementation flow

- Confirm time window, scope, service, owner, cost signal, and requested action.
- Use official docs for budget/quota behavior and sampled read-only evidence for usage or budget API shape.
- Classify actions as observe, alert, stop, scale, quota, or budget mutation.
- Return financial risk, blocker list, emergency safe actions, approvals, and verification plan.

## High-risk assumptions to kill

- “Budget alerts prevent overspend.”
- “Forecasted cost is exact.”
- “Expensive equals waste.”
- “Stopping the biggest item is always safe.”

Those are lazy assumptions.

## Safe command/code verification targets

- Check usage by service, region, tag, compartment, SKU, and resource for the confirmed window.
- Check budget target type, alert rules, recipients, lifecycle state, and threshold basis.
- Validate quota and high-cost shape requests against owner, duration, and shutdown automation.
- Confirm emergency actions are reversible and approved.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks to raise thresholds or quotas without financial authority.
- The request treats alert noise as the problem instead of spend cause.
- Emergency stop/delete would affect unknown production dependencies.
