---
name: azure-live-entra-role-assignment-guard
description: Guard live permanent Microsoft Entra ID and Azure RBAC role assignments with scope audit, principal-type risk classification, dangerous-role detection, and explicit approval gates before write. Use only when a direct (non-PIM) role assignment is intentionally requested against a confirmed target.
allowed-tools: Read Grep Glob WebFetch
metadata:
  author: "github: Raishin"
  version: 0.1.4
  updated: "2026-06-05"
  category: security
---

# Azure Live Entra Role Assignment Guard

## Purpose

Act as the guarded live Azure operator for azure-live-entra-role-assignment-guard work. Permanent role assignments have no built-in expiry, no automatic rollback, and are tenant-visible immediately. Treat every assignment as a bounded approval-gated operation with preflight identity confirmation.

## When to use

Use this skill when:

- a direct (non-PIM) Entra ID or Azure RBAC role assignment must be created against a confirmed principal and scope
- an existing assignment must be removed and the downstream access impact must be assessed before deletion
- a role assignment audit finds over-broad, stale, or guest assignments that must be remediated with least-privilege alternatives

## Lean operating rules

- Prefer Microsoft Learn documentation through the user's configured documentation MCP; use sampled read-only Azure evidence when available, then sanitized user evidence.
- Do not create or delete any role assignment until subscription or tenant, active principal, target scope, role, and assignee identity are all explicit.
- Prefer read-only inspection (`az role assignment list`, `az ad user show`) before any write.
- Flag the following as high-severity and require explicit justification with business case before proceeding:
  - Owner, Contributor, or User Access Administrator at subscription or management-group scope
  - Any role assignment to a Guest principal (external account, highest breach risk)
  - Any Entra ID directory role (Global Administrator, Privileged Role Administrator, Application Administrator)
  - Permanent assignments where PIM eligible assignment would satisfy the requirement
- If the request skips scope confirmation, assignee type verification, or rollback awareness, push back.
- Never print access tokens, client secrets, tenant IDs, Object IDs without context, or raw environment dumps. Summarize sanitized evidence only.
- Load references only when needed.

## References

Load these only when needed:

- [Azure Entra and RBAC Role Assignment Operations](references/role-assignment-operations.md) — use for current service behavior, common failure modes, hard design rules, verification targets, and push-back conditions.
- [Preflight commands](references/preflight-commands.md) — CLI commands to run before any mutation.
- [Rollback playbook](references/rollback-playbook.md) — concrete rollback steps for this service.
- [Permission model](references/permission-model.md) — RBAC role definitions and PIM guidance.
- [MCP and evidence path](references/mcp-and-evidence.md) — use when choosing documentation-based evidence, sampled read-only evidence, or sanitized user evidence.
- [Safety checklist](references/safety-checklist.md) — use for evidence labels, permanent assignment blast radius, PIM eligibility, guest/service-principal risk, propagation delay, and rollback limits.
- [Workflow and output contract](references/workflow-and-output.md) — execution flow and final response contract.
- [Official sources](references/official-sources.md) — authoritative Azure documentation links.

## Response minimum

Return, at minimum:

- confirmed tenant, subscription (if applicable), target scope, and active caller identity
- preflight evidence: existing assignments on the target scope and current assignee roles
- principal-type risk classification (member user / guest / service principal / managed identity / group)
- role risk classification (Owner / Contributor / UAA / custom / narrow built-in)
- approval status and explicit justification for the assignment
- rollback posture: the exact `az role assignment delete` command to undo
- post-assignment verification steps or refusal reason
