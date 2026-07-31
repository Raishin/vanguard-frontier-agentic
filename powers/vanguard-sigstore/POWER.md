---
name: "vanguard-sigstore"
displayName: "Vanguard Frontier — Sigstore"
description: "Reviews cosign image signing, Kyverno imageVerify policy identity constraints, SBOM and SLSA provenance attestations, Rekor... Static review only; no live mutations."
keywords: ["sigstore", "cosign", "supply-chain-integrity", "image-signing"]
author: "VincentChuWaiChow"
---
# Vanguard Frontier — Sigstore

Reviews cosign image signing, Kyverno imageVerify policy identity constraints, SBOM and SLSA provenance attestations, Rekor... Static review only; no live mutations.

## When to engage this Power

Activate when the task references Sigstore services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- *(no maestro for this provider; reference agents directly under `agents/sigstore/`)*

Reference agents directly from agents/sigstore/ without maestro-based routing.

## Live-guard agents (gate_mode only)

- *(none — this provider has no live-mutation guards in the catalog)*

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Static review only -- agents analyze configuration and provide findings without mutating live systems.
- Supply-chain policy changes can block valid deployments; verify cosign keyless trust roots before enforcement.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/VincentChuWaiChow/vanguard-frontier-agentic) marketplace. For this provider, see `agents/sigstore/` in that repository. The single agent in this provider ships a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add VincentChuWaiChow/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider sigstore --repo .`
