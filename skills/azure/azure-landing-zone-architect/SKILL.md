---
name: azure-landing-zone-architect
description: Use this skill for Azure landing-zone design, management-group and subscription hierarchy reviews, platform-versus-application boundary decisions, or multi-subscription Azure platform architecture critiques that span governance, identity, networking, security, and operations.
allowed-tools: Read Grep Glob
metadata:
  author: github: Raishin
  version: 0.1.0
  updated: "2026-05-05"
  category: compliance
---

# Azure Landing Zone Architect

## Purpose

Design or review Azure landing zones with an operator-grade focus on structure, dependencies, and blast radius.

This skill is for platform decisions that cut across:

- management groups,
- subscriptions,
- platform versus application landing zones,
- identity and access boundaries,
- network topology and shared services,
- governance and policy inheritance,
- security baselines,
- management, monitoring, backup, and recovery posture.

## When to use

Use this skill when the user asks for:

- a greenfield Azure landing-zone design,
- a brownfield hierarchy or subscription-placement critique,
- shared-services or platform-subscription layout advice,
- a hub-spoke or alternative connectivity decision in landing-zone context,
- a review of whether governance, security, and operations dependencies were missed,
- clarification of platform-team versus application-team ownership boundaries.

Do not use this skill for:

- narrow RBAC assignment questions with no platform-design component,
- single-service implementation tutorials,
- writing production Bicep or Terraform on first pass,
- workload-only design questions that do not affect the platform operating model.

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
