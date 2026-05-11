---
name: "vanguard-ovhcloud"
displayName: "Vanguard Frontier — OVHcloud"
description: "Curated OVHcloud agents for IAM, Managed Kubernetes, networking, and KMS with live-mutation guards. Routes via ovhcloud-maestro to specialist or live-guard agents. EU-headquartered sovereignty cloud; mutations require project ID and region confirmation."
keywords: ["ovhcloud", "ovh", "iam", "managed-kubernetes", "kms", "eu-sovereignty", "live-guard", "data-residency"]
author: "Raishin"
---
# Vanguard Frontier — OVHcloud

Curated OVHcloud agents for IAM, Managed Kubernetes, networking, and KMS with live-mutation guards. Routes via ovhcloud-maestro to specialist or live-guard agents. EU-headquartered sovereignty cloud; mutations require project ID and region confirmation.

## When to engage this Power

Activate when the task references OVHcloud services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- **`ovhcloud-maestro-agent`** — classifies and routes the task to the right specialist

Use the maestro as the entry point: classify the task, then dispatch to one specialist or a parallel team of specialists. Never have the maestro itself execute a live mutation.

## Live-guard agents (gate_mode only)

- `ovhcloud-live-kms-key-destruction-guard-agent` — never auto-dispatched; gate_mode only

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Confirm OVHcloud project ID and region before any live mutation.
- Live-guard agents (ovhcloud-live-*) must never be auto-dispatched; require approval and rollback plan.
- EU sovereignty cloud — review data-residency requirements before cross-region replication.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/Raishin/vanguard-frontier-agentic) marketplace. For this provider, see `agents/ovhcloud/` in that repository. All 6 agents in this provider ship a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add Raishin/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider ovhcloud --repo .`
