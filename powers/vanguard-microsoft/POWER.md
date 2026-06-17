---
name: "vanguard-microsoft"
displayName: "Vanguard Frontier — Microsoft"
description: "Curated Microsoft 365 and Dynamics 365 agents for tenant governance, Entra identity and Conditional Access, Intune endpoints, Purview data security and compliance, Defender XDR, Teams/SharePoint/Exchange collaboration, Microsoft 365 Copilot readiness, Power Platform governance, and Dynamics 365 ERP/CRM (Finance, Supply Chain, Business Central, Sales, Customer Service, Field Service) — static review only, no tenant or production mutations. Routes via microsoft-maestro to M365, D365, Power Platform, and Copilot specialist agents. Microsoft licensing, certification, and API surfaces are drift-prone; agents always verify against current Microsoft Learn documentation before rendering findings."
keywords: ["microsoft", "m365", "d365", "entra", "purview", "copilot", "power-platform", "static-review"]
author: "Raishin"
---
# Vanguard Frontier — Microsoft

Curated Microsoft 365 and Dynamics 365 agents for tenant governance, Entra identity and Conditional Access, Intune endpoints, Purview data security and compliance, Defender XDR, Teams/SharePoint/Exchange collaboration, Microsoft 365 Copilot readiness, Power Platform governance, and Dynamics 365 ERP/CRM (Finance, Supply Chain, Business Central, Sales, Customer Service, Field Service) — static review only, no tenant or production mutations. Routes via microsoft-maestro to M365, D365, Power Platform, and Copilot specialist agents. Microsoft licensing, certification, and API surfaces are drift-prone; agents always verify against current Microsoft Learn documentation before rendering findings.

## When to engage this Power

Activate when the task references Microsoft services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- **`copilot-governance-maestro-agent`** — classifies and routes the task to the right specialist

Use the maestro as the entry point: classify the task, then dispatch to one specialist or a parallel team of specialists. Never have the maestro itself execute a live mutation.

## Live-guard agents (gate_mode only)

- *(none — this provider has no live-mutation guards in the catalog)*

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Static review only — agents never request tenant credentials, tokens, customer data, or PII, and never mutate a Microsoft 365 tenant or Dynamics 365 environment.
- Apply Zero Trust by default: verify explicitly, least privilege (JIT/JEA), assume breach; confirm tenant, environment, and data classification before any recommendation.
- Microsoft 365 Copilot and Copilot Studio configurations are adversarially reviewed for oversharing, ungrounded Graph exposure, and missing human-handoff controls before any approve decision.
- Production-impacting actions (Conditional Access changes, D365 cutover, Power Platform prod deploy, MFA changes) are live-guard gated — never auto-dispatched; require explicit approval, scope confirmation, and rollback plan.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/Raishin/vanguard-frontier-agentic) marketplace. For this provider, see `agents/microsoft/` in that repository. All 21 agents in this provider ship a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add Raishin/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider microsoft --repo .`
