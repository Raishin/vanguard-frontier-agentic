# MCP and evidence path

Use this reference only when you need to decide how to gather Entra evidence.

## Live-first evidence rule

1. Prefer live Azure MCP capability evidence when the active client exposes Azure tools.
2. Treat the runtime-exposed tool inventory as truth.
3. If Entra-specific tooling is not exposed live, say so and switch to documentation-based guidance instead of pretending the namespace exists.

## Azure MCP grounding

Based on Microsoft documentation:

- Microsoft recommends **consolidated mode** for AI agents because it reduces tool count and improves usability.
- Namespace filtering means a client may expose only a subset of Azure tools.
- Do not assume that the active client exposes Entra-specific operations just because Microsoft documents broader Azure MCP capabilities.

## Evidence hierarchy

Use this order:

1. **live evidence** — Azure MCP output, sanitized screenshots, sanitized policy exports, sanitized logs, or user-provided config excerpts
2. **user-provided sanitized evidence** — redacted CA policy summaries, app-registration details, audit logs, sign-in logs, risk events, or screenshots
3. **documentation-based** — Microsoft Learn and official Azure MCP documentation
4. **inference** — conclusions derived from patterns but not directly proven by evidence

## Entra caution points

- Do not bless Conditional Access exclusions without explicit break-glass and recovery logic.
- Do not assume MFA means safe posture if registration, authentication methods, or legacy paths are weak.
- Do not confuse identity governance controls with full Entra ID security posture.
- Do not assume workload identities, service principals, or app registrations are low-risk just because they are nonhuman.
