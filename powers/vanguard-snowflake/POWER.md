---
name: "vanguard-snowflake"
displayName: "Vanguard Frontier — Snowflake (Azure)"
description: "Curated Snowflake-on-Azure agents for RBAC access governance and data-platform engineering — static review only, no account or production mutations. Covers role hierarchy and least privilege, ACCOUNTADMIN restriction, SECURITYADMIN/SYSADMIN separation of duties, future grants and managed-access schemas, network policies, key-pair/Entra OAuth/SSO/SCIM authentication, Azure Private Link and storage integration, and masking/row-access/tagging governance. Snowflake and Azure surfaces are drift-prone; agents always verify against current Snowflake and Microsoft Learn documentation before rendering findings."
keywords: ["snowflake", "azure", "rbac", "least-privilege", "data-governance", "private-link", "static-review"]
author: "Raishin"
---
# Vanguard Frontier — Snowflake (Azure)

Curated Snowflake-on-Azure agents for RBAC access governance and data-platform engineering — static review only, no account or production mutations. Covers role hierarchy and least privilege, ACCOUNTADMIN restriction, SECURITYADMIN/SYSADMIN separation of duties, future grants and managed-access schemas, network policies, key-pair/Entra OAuth/SSO/SCIM authentication, Azure Private Link and storage integration, and masking/row-access/tagging governance. Snowflake and Azure surfaces are drift-prone; agents always verify against current Snowflake and Microsoft Learn documentation before rendering findings.

## When to engage this Power

Activate when the task references Snowflake (Azure) services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- *(no maestro for this provider; reference agents directly under `agents/snowflake/`)*

Reference agents directly from agents/snowflake/ without maestro-based routing.

## Live-guard agents (gate_mode only)

- `snowflake-live-rbac-grant-guard-at-azure-agent` — never auto-dispatched; gate_mode only

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Static review only — agents never request account credentials, key-pair private keys, OAuth secrets, or customer data, and never mutate a Snowflake account or Azure resource.
- Enforce least privilege: custom business-function roles under SYSADMIN, restrict ACCOUNTADMIN to a minimum of controlled users, never grant sensitive privileges to PUBLIC, separate SECURITYADMIN (grants) from SYSADMIN (objects).
- Service accounts use key-pair or Entra OAuth (never passwords); enforce network policies and MFA for human users.
- Production role/grant/policy/warehouse changes are live-guard gated — never auto-dispatched; require explicit approval, scope confirmation, and rollback plan.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/Raishin/vanguard-frontier-agentic) marketplace. For this provider, see `agents/snowflake/` in that repository. All 3 agents in this provider ship a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add Raishin/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider snowflake --repo .`
