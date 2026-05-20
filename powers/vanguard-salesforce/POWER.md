---
name: "vanguard-salesforce"
displayName: "Vanguard Frontier — Salesforce"
description: "Curated Salesforce agents for admin review, development, security, integration, revenue ops, service ops, marketing ops, Agentforce/AI risk, and compliance — static review only, no org mutations. Routes via salesforce-maestro to specialist agents covering Sales Cloud, Service Cloud, Experience Cloud, Marketing Cloud, MuleSoft, Tableau, and industry verticals. All Salesforce terminology and API surfaces are drift-prone; agents always verify against current official documentation before rendering findings."
keywords: ["salesforce", "agentforce", "crm", "apex", "lwc", "mulesoft", "compliance", "static-review"]
author: "Raishin"
---
# Vanguard Frontier — Salesforce

Curated Salesforce agents for admin review, development, security, integration, revenue ops, service ops, marketing ops, Agentforce/AI risk, and compliance — static review only, no org mutations. Routes via salesforce-maestro to specialist agents covering Sales Cloud, Service Cloud, Experience Cloud, Marketing Cloud, MuleSoft, Tableau, and industry verticals. All Salesforce terminology and API surfaces are drift-prone; agents always verify against current official documentation before rendering findings.

## When to engage this Power

Activate when the task references Salesforce services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- **`salesforce-maestro-agent`** — classifies and routes the task to the right specialist

Use the maestro as the entry point: classify the task, then dispatch to one specialist or a parallel team of specialists. Never have the maestro itself execute a live mutation.

## Live-guard agents (gate_mode only)

- `salesforce-live-guard-agent` — never auto-dispatched; gate_mode only

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Static review only — agents never request org credentials, session tokens, or user PII, and never mutate a Salesforce org.
- Salesforce API versions and feature availability vary by org edition and release; verify org context (edition, API version, enabled features) before applying any recommendation.
- Agentforce and Einstein AI configurations are adversarially reviewed for prompt-injection risk, ungrounded automation, and missing human-handoff controls before any approve-or-merge decision.
- Live-guard agent (salesforce-live-guard-agent) must never be auto-dispatched; require explicit approval, target org confirmation, and rollback plan.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/Raishin/vanguard-frontier-agentic) marketplace. For this provider, see `agents/salesforce/` in that repository. All 20 agents in this provider ship a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add Raishin/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider salesforce --repo .`
