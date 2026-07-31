---
name: "vanguard-cilium"
displayName: "Vanguard Frontier — Cilium"
description: "Reviews cilium CiliumNetworkPolicy, CiliumClusterwideNetworkPolicy, standard NetworkPolicy, ClusterMesh cross-cluster policy... Static review only; no live mutations."
keywords: ["cilium", "network-policy", "ebpf", "cluster-mesh"]
author: "VincentChuWaiChow"
---
# Vanguard Frontier — Cilium

Reviews cilium CiliumNetworkPolicy, CiliumClusterwideNetworkPolicy, standard NetworkPolicy, ClusterMesh cross-cluster policy... Static review only; no live mutations.

## When to engage this Power

Activate when the task references Cilium services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- *(no maestro for this provider; reference agents directly under `agents/cilium/`)*

Reference agents directly from agents/cilium/ without maestro-based routing.

## Live-guard agents (gate_mode only)

- *(none — this provider has no live-mutation guards in the catalog)*

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Static review only -- agents analyze configuration and provide findings without mutating live systems.
- Network policies must be reviewed for unintended traffic blocking across namespaces and cluster-mesh endpoints.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/VincentChuWaiChow/vanguard-frontier-agentic) marketplace. For this provider, see `agents/cilium/` in that repository. The single agent in this provider ships a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add VincentChuWaiChow/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider cilium --repo .`
