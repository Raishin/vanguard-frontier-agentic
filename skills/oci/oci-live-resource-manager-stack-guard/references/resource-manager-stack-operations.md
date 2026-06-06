# OCI Live Resource Manager Stack Guard Operations

> Version note: OCI service behavior, catalog entries, and tooling change over time. Verify exact command syntax, permissions, regional availability, feature maturity, and catalog targets before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Approving apply without reading the plan.
- Treating drift detection as remediation.
- Destroying because the stack is wrong without checking retained resources, dependencies, and state.
- Assuming repo merge approval equals live infrastructure approval.

## Officially grounded service shape

- Official OCI documentation describes Resource Manager as Terraform-backed IaC for OCI resources, with stacks, configuration, state, jobs, and drift detection.
- Official OCI documentation says Resource Manager locks stack state so only one job can run on a stack at a time.
- Official OCI documentation describes plan, apply, destroy, and import state jobs; destroy releases resources associated with a stack and can be destructive.
- OCI API evidence through the user’s configured read-only OCI MCP shows stack listing can filter by compartment, stack, lifecycle state, display name, sorting, and pagination.
- OCI API evidence through the user’s configured read-only OCI MCP shows job listing can filter by compartment, stack, job, lifecycle state, display name, sorting, and pagination.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Separate plan, apply, destroy, import-state, drift, state, and provider-version evidence.
- Require explicit approval before apply, destroy, import state, or rollback state.
- Block if the plan includes destructive replacement, IAM/network/security changes, secret exposure, or unknown values without owner review.
- Do not expose Terraform variables, state secrets, private endpoints, customer data, or sensitive identifiers.

## Minimal safe implementation flow

- Confirm stack, job type, environment, owner, plan source, drift status, and desired decision.
- Use official docs for Resource Manager/Terraform behavior and sampled read-only evidence for API shape/current job state.
- Review plan actions, drift, state, locks, variables, provider versions, and rollback path.
- Return verdict, blockers, approval state, safe next actions, and post-job validation.

## High-risk assumptions to kill

- “Plan succeeded, so apply is safe.”
- “Drift means apply should run.”
- “Destroy only affects test resources.”
- “State rollback restores real infrastructure automatically.”

Those are lazy assumptions.

## Safe command/code verification targets

- Check stack lifecycle, latest jobs, drift status, and plan output from sanitized evidence.
- Inspect destructive actions, IAM/network changes, replacements, unknowns, and variables.
- Confirm state lock, backup/state-version path, and rollback limitations.
- Validate live resources after apply/destroy and capture job logs.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations or live-guard dispatch have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks to apply or destroy without plan output.
- Drift was not detected or the drift scope is unknown.
- The plan includes destructive or security-sensitive changes without explicit approval.
