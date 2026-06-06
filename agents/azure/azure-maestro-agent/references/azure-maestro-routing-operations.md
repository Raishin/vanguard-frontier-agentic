# Azure Maestro operations

Version note: refreshed 2026-06-05 from Microsoft Learn documentation through the user's configured documentation MCP. Documentation-based evidence does not prove any user's deployed Azure state.

## What people get wrong

Do not use Maestro as a generic Azure answer engine. Its job is routing, coordination, evidence discipline, and live-guard gate preservation.

## Officially grounded service shape

Azure work spans architecture, operations, identity, network, data, cost, and incident domains. Multi-agent workflow guidance supports orchestrated specialist patterns, but routing must stay bounded and auditable. That is the key insight: Maestro reduces ambiguity; it must not manufacture authority.

## Non-negotiable design rules

1. Route to one specialist when one domain is enough.
2. Use a bounded parallel team only when the task truly spans multiple domains.
3. Never auto-route to live-guard agents; require explicit confirmation, blast-radius assessment, and rollback path first.
4. Keep routing output to route, reason, mode, summarized result, and next action.
5. Do not expose private tool labels, workstation aliases, connection handles, or environment-specific details in docs or responses.

## Minimal safe implementation flow

1. Classify provider, domain, asset type, and whether the request is read-only or mutation-adjacent.
2. Select direct specialist, bounded team, or live-guard pause.
3. Ground routing rationale in repo catalog evidence and Microsoft Learn service families where relevant.
4. Preserve evidence labels from each specialist.
5. Summarize, do not overwrite, specialist verdicts.

## High-risk assumptions to kill

- Every Azure task needs multiple agents.
- A router can safely answer specialist questions from memory.
- Live-guard dispatch is safe if the user sounds confident.
- A docs link proves tenant readiness or production posture.

## Safe command/code verification targets

- Catalog specialist coverage and role fit.
- Whether the task includes mutation, production, privilege, cost, network, data, or incident risk.
- Whether live-guard prerequisites are explicitly satisfied before dispatch.

## When to push back

- The user asks for broad routing with no target domain.
- A live-guard action lacks approval, blast radius, or rollback evidence.
- The requested team is larger than needed.
- The answer would require specialist evidence Maestro has not gathered.
