---
name: "vanguard-marketing"
displayName: "Vanguard Frontier — Marketing"
description: "Curated Marketing agents for ai advertising targeting fairness, analytics data minimization, email sender authentication, eu ai act marketing system. Routes via marketing-maestro-agent to specialist agents based on task scope. Static review only; no live mutations."
keywords: ["marketing-governance", "consent-compliance", "advertising-fairness", "email-authentication"]
author: "Raishin"
---
# Vanguard Frontier — Marketing

Curated Marketing agents for ai advertising targeting fairness, analytics data minimization, email sender authentication, eu ai act marketing system. Routes via marketing-maestro-agent to specialist agents based on task scope. Static review only; no live mutations.

## When to engage this Power

Activate when the task references Marketing services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- **`marketing-maestro-agent`** — classifies and routes the task to the right specialist

Use the maestro as the entry point: classify the task, then dispatch to one specialist or a parallel team of specialists. Never have the maestro itself execute a live mutation.

## Live-guard agents (gate_mode only)

- *(none — this provider has no live-mutation guards in the catalog)*

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Route all tasks through marketing-maestro-agent for proper classification and dispatch.
- Static review only -- agents analyze configuration and provide findings without mutating live systems.
- Review covers consent, privacy, fairness, and regulatory compliance for marketing systems.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/Raishin/vanguard-frontier-agentic) marketplace. For this provider, see `agents/marketing/` in that repository. All 14 agents in this provider ship a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add Raishin/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider marketing --repo .`
