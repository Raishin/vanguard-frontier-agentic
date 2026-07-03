---
name: "vanguard-frontend"
displayName: "Vanguard Frontier — Frontend"
description: "Curated frontend and web development agents for component architecture, accessibility, performance, testing, and security posture — static review only, no live builds, deploys, or dependency mutations. Routes via frontend-maestro to specialist agents. Framework, bundler, and testing-tool surfaces are drift-prone; agents always verify against current official documentation before rendering findings."
keywords: ["frontend", "web", "react", "vue", "accessibility", "performance", "testing", "static-review"]
author: "Raishin"
---
# Vanguard Frontier — Frontend

Curated frontend and web development agents for component architecture, accessibility, performance, testing, and security posture — static review only, no live builds, deploys, or dependency mutations. Routes via frontend-maestro to specialist agents. Framework, bundler, and testing-tool surfaces are drift-prone; agents always verify against current official documentation before rendering findings.

## When to engage this Power

Activate when the task references Frontend services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- **`frontend-maestro-agent`** — classifies and routes the task to the right specialist

Use the maestro as the entry point: classify the task, then dispatch to one specialist or a parallel team of specialists. Never have the maestro itself execute a live mutation.

## Live-guard agents (gate_mode only)

- *(none — this provider has no live-mutation guards in the catalog)*

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Static review only — agents never request API keys, auth tokens, or customer data, and never run live build, deploy, or dependency-mutation commands.
- Route all tasks through frontend-maestro for proper classification and dispatch to specialist agents.
- Review covers framework/component architecture, accessibility (WCAG) compliance, performance budgets, test coverage, and supply-chain integrity.
- Production-impacting actions (deploys, dependency upgrades, build-config changes) are live-guard gated — never auto-dispatched; require explicit approval and rollback plan.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/Raishin/vanguard-frontier-agentic) marketplace. For this provider, see `agents/frontend/` in that repository. All 35 agents in this provider ship a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add Raishin/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider frontend --repo .`
