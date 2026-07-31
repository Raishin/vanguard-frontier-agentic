---
name: "vanguard-netsuite"
displayName: "Vanguard Frontier — Netsuite"
description: "Curated Netsuite agents for netsuite administrator, netsuite ai connector mcp, netsuite ai foundations, netsuite application developer. Routes via netsuite-maestro-agent to specialist or live-guard agents based on task scope. Live-mutation agents require approval, target confirmation, evidence capture, and a rollback plan; static specialists never mutate."
keywords: ["netsuite", "static-review", "configuration-audit", "best-practices"]
author: "VincentChuWaiChow"
---
# Vanguard Frontier — Netsuite

Curated Netsuite agents for netsuite administrator, netsuite ai connector mcp, netsuite ai foundations, netsuite application developer. Routes via netsuite-maestro-agent to specialist or live-guard agents based on task scope. Live-mutation agents require approval, target confirmation, evidence capture, and a rollback plan; static specialists never mutate.

## When to engage this Power

Activate when the task references Netsuite services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- **`netsuite-maestro-agent`** — classifies and routes the task to the right specialist

Use the maestro as the entry point: classify the task, then dispatch to one specialist or a parallel team of specialists. Never have the maestro itself execute a live mutation.

## Live-guard agents (gate_mode only)

- `netsuite-live-org-mutation-guard-agent` — never auto-dispatched; gate_mode only

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Live-guard agents (the mutating-runtime operators) must never be auto-dispatched; require explicit approval, evidence capture, and a rollback plan. Read-only-runtime and static-review agents on this board are not guards.
- Route all tasks through netsuite-maestro-agent for proper classification and dispatch.
- Mixed-tier board: static specialists analyze configuration without mutating live systems; live-guard agents mutate only under approval, target confirmation, evidence capture, and a pre-approved rollback plan.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/VincentChuWaiChow/vanguard-frontier-agentic) marketplace. For this provider, see `agents/netsuite/` in that repository. All 25 agents in this provider ship a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add VincentChuWaiChow/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider netsuite --repo .`
