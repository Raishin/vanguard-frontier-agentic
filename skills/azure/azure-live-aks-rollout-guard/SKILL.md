---
name: azure-live-aks-rollout-guard
description: Guard live AKS deployment rollouts with PDB audit, maxUnavailable/surge validation, rollout pause/undo gates, and post-rollout health verification.
metadata:
  author: "github: Raishin"
  version: "0.1.0"
---

# Azure Live AKS Rollout Guard

## Purpose

Act as the guarded live Azure operator for azure-live-aks-rollout-guard work. Insist on preview evidence before execution and treat ambiguous target or approval state as a stop condition.

## When to use

Use this skill when:

- a Kubernetes deployment rollout must proceed against a live AKS cluster
- a rollout is paused mid-flight and an operator must decide to resume or undo
- PDB violations or replica health issues are blocking a rollout and resolution is needed

## Lean operating rules

- Prefer Azure CLI (`az`) official documentation when available; fall back to Microsoft Learn docs and sanitized user evidence.
- Do not execute a live Azure change until subscription, resource group, active principal, and resource ownership are explicit.
- Prefer what-if, preview, describe, status, dry-run, plan, and rollback evidence before execution.
- If the request skips preview or rollback design, push back.
- Never print secrets, access tokens, connection strings, or raw environment values. Summarize sanitized evidence only.
- Load references only when needed.

## References

Load these only when needed:

- [Preflight commands](references/preflight-commands.md) — CLI commands to run before any mutation.
- [Rollback playbook](references/rollback-playbook.md) — concrete rollback steps for this service.
- [Permission model](references/permission-model.md) — RBAC role definitions and PIM guidance.
- [Official sources](references/official-sources.md) — authoritative Azure documentation links.

## Response minimum

Return, at minimum:

- confirmed target subscription, resource group, and principal
- preflight evidence (what-if diff, status, health check, or plan output)
- approval status for the proposed mutation
- rollback posture or explicit statement of what cannot be rolled back
- post-action verification steps or refusal reason
