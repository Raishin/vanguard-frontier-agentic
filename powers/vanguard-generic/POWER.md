---
name: "vanguard-generic"
displayName: "Vanguard Frontier — Generic"
description: "Curated Generic review agents covering ci test pipeline, helm chart quality, kubernetes manifest quality, llm ai pipeline test. Reference agents directly under agents/generic/. Static review only; no live mutations."
keywords: ["test-quality", "ci-pipeline", "helm-chart", "manifest-review"]
author: "Raishin"
---
# Vanguard Frontier — Generic

Curated Generic review agents covering ci test pipeline, helm chart quality, kubernetes manifest quality, llm ai pipeline test. Reference agents directly under agents/generic/. Static review only; no live mutations.

## When to engage this Power

Activate when the task references Generic services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- *(no maestro for this provider; reference agents directly under `agents/generic/`)*

Use the maestro as the entry point: classify the task, then dispatch to one specialist or a parallel team of specialists. Never have the maestro itself execute a live mutation.

## Live-guard agents (gate_mode only)

- *(none — this provider has no live-mutation guards in the catalog)*

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Static review only -- agents analyze configuration and provide findings without mutating live systems.
- Agents are provider-agnostic and focus on CI, Helm, manifest, and test-quality patterns.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/Raishin/vanguard-frontier-agentic) marketplace. For this provider, see `agents/generic/` in that repository. 9 of 10 agents in this provider ship a Kiro adapter; the rest provide steering context only.

## Companion install paths

- **Claude Code:** `/plugin marketplace add Raishin/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider generic --repo .`
