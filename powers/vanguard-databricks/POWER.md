---
name: "vanguard-databricks"
displayName: "Vanguard Frontier — Databricks (Azure)"
description: "Curated Azure Databricks agents for Unity Catalog governance and lakehouse engineering — static review only, no workspace or production mutations. Covers Unity Catalog securables and schema-scoped least-privilege grants, identity federation and account groups, account/workspace/metastore admin separation, run-production-as-service-principal, Entra ID managed identities, Access Connector + ADLS Gen2 external locations, cluster policies, and audit logs. Databricks and Azure surfaces are drift-prone; agents always verify against current Databricks and Microsoft Learn documentation before rendering findings."
keywords: ["databricks", "azure", "unity-catalog", "lakehouse", "least-privilege", "data-engineering", "static-review"]
author: "VincentChuWaiChow"
---
# Vanguard Frontier — Databricks (Azure)

Curated Azure Databricks agents for Unity Catalog governance and lakehouse engineering — static review only, no workspace or production mutations. Covers Unity Catalog securables and schema-scoped least-privilege grants, identity federation and account groups, account/workspace/metastore admin separation, run-production-as-service-principal, Entra ID managed identities, Access Connector + ADLS Gen2 external locations, cluster policies, and audit logs. Databricks and Azure surfaces are drift-prone; agents always verify against current Databricks and Microsoft Learn documentation before rendering findings.

## When to engage this Power

Activate when the task references Databricks (Azure) services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- *(no maestro for this provider; reference agents directly under `agents/databricks/`)*

Reference agents directly from agents/databricks/ without maestro-based routing.

## Live-guard agents (gate_mode only)

- `databricks-live-unity-catalog-grant-guard-at-azure-agent` — never auto-dispatched; gate_mode only

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Static review only — agents never request workspace tokens, service-principal secrets, storage keys, or customer data, and never mutate a Databricks workspace, Unity Catalog, or Azure resource.
- Enforce least privilege: schema-scoped grants (CREATE TABLE/VOLUME/FUNCTION at schema level), no broad ALL PRIVILEGES, assign access to account groups not individuals, separate account/workspace/metastore admin roles.
- Prefer Azure managed identities over service principals for storage access; production data is operated by service principals, not interactive users.
- Production grant/role/policy/cluster changes are live-guard gated — never auto-dispatched; require explicit approval, scope confirmation, and rollback plan.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/VincentChuWaiChow/vanguard-frontier-agentic) marketplace. For this provider, see `agents/databricks/` in that repository. All 3 agents in this provider ship a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add VincentChuWaiChow/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider databricks --repo .`
