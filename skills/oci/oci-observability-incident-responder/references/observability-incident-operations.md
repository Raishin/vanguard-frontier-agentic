# OCI Observability Incident Responder Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Treating an alarm as actionable without a runbook.
- Muting or deleting alarms to reduce noise before proving business impact.
- Searching logs without time window, compartment, service, and severity scope.
- Granting broad admin to responders because the incident is urgent.

## Officially grounded service shape

- Official OCI documentation describes Monitoring as metrics and alarms for active/passive monitoring, with alarms sending messages when metric streams meet triggers.
- Official OCI documentation describes event-based alarm notifications and repeat-notification behavior for point-in-time events.
- Official OCI documentation describes Logging as a managed interface to access, enable, manage, and search logs; log groups are access-control boundaries for logs.
- OCI API evidence through the user’s configured read-only OCI MCP shows alarm listing is compartment-scoped, supports subtree traversal only from root with tenancy-level permissions, and can filter by display name, lifecycle state, severity sorting, and pagination.
- OCI API evidence through the user’s configured read-only OCI MCP shows log-group listing is compartment-scoped and can optionally traverse nested compartments.

Documentation evidence proves documented service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Microsoft Learn documentation through the user's configured documentation MCP can prove documented Azure behavior. None of these prove broad tenancy/subscription posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Separate symptom, signal, scope, severity, timeline, impact, owner, containment, and recovery evidence.
- Require time window, compartment, service, metric/log source, alarm destination, and runbook before conclusions.
- Treat alarm suppression, routing changes, log deletion, and automation permissions as high risk.
- Do not expose log payloads, customer data, endpoints, identities, or sensitive identifiers.

## Minimal safe implementation flow

- Confirm incident scope, severity, service, time window, owner, and decision needed.
- Use official docs for Monitoring/Logging behavior and sampled read-only evidence for API shape/current telemetry state.
- Build evidence timeline from alarms, metrics, logs, events, notifications, service health, and recent changes.
- Return verdict, impact, likely cause, containment, validation, and follow-up actions.

## High-risk assumptions to kill

- “No alarm means no incident.”
- “FIRING means root cause is known.”
- “Dashboard green means users are fine.”
- “Admin access is required for response.”

Those are lazy assumptions.

## Safe command/code verification targets

- Check alarm lifecycle, severity, destinations, query, suppression, metric namespace, and notification path.
- Check log groups, logs, search permissions, time window, and sensitive payload handling.
- Correlate metrics, logs, events, service health, and recent changes.
- Validate containment and recovery with user-impact evidence.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks to suppress alarms without owner and impact evidence.
- The time window, compartment, service, or severity is vague.
- The evidence contains unsanitized logs, customer data, credentials, or sensitive identifiers.
