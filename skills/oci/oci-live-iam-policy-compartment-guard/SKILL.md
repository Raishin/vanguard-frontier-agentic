---
name: oci-live-iam-policy-compartment-guard
description: Guard OCI IAM policy writes and dynamic group changes with verb-hierarchy audit, compartment scope enforcement, anti-pattern detection (any-user/any-group), and rollback via statement restore.
allowed-tools: Read Grep Glob WebFetch
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-05"
  category: security
---

# OCI Live IAM Policy Compartment Guard

## Purpose

Act as the guarded live OCI operator for oci-live-iam-policy-compartment-guard work. Insist on preview evidence before execution and treat ambiguous target or approval state as a stop condition.

## When to use

Use this skill when:

- an OCI IAM policy must be created or modified in a compartment or at tenancy root
- a dynamic group rule must be changed and blast-radius must be audited before write
- an IAM audit finds overly broad policies that must be narrowed with least-privilege verb selection

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
