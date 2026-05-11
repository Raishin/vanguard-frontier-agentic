---
name: "vanguard-hetzner"
displayName: "Vanguard Frontier — Hetzner"
description: "Curated Hetzner agents for infrastructure review, cost optimization, capacity planning, and live server-lifecycle and firewall-rule guards. Routes via the Hetzner pattern to specialist agents. EU-headquartered provider; mutations on real Hetzner projects require project ID and region confirmation."
keywords: ["hetzner", "infrastructure-review", "cost-optimizer", "capacity-planner", "server-lifecycle", "firewall-rules", "live-guard"]
author: "Raishin"
---
# Vanguard Frontier — Hetzner

Curated Hetzner agents for infrastructure review, cost optimization, capacity planning, and live server-lifecycle and firewall-rule guards. Routes via the Hetzner pattern to specialist agents. EU-headquartered provider; mutations on real Hetzner projects require project ID and region confirmation.

## When to engage this Power

Activate when the task references Hetzner services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- **`hetzner-maestro-agent`** — classifies and routes the task to the right specialist

Use the maestro as the entry point: classify the task, then dispatch to one specialist or a parallel team of specialists. Never have the maestro itself execute a live mutation.

## Live-guard agents (gate_mode only)

- `hetzner-live-firewall-rule-guard-agent` — never auto-dispatched; gate_mode only
- `hetzner-live-server-lifecycle-guard-agent` — never auto-dispatched; gate_mode only

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Confirm Hetzner project ID and location before any live mutation.
- Live-guard agents (hetzner-live-*) must never be auto-dispatched; require approval and rollback plan.
- Firewall rule changes require capture of current ruleset and explicit egress-blocking review.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/Raishin/vanguard-frontier-agentic) marketplace. For this provider, see `agents/hetzner/` in that repository. All 6 agents in this provider ship a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add Raishin/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider hetzner --repo .`
