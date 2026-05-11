---
name: "vanguard-ionos"
displayName: "Vanguard Frontier — IONOS"
description: "Curated IONOS agents for security and compliance review, datacenter design, cost optimization, Managed Kubernetes operations, and live database-lifecycle guards. Routes via ionos-maestro to specialist or live-guard agents. Mutations require contract ID and datacenter confirmation."
keywords: ["ionos", "security-compliance", "datacenter-designer", "managed-kubernetes", "database-lifecycle", "live-guard", "eu-sovereignty"]
author: "Raishin"
---
# Vanguard Frontier — IONOS

Curated IONOS agents for security and compliance review, datacenter design, cost optimization, Managed Kubernetes operations, and live database-lifecycle guards. Routes via ionos-maestro to specialist or live-guard agents. Mutations require contract ID and datacenter confirmation.

## When to engage this Power

Activate when the task references IONOS services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- **`ionos-maestro-agent`** — classifies and routes the task to the right specialist

Use the maestro as the entry point: classify the task, then dispatch to one specialist or a parallel team of specialists. Never have the maestro itself execute a live mutation.

## Live-guard agents (gate_mode only)

- `ionos-live-database-lifecycle-guard-agent` — never auto-dispatched; gate_mode only

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Confirm IONOS contract ID and datacenter before any live mutation.
- Live-guard agents (ionos-live-*) must never be auto-dispatched; require approval and rollback plan.
- DBaaS lifecycle mutations require backup verification and replication-status review.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/Raishin/vanguard-frontier-agentic) marketplace. For this provider, see `agents/ionos/` in that repository. All 6 agents in this provider ship a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add Raishin/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider ionos --repo .`
