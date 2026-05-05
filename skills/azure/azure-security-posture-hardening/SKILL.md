---
name: azure-security-posture-hardening
description: Use this skill for Azure security posture review, baseline hardening, managed identity adoption, Key Vault posture, private access decisions, Azure Policy guardrails, and logging or audit gap analysis. Trigger when the user asks how to harden an Azure workload or platform without defaulting to broad access or public exposure.
allowed-tools: Read Grep Glob
metadata:
  author: github: Raishin
  version: 0.1.0
  updated: "2026-05-05"
  category: security
---

# Azure Security Posture Hardening

## Purpose

Review and harden Azure platform or workload posture using operator-grade controls:

- least privilege,
- managed identities over stored secrets,
- private access where justified,
- Key Vault hardening,
- policy-enforced controls,
- audit and diagnostic coverage,
- staged remediation with rollout safety.

## When to use

Use this skill when the user asks for:

- Azure security baseline or posture review,
- managed identity migration guidance,
- Key Vault hardening or secret-handling critique,
- private endpoint or public exposure decisions for sensitive services,
- Azure Policy or Defender-backed hardening recommendations,
- logging, diagnostics, or auditability expectations for Azure security controls,
- zero-trust-oriented review of platform or workload controls.

Do not use this skill as a full compliance audit, incident forensics runbook, or a substitute for deep service-specific implementation docs.

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
