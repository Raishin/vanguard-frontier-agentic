---
name: "vanguard-nvidia"
displayName: "Vanguard Frontier — NVIDIA"
description: "Curated NVIDIA agents for GPU resource governance, NIM model deployment, NGC registry hygiene, supply-chain integrity, and runtime evidence gating. Routes via nvidia-maestro to specialist agents and through the runtime-evidence-gate before runtime-affecting mutations. GPU resource changes require capacity, cost, and supply-chain review."
keywords: ["nvidia", "ngc", "nim", "gpu-governance", "runtime-evidence", "supply-chain"]
author: "VincentChuWaiChow"
---
# Vanguard Frontier — NVIDIA

Curated NVIDIA agents for GPU resource governance, NIM model deployment, NGC registry hygiene, supply-chain integrity, and runtime evidence gating. Routes via nvidia-maestro to specialist agents and through the runtime-evidence-gate before runtime-affecting mutations. GPU resource changes require capacity, cost, and supply-chain review.

## When to engage this Power

Activate when the task references NVIDIA, NGC, NIM, GPU, or CUDA. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- **`nvidia-maestro-agent`** — classifies and routes the task to the right specialist

Use the maestro as the entry point: classify the task, then dispatch to one specialist or a parallel team of specialists. Never have the maestro itself execute a live mutation.

## Live-guard agents (gate_mode only)

- *(none — this provider has no live-mutation guards in the catalog)*

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Runtime mutations require evidence via nvidia-runtime-evidence-gate before execution.
- GPU resource allocation must be reviewed for capacity, cost, and tenant isolation impact.
- NGC container provenance and SBOM must be validated before deployment to runtime hosts.
- Driver and CUDA version changes have node-wide blast radius — review compatibility matrix first.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/VincentChuWaiChow/vanguard-frontier-agentic) marketplace. For this provider, see `agents/nvidia/` in that repository. 11 of 12 agents in this provider ship a Kiro adapter; the rest provide steering context only.

## Companion install paths

- **Claude Code:** `/plugin marketplace add VincentChuWaiChow/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider nvidia --repo .`
