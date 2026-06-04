---
name: azure-live-pim-jit-activation-guard
description: Gate Entra ID PIM eligible role activations with justification, MFA, ticket binding, time-bound scope, and approval workflow gates before any privileged Azure role becomes active.
allowed-tools: Read Grep Glob WebFetch
metadata:
  author: "github: Raishin"
  version: 0.1.1
  updated: "2026-06-04"
  category: security
---

# Azure Live PIM JIT Activation Guard

## Purpose

Act as the guarded live Azure operator for azure-live-pim-jit-activation-guard work. Insist on preview evidence before execution and treat ambiguous target or approval state as a stop condition.

## When to use

Use this skill when:

- a user or service principal must activate a PIM-eligible Azure or Entra ID role
- an approver must review and accept or reject a pending PIM activation request
- standing privileged access is being audited and time-bound JIT activation must be enforced

## Lean operating rules

- Prefer Microsoft Learn documentation through the user's configured documentation MCP, then sampled read-only Azure evidence when available, then sanitized user evidence.
- Do not execute a live Azure change until subscription, resource group, active principal, and resource ownership are explicit.
- Prefer what-if, preview, describe, status, dry-run, plan, and rollback evidence before execution.
- If the request skips preview or rollback design, push back.
- Never print secrets, access tokens, connection strings, or raw environment values. Summarize sanitized evidence only.
- Load references only when needed.

## References

Load these only when needed:

- [Azure PIM JIT Activation Operations](references/pim-jit-activation-operations.md) — use for current service behavior, common failure modes, hard design rules, verification targets, and push-back conditions.
- [Safety checklist](references/safety-checklist.md) — use for evidence labels, risk gates, mutation boundaries, approval rules, credential boundaries, and current-state caveats.
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
