# OCI Identity Access Governor Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Approving broad manage permissions because the request is temporary.
- Reading policy names instead of policy statements.
- Ignoring dynamic-group matching rules and resource-principal blast radius.
- Assuming a compartment boundary is safe when policies at parent scopes still apply.

## Officially grounded service shape

- Official OCI documentation describes IAM policies as statements that grant groups or dynamic groups permissions to resources in a tenancy or compartment.
- Official OCI documentation says dynamic groups define matching rules for resources acting as principals and need policies before they have permissions.
- OCI API evidence through the user’s configured read-only OCI MCP shows policy listing is compartment-scoped and can filter by name and lifecycle state, but determining which policies apply to a group or compartment requires reviewing individual policy statements.
- OCI API evidence through the user’s configured read-only OCI MCP shows group and dynamic-group listing are tenancy-scoped list surfaces with name, lifecycle, sorting, and pagination filters.
- OCI API evidence through the user’s configured read-only OCI MCP shows compartment listing can return child compartments and optionally traverse a hierarchy depending on access-level and subtree options.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Review the actual policy statements, scope, verbs, resource types, conditions, groups, and dynamic-group rules.
- Separate inspect/read/use/manage access and call out destructive or privilege-escalating verbs.
- Require owner, justification, expiry, rollback, monitoring, and break-glass path for risky access.
- Do not ask for or commit identity exports containing sensitive identifiers or user data.

## Minimal safe implementation flow

- Confirm requested access decision, actor type, scope, resource family, duration, and risk tolerance.
- Use official docs for IAM policy semantics and sampled read-only evidence for list/API shape or sanitized policy snippets.
- Classify each grant as least-privilege, overbroad, ambiguous, destructive, or unsupported.
- Return safer statements, blockers, validation checks, and approval conditions.

## High-risk assumptions to kill

- “The policy name describes what it grants.”
- “Compartment-scoped manage access is always safe.”
- “Dynamic groups are harmless because no users are listed.”
- “Temporary broad access does not need expiry and audit.”

Those are lazy assumptions.

## Safe command/code verification targets

- Review policy statements rather than names alone.
- Check group, dynamic-group, compartment, lifecycle, and parent-scope evidence without exposing identifiers.
- Map verbs and resource types to required operations and remove unused permissions.
- Validate conditions, expiry, audit logging, and rollback for access changes.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks for broad manage all-resources access without a narrow task.
- The actor, scope, resource type, duration, or rollback is unclear.
- The evidence contains unsanitized identity, tenancy, group, or customer data.
