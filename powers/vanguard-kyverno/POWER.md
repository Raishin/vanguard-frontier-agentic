---
name: "vanguard-kyverno"
displayName: "Vanguard Frontier — Kyverno"
description: "Reviews kyverno ClusterPolicy and Policy resources for failureAction, background scanning, PolicyException audit, mutate/gene... Static review only; no live mutations."
keywords: ["kyverno", "admission-policy", "cluster-policy", "policy-enforcement"]
author: "Raishin"
---
# Vanguard Frontier — Kyverno

Reviews kyverno ClusterPolicy and Policy resources for failureAction, background scanning, PolicyException audit, mutate/gene... Static review only; no live mutations.

## When to engage this Power

Activate when the task references Kyverno services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- *(no maestro for this provider; reference agents directly under `agents/kyverno/`)*

Use the maestro as the entry point: classify the task, then dispatch to one specialist or a parallel team of specialists. Never have the maestro itself execute a live mutation.

## Live-guard agents (gate_mode only)

- *(none — this provider has no live-mutation guards in the catalog)*

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Static review only -- agents analyze configuration and provide findings without mutating live systems.
- Cluster-scoped policies can reject legitimate workloads; validate against existing deployments before applying.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/Raishin/vanguard-frontier-agentic) marketplace. For this provider, see `agents/kyverno/` in that repository. All 1 agents in this provider ship a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add Raishin/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider kyverno --repo .`
