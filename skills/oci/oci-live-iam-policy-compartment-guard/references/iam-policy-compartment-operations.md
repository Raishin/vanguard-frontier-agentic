# OCI Live IAM Policy Compartment Guard Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Approving policy names instead of policy statements.
- Treating temporary broad access as harmless.
- Ignoring parent-scope policies that still apply to child compartments.
- Changing dynamic-group rules without checking resource-principal blast radius.

## Officially grounded service shape

- Official OCI documentation defines policy statements as Allow subject to verb resource in location with optional conditions.
- Official OCI documentation orders policy verbs from inspect to read to use to manage, with service-specific exceptions.
- Official OCI documentation warns that broad subjects can include many principals; broad resource types can cover all resources in scope.
- OCI API evidence through the user’s configured read-only OCI MCP shows policy listing is compartment-scoped and can filter by name and lifecycle state, but policy applicability requires reviewing individual statements.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Review statements, subject, verb, resource type, location, conditions, and dynamic-group matching rules.
- Flag broad subjects, all-resources, tenancy-wide scope, and manage verbs as high risk until justified.
- Capture current statements before write and require explicit approval and rollback plan.
- Do not expose identity exports, group names tied to real users, sensitive identifiers, or customer data.

## Minimal safe implementation flow

- Confirm requested access change, actor, scope, duration, owner, and rollback requirement.
- Use official docs for policy syntax/verbs and sampled read-only evidence for policy list/API shape.
- Compare proposed statements against least privilege and existing effective scope.
- Return verdict, safer policy shape, blockers, rollback capture, and validation checks.

## High-risk assumptions to kill

- “The policy name proves intent.”
- “Manage at compartment scope is low risk.”
- “Dynamic groups only affect one resource.”
- “Delete and recreate is a safe rollback.”

Those are lazy assumptions.

## Safe command/code verification targets

- Review exact sanitized policy statements and dynamic-group matching rules.
- Check policy lifecycle, compartment attachment, parent-scope effects, and broad-principal/resource patterns.
- Map each requested operation to the least verb/resource type.
- Validate rollback by restoring prior statements, not by memory.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks for broad manage access without a narrow operation.
- The policy subject, scope, duration, or rollback is unclear.
- The evidence includes unsanitized identity, group, principal, or customer data.
