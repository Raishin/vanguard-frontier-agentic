---
name: "vanguard-falco"
displayName: "Vanguard Frontier — Falco"
description: "Reviews falco rules for macro correctness, exception blast radius, sensitive-path coverage, K8s audit gaps, and alert output... Static review only; no live mutations."
keywords: ["falco", "runtime-threat", "syscall-rules", "container-security"]
author: "VincentChuWaiChow"
---
# Vanguard Frontier — Falco

Reviews falco rules for macro correctness, exception blast radius, sensitive-path coverage, K8s audit gaps, and alert output... Static review only; no live mutations.

## When to engage this Power

Activate when the task references Falco services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- *(no maestro for this provider; reference agents directly under `agents/falco/`)*

Reference agents directly from agents/falco/ without maestro-based routing.

## Live-guard agents (gate_mode only)

- *(none — this provider has no live-mutation guards in the catalog)*

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Static review only -- agents analyze configuration and provide findings without mutating live systems.
- Rule changes must be evaluated for false-positive rate impact on production alerting.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/VincentChuWaiChow/vanguard-frontier-agentic) marketplace. For this provider, see `agents/falco/` in that repository. The single agent in this provider ships a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add VincentChuWaiChow/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider falco --repo .`
