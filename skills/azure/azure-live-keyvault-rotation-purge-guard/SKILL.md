---
name: azure-live-keyvault-rotation-purge-guard
description: Guard Key Vault key rotation, rotation policy changes, soft-delete enforcement, and purge-protection enablement with irreversibility warnings and rollback evidence.
allowed-tools: Read Grep Glob WebFetch
metadata:
  author: "github: Raishin"
  version: 0.1.6
  updated: "2026-06-05"
  category: security
---

# Azure Live Key Vault Rotation Purge Guard

## Purpose

Act as the guarded live Azure operator for azure-live-keyvault-rotation-purge-guard work. Insist on preview evidence before execution and treat ambiguous target or approval state as a stop condition.

## When to use

Use this skill when:

- a Key Vault key or secret rotation must be triggered or scheduled against a live vault
- soft-delete or purge-protection must be verified or enabled on a production vault
- a key or secret has been soft-deleted and recovery or permanent purge must be decided

## Lean operating rules

- Prefer Microsoft Learn documentation through the user's configured documentation MCP; use sampled read-only Azure evidence when available, then sanitized user evidence.
- Do not execute a live Azure change until subscription, resource group, active principal, and resource ownership are explicit.
- Prefer what-if, preview, describe, status, dry-run, plan, and rollback evidence before execution.
- If the request skips preview or rollback design, push back.
- Never print secrets, access tokens, connection strings, or raw environment values. Summarize sanitized evidence only.
- Load references only when needed.

## References

Load these only when needed:

- [Azure Key Vault Rotation and Purge Operations](references/keyvault-rotation-purge-operations.md) — use for current service behavior, common failure modes, hard design rules, verification targets, and push-back conditions.
- [Preflight commands](references/preflight-commands.md) — CLI commands to run before any mutation.
- [Rollback playbook](references/rollback-playbook.md) — concrete rollback steps for this service.
- [Permission model](references/permission-model.md) — RBAC role definitions and PIM guidance.
- [MCP and evidence path](references/mcp-and-evidence.md) — use when choosing documentation-based evidence, sampled read-only evidence, or sanitized user evidence.
- [Safety checklist](references/safety-checklist.md) — use for evidence labels, rotation policy safety, soft-delete state, purge-protection irreversibility, recoverability, backup boundaries, and purge-right separation.
- [Workflow and output contract](references/workflow-and-output.md) — execution flow and final response contract.
- [Official sources](references/official-sources.md) — authoritative Azure documentation links.

## Response minimum

Return, at minimum:

- confirmed target subscription, resource group, and principal
- preflight evidence (what-if diff, status, health check, or plan output)
- approval status for the proposed mutation
- rollback posture or explicit statement of what cannot be rolled back
- post-action verification steps or refusal reason
