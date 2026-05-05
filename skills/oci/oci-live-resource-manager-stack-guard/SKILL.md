---
name: oci-live-resource-manager-stack-guard
description: Guard OCI Resource Manager stack plan, apply, and destroy jobs with drift detection, state-version rollback, stack auto-lock awareness, and approval gates.
allowed-tools: Read Grep Glob WebFetch
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-05"
  category: delivery
---

# OCI Live Resource Manager Stack Guard

## Purpose

Act as the guarded live OCI operator for oci-live-resource-manager-stack-guard work. Insist on preview evidence before execution and treat ambiguous target or approval state as a stop condition.

## When to use

Use this skill when:

- an OCI Resource Manager stack apply or destroy job must be run against a live environment
- drift has been detected on a stack and resolution requires an apply job with human approval
- a Resource Manager stack state must be inspected, imported, or rolled back after a partial apply

## Lean operating rules

- Prefer OCI CLI (`oci`) official documentation when available; fall back to Oracle Cloud docs and sanitized user evidence.
- Do not execute a live OCI change until tenancy, compartment, active principal, and resource ownership are explicit.
- Prefer plan, detect-drift, inspect, read, describe, and rollback evidence before execution.
- If the request skips preview or rollback design, push back.
- Never print secrets, API keys, tenancy OCIDs, private key contents, or raw config values. Summarize sanitized evidence only.
- Load references only when needed.

## References

Load these only when needed:

- [Preflight commands](references/preflight-commands.md) — OCI CLI commands to run before any mutation.
- [Rollback playbook](references/rollback-playbook.md) — concrete rollback steps for this service.
- [Permission model](references/permission-model.md) — OCI IAM policy statements and dynamic group guidance.
- [Official sources](references/official-sources.md) — authoritative OCI documentation links.

## Response minimum

Return, at minimum:

- confirmed tenancy, compartment, and active principal
- preflight evidence (plan output, drift result, inspect/read, health check)
- approval status for the proposed mutation
- rollback posture or explicit statement of what cannot be rolled back
- post-action verification steps or refusal reason
