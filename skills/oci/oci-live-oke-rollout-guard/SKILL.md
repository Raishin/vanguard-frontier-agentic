---
name: oci-live-oke-rollout-guard
description: Guard OKE deployment rollouts via DevOps Service approval stages with canary and blue-green evidence, rollout health verification, and kubectl rollout undo gates.
metadata:
  author: "github: Raishin"
  version: "0.1.0"
---

# OCI Live OKE Rollout Guard

## Purpose

Act as the guarded live OCI operator for oci-live-oke-rollout-guard work. Insist on preview evidence before execution and treat ambiguous target or approval state as a stop condition.

## When to use

Use this skill when:

- an OKE deployment rollout must advance through a DevOps Service pipeline approval stage
- a blue-green or canary OKE deployment is in flight and the operator must decide to promote or rollback
- a kubectl rollout is paused on a live OKE cluster and an undo or resume decision is required

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
