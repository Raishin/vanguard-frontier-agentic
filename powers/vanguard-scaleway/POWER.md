---
name: "vanguard-scaleway"
displayName: "Vanguard Frontier — Scaleway"
description: "Curated Scaleway agents for IAM, Kapsule (managed Kubernetes), networking, and cost optimization with live-mutation guards. Routes via scaleway-maestro to specialist or live-guard agents. EU-region only (PAR, AMS, WAW); mutations require organization ID and region confirmation."
keywords: ["scaleway", "iam", "kapsule", "managed-kubernetes", "cost-optimizer", "eu-region", "live-guard"]
author: "Raishin"
---
# Vanguard Frontier — Scaleway

Curated Scaleway agents for IAM, Kapsule (managed Kubernetes), networking, and cost optimization with live-mutation guards. Routes via scaleway-maestro to specialist or live-guard agents. EU-region only (PAR, AMS, WAW); mutations require organization ID and region confirmation.

## When to engage this Power

Activate when the task references Scaleway services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- **`scaleway-maestro-agent`** — classifies and routes the task to the right specialist

Use the maestro as the entry point: classify the task, then dispatch to one specialist or a parallel team of specialists. Never have the maestro itself execute a live mutation.

## Live-guard agents (gate_mode only)

- `scaleway-live-kapsule-rollout-guard-agent` — never auto-dispatched; gate_mode only

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Confirm Scaleway organization ID and region (PAR, AMS, WAW) before any live mutation.
- Live-guard agents (scaleway-live-*) must never be auto-dispatched; require approval and rollback plan.
- Kapsule rollout changes require PDB audit and health-signal verification.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/Raishin/vanguard-frontier-agentic) marketplace. For this provider, see `agents/scaleway/` in that repository. All 6 agents in this provider ship a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add Raishin/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider scaleway --repo .`
