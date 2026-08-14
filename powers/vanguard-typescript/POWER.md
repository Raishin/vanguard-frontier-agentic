---
name: "vanguard-typescript"
displayName: "Vanguard Frontier — TypeScript"
description: "Curated TypeScript agents for the TypeScript program and the published package — type soundness in shared code, runtime boundary contracts, module resolution and emit, Node execution compatibility, declaration governance, build-graph cost, static enforcement policy, async contract reliability, publication integrity, estate modernization, and MCP tool contracts. Routes via typescript-maestro to specialist agents based on task scope. Static review only; no live mutations."
keywords: ["typescript", "tsconfig", "tsc", "declaration-emit", "module-resolution", "type-safety", "npm-publish", "static-review"]
author: "VincentChuWaiChow"
---
# Vanguard Frontier — TypeScript

Curated TypeScript agents for the TypeScript program and the published package — type soundness in shared code, runtime boundary contracts, module resolution and emit, Node execution compatibility, declaration governance, build-graph cost, static enforcement policy, async contract reliability, publication integrity, estate modernization, and MCP tool contracts. Routes via typescript-maestro to specialist agents based on task scope. Static review only; no live mutations.

## When to engage this Power

Activate when the task references TypeScript services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- **`typescript-maestro-agent`** — classifies and routes the task to the right specialist

Use the maestro as the entry point: classify the task, then dispatch to one specialist or a parallel team of specialists. Never have the maestro itself execute a live mutation.

## Live-guard agents (gate_mode only)

- *(none — this provider has no live-mutation guards in the catalog)*

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Static review only — agents read source and sanitized configuration, and never compile, build, test, publish, deploy, sign, or contact a live system.
- Never request or accept secrets, npm or registry tokens, signing keys, connection strings, tenant identifiers, or customer data.
- This repository contains no TypeScript program of its own, so no verdict may be grounded in an assumed compiler, tsconfig, or Node version — a version-gated conclusion requires evidence the user supplies, and its absence is a refuse-and-ask.
- Compile-time is not runtime: types are erased, so a passing build is evidence about the source and never about the payload crossing an I/O boundary.
- Frontend application diffs, framework specifics, bundler configuration, dependency intake, and the monorepo task graph belong to the frontend board — hand them off rather than absorbing them.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/VincentChuWaiChow/vanguard-frontier-agentic) marketplace. For this provider, see `agents/typescript/` in that repository. All 14 agents in this provider ship a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add VincentChuWaiChow/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider typescript --repo .`
