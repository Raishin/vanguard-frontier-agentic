---
name: "vanguard-hr"
displayName: "Vanguard Frontier — HR"
description: "Curated HR agents for analytics people data, benefits payroll, compensation equity, culture dei. Routes via hr-maestro-agent to specialist agents based on task scope. Static review only; no live mutations."
keywords: ["hr-governance", "employment-risk", "compensation-equity", "recruiting"]
author: "Raishin"
---
# Vanguard Frontier — HR

Curated HR agents for analytics people data, benefits payroll, compensation equity, culture dei. Routes via hr-maestro-agent to specialist agents based on task scope. Static review only; no live mutations.

## When to engage this Power

Activate when the task references HR services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- **`hr-maestro-agent`** — classifies and routes the task to the right specialist

Use the maestro as the entry point: classify the task, then dispatch to one specialist or a parallel team of specialists. Never have the maestro itself execute a live mutation.

## Live-guard agents (gate_mode only)

- *(none — this provider has no live-mutation guards in the catalog)*

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Route all tasks through hr-maestro-agent for proper classification and dispatch.
- Static review only -- agents analyze configuration and provide findings without mutating live systems.
- All findings must respect employee privacy and data-minimization principles.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/Raishin/vanguard-frontier-agentic) marketplace. For this provider, see `agents/hr/` in that repository. All 15 agents in this provider ship a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add Raishin/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider hr --repo .`
