---
name: azure-role-selector
description: Use this skill when the user asks which Azure role to assign, how to grant minimum access, whether a built-in role is sufficient, or when a custom role may be required.
metadata:
  author: github: Raishin
---

# Azure Role Selector

## Purpose

Select the narrowest Azure role and assignment scope that satisfies the requested access without defaulting to broad standing privilege.

## When to use

Use this skill when the user needs to:

- map requested Azure operations to a role,
- grant minimum access to a user, group, service principal, managed identity, or workload identity,
- decide whether a built-in role is enough,
- separate control-plane permissions from data-plane permissions,
- decide whether a custom role is justified,
- choose the safest assignment scope and validation path.

Do not use this skill for tenant-wide governance design, access review programs, or broad RBAC posture critique. Route those asks toward `azure-rbac-review` or a governance-focused skill.

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
