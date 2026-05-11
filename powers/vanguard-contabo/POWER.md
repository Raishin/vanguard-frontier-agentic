---
name: "vanguard-contabo"
displayName: "Vanguard Frontier — Contabo"
description: "Curated Contabo agents for security hardening, cost optimization, capacity planning, and live instance-lifecycle and storage-operations guards. Routes via the Contabo pattern to specialist agents. Mutations on real Contabo accounts require account context and region confirmation."
keywords: ["contabo", "security-hardening", "cost-optimizer", "capacity-planner", "instance-lifecycle", "storage-operations", "live-guard"]
author: "Raishin"
---
# Vanguard Frontier — Contabo

Curated Contabo agents for security hardening, cost optimization, capacity planning, and live instance-lifecycle and storage-operations guards. Routes via the Contabo pattern to specialist agents. Mutations on real Contabo accounts require account context and region confirmation.

## When to engage this Power

Activate when the task references Contabo services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- **`contabo-maestro-agent`** — classifies and routes the task to the right specialist

Use the maestro as the entry point: classify the task, then dispatch to one specialist or a parallel team of specialists. Never have the maestro itself execute a live mutation.

## Live-guard agents (gate_mode only)

- `contabo-live-instance-lifecycle-guard-agent` — never auto-dispatched; gate_mode only
- `contabo-live-storage-operations-guard-agent` — never auto-dispatched; gate_mode only

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Confirm Contabo account context and region before any live mutation.
- Live-guard agents (contabo-live-*) must never be auto-dispatched; require approval and rollback plan.
- Storage operations on object storage and block storage require backup verification before destructive actions.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/Raishin/vanguard-frontier-agentic) marketplace. For this provider, see `agents/contabo/` in that repository. This provider's 6 agents do not yet ship Kiro adapters — this Power supplies steering content only. Use `npx vfa-export-agents --platform kiro --provider contabo` from the npm package once Kiro adapters land.

## Companion install paths

- **Claude Code:** `/plugin marketplace add Raishin/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider contabo --repo .`
