---
name: "vanguard-databricks"
displayName: "Vanguard Frontier — Databricks"
description: "Curated Databricks agents spanning a cloud-neutral lakehouse and AI board plus Azure-specific assets — static review only, no workspace or production mutations. Covers account and workspace topology, Unity Catalog governance (three-level namespace, GRANT model, workspace-catalog binding, governed tags), identity/network security (SCIM, service principals, OAuth vs personal access tokens, IP access lists, serverless egress, secret scopes), data protection and privacy (row filters, column masks, ABAC, classification, erasure via REORG/VACUUM, Delta Sharing egress, residency), Lakeflow pipelines and Delta table layout, Structured Streaming recovery, data quality and Lakehouse Monitoring, SQL warehouse performance, AI/BI Genie and metric views, MLflow and Model Serving, GenAI agent engineering and evaluation, Declarative Automation Bundles and CI/CD, operational evidence from system tables, FinOps cost attribution, and value realization. Databricks surfaces are drift-prone and differ by cloud, tier, and compute type; agents verify against current Databricks documentation, and pin version-sensitive client APIs against library documentation, before rendering findings."
keywords: ["databricks", "unity-catalog", "lakehouse", "lakeflow", "mlflow", "genai", "finops", "least-privilege", "data-engineering", "static-review"]
author: "VincentChuWaiChow"
---
# Vanguard Frontier — Databricks

Curated Databricks agents spanning a cloud-neutral lakehouse and AI board plus Azure-specific assets — static review only, no workspace or production mutations. Covers account and workspace topology, Unity Catalog governance (three-level namespace, GRANT model, workspace-catalog binding, governed tags), identity/network security (SCIM, service principals, OAuth vs personal access tokens, IP access lists, serverless egress, secret scopes), data protection and privacy (row filters, column masks, ABAC, classification, erasure via REORG/VACUUM, Delta Sharing egress, residency), Lakeflow pipelines and Delta table layout, Structured Streaming recovery, data quality and Lakehouse Monitoring, SQL warehouse performance, AI/BI Genie and metric views, MLflow and Model Serving, GenAI agent engineering and evaluation, Declarative Automation Bundles and CI/CD, operational evidence from system tables, FinOps cost attribution, and value realization. Databricks surfaces are drift-prone and differ by cloud, tier, and compute type; agents verify against current Databricks documentation, and pin version-sensitive client APIs against library documentation, before rendering findings.

## When to engage this Power

Activate when the task references Databricks services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- **`databricks-maestro-agent`** — classifies and routes the task to the right specialist

Use the maestro as the entry point: classify the task, then dispatch to one specialist or a parallel team of specialists. Never have the maestro itself execute a live mutation.

## Live-guard agents (gate_mode only)

- `databricks-live-unity-catalog-grant-guard-at-azure-agent` — never auto-dispatched; gate_mode only

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Static review only — agents never request workspace tokens, service-principal secrets, storage keys, or customer data, and never mutate a Databricks workspace, Unity Catalog, or Azure resource.
- Enforce least privilege: schema-scoped grants (CREATE TABLE/VOLUME/FUNCTION at schema level), no broad ALL PRIVILEGES, assign access to account groups not individuals, separate account/workspace/metastore admin roles.
- Prefer Azure managed identities over service principals for storage access; production data is operated by service principals, not interactive users.
- Production grant/role/policy/cluster changes are live-guard gated — never auto-dispatched; require explicit approval, scope confirmation, and rollback plan.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/VincentChuWaiChow/vanguard-frontier-agentic) marketplace. For this provider, see `agents/databricks/` in that repository. All 20 agents in this provider ship a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add VincentChuWaiChow/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider databricks --repo .`
