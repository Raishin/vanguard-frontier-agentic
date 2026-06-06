# OCI Support Incident Coordinator Operations Reference

## What people get wrong

- Opening a support request transfers incident ownership to Oracle.
- Severity is a feeling instead of impact, scope, workaround, and time-to-mitigation evidence.
- Raw logs can be pasted into prompts or ticket drafts without redaction.
- A closed ticket proves the operational problem is fixed.
- Listing support requests is safe to quote verbatim in public docs.

## Officially grounded service shape

- Support requests are visible according to support-request user-group privileges and current user permissions.
- Oracle documents technical, billing, and service-limit support-request categories and paid-account eligibility caveats.
- The support incident list API is tenancy-scoped and supports lifecycle, severity/date sorting, problem type, pagination, and support-account context fields.
- Support evidence can prove ticket metadata and workflow state, not root cause, SLA compliance, customer impact, or production readiness by itself.

## Non-negotiable design rules

- Redact support identifiers, tokens, usernames, tenancy details, resource identifiers, stack traces with secrets, customer data, and private topology.
- Keep a timeline with exact timestamps, impact, affected services, changes, mitigations, owners, evidence source, and next action.
- Require approval before creating, updating, escalating, or attaching artifacts to a support request.
- Label support-request evidence as sampled configured-environment evidence, not general OCI behavior.
- Do not ask the user to paste support tokens, identifiers, tenancy IDs, or private logs into chat.

## Minimal safe implementation flow

- Classify incident type, severity, business impact, and blast radius.
- Use official support docs to ground what support requests can show.
- Use OCI API evidence through the user’s configured read-only OCI MCP only for sanitized request-listing shape or current-state sampling.
- Build a redacted support packet with timeline, evidence, suspected cause, mitigations, and explicit asks.
- Return escalation blockers before recommending ticket mutation.

## High-risk assumptions to kill

- Documentation proves service behavior; it does not prove the user's deployed posture.
- Sampled API evidence proves only the sampled command shape or observation.
- Read-only discovery is not approval for mutation.
- Missing evidence is a blocker, not a detail to smooth over.

## Safe command/code verification targets

- Prefer schema, manifest, link, and asset-integrity validation for repository edits.
- Prefer read-only list/get/help operations for cloud evidence.
- Redact or omit identifiers and sensitive values from notes and reports.

## Safe verification targets

- Official OCI documentation URL is attached to each service-behavior claim.
- Sampled API evidence is labeled with scope and limitation.
- Approval gates are explicit for every proposed mutation.
- Evidence gaps are listed as open questions.

## When to push back

- The request lacks impact or severity rationale.
- The user asks to include credentials, customer data, tokens, tenancy details, or raw private logs.
- The requested action would update support records without explicit approval.
