---
name: "vanguard-opentelemetry"
displayName: "Vanguard Frontier — OpenTelemetry"
description: "Reviews openTelemetry Collector pipeline configuration — receiver/processor/exporter ordering, memory_limiter placement,... Static review only; no live mutations."
keywords: ["opentelemetry", "otel-collector", "tracing", "observability-pipeline"]
author: "Raishin"
---
# Vanguard Frontier — OpenTelemetry

Reviews openTelemetry Collector pipeline configuration — receiver/processor/exporter ordering, memory_limiter placement,... Static review only; no live mutations.

## When to engage this Power

Activate when the task references OpenTelemetry services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- *(no maestro for this provider; reference agents directly under `agents/opentelemetry/`)*

Reference agents directly from agents/opentelemetry/ without maestro-based routing.

## Live-guard agents (gate_mode only)

- *(none — this provider has no live-mutation guards in the catalog)*

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Static review only -- agents analyze configuration and provide findings without mutating live systems.
- Collector pipeline changes affect observability for all instrumented services; review cardinality impact.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/Raishin/vanguard-frontier-agentic) marketplace. For this provider, see `agents/opentelemetry/` in that repository. The single agent in this provider ships a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add Raishin/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider opentelemetry --repo .`
