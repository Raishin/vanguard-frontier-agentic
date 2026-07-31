---
name: azure-live-app-service-slot-swap-guard
description: Guard live App Service slot swaps with sticky-settings audit, warmup probe verification, swap-with-preview staging, and instant rollback posture.
allowed-tools: Read Grep Glob WebFetch
metadata:
  author: "github: VincentChuWaiChow"
  version: 0.1.6
  updated: "2026-06-05"
  category: delivery
---

# Azure Live App Service Slot Swap Guard

## Purpose

Act as the guarded live Azure operator for azure-live-app-service-slot-swap-guard work. Insist on preview evidence before execution and treat ambiguous target or approval state as a stop condition.

## When to use

Use this skill when:

- an App Service slot swap to production must be staged and committed against a live environment
- sticky settings or connection strings differ between slots and the operator must audit before swap
- a swap-with-preview is in progress and the operator must decide to complete or reset

## Lean operating rules

- Prefer Microsoft Learn documentation through the user's configured documentation MCP; use sampled read-only Azure evidence when available, then sanitized user evidence.
- Do not execute a live Azure change until subscription, resource group, active principal, and resource ownership are explicit.
- Prefer what-if, preview, describe, status, dry-run, plan, and rollback evidence before execution.
- If the request skips preview or rollback design, push back.
- Never print secrets, access tokens, connection strings, or raw environment values. Summarize sanitized evidence only.
- Load references only when needed.

## References

Load these only when needed:

- [Azure App Service Slot Swap Operations](references/slot-swap-operations.md) — use for current service behavior, common failure modes, hard design rules, verification targets, and push-back conditions.
- [Preflight commands](references/preflight-commands.md) — CLI commands to run before any mutation.
- [Rollback playbook](references/rollback-playbook.md) — concrete rollback steps for this service.
- [Permission model](references/permission-model.md) — RBAC role definitions and PIM guidance.
- [MCP and evidence path](references/mcp-and-evidence.md) — use when choosing documentation-based evidence, sampled read-only evidence, or sanitized user evidence.
- [Safety checklist](references/safety-checklist.md) — use for evidence labels, slot-swap gates, sticky settings, warm-up status, production target confirmation, and rollback limits.
- [Workflow and output contract](references/workflow-and-output.md) — execution flow and final response contract.
- [Official sources](references/official-sources.md) — authoritative Azure documentation links.

## Response minimum

Return, at minimum:

- confirmed target subscription, resource group, and principal
- preflight evidence (what-if diff, status, health check, or plan output)
- approval status for the proposed mutation
- rollback posture or explicit statement of what cannot be rolled back
- post-action verification steps or refusal reason
