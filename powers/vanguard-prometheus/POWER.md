---
name: "vanguard-prometheus"
displayName: "Vanguard Frontier — Prometheus"
description: "Reviews prometheus and AlertManager configuration for cardinality risks, alert correctness, scrape security, routing safety,... Static review only; no live mutations."
keywords: ["prometheus", "alertmanager", "metrics-cardinality", "scrape-config"]
author: "Raishin"
---
# Vanguard Frontier — Prometheus

Reviews prometheus and AlertManager configuration for cardinality risks, alert correctness, scrape security, routing safety,... Static review only; no live mutations.

## When to engage this Power

Activate when the task references Prometheus services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- *(no maestro for this provider; reference agents directly under `agents/prometheus/`)*

Reference agents directly from agents/prometheus/ without maestro-based routing.

## Live-guard agents (gate_mode only)

- *(none — this provider has no live-mutation guards in the catalog)*

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Static review only -- agents analyze configuration and provide findings without mutating live systems.
- Alerting rule and scrape config changes affect monitoring coverage; review for metric-name collisions.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/Raishin/vanguard-frontier-agentic) marketplace. For this provider, see `agents/prometheus/` in that repository. The single agent in this provider ships a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add Raishin/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider prometheus --repo .`
