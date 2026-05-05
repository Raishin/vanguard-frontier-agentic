---
name: azure-rbac-review
description: Use this skill for Azure RBAC, Entra-backed access, role assignment, custom role, scope, subscription, management group, or least-privilege review tasks. Trigger when the user asks whether Azure access is too broad or how to grant access safely.
allowed-tools: Read Grep Glob
metadata:
  author: github: Raishin
  version: 0.1.0
---

# Azure RBAC Review

## Purpose

Review Azure access decisions against least privilege, scope minimization, and operational safety.

## Lean operating rules

- Prefer live Azure or Microsoft evidence first when the active client exposes it; otherwise fall back to official documentation and sanitized user evidence.
- Separate confirmed facts from inference. If state was not queried or shown, say so.
- Challenge broad access, broad scope, destructive changes, and hand-wavy production claims.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.

## References

Load these only when needed:

- [MCP and evidence path](references/mcp-and-evidence.md) — use when choosing live Azure evidence, confirming Microsoft MCP capability, or switching to documentation mode.
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review, applying stress checks, or formatting the final answer.
- [Official sources](references/official-sources.md) — use when you need the detailed Microsoft documentation list or source notes.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the main risks or control gaps,
- the safest next actions,
- the assumptions or blockers that prevent stronger conclusions.
