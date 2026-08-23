---
name: "vanguard-snowflake"
displayName: "Vanguard Frontier — Snowflake"
description: "A cloud-neutral Snowflake board routed by snowflake-maestro: architecture and account topology, platform administration, identity and effective access, network and private connectivity, governance and privacy controls, independent compliance evidence, FinOps unit economics, query performance, batch pipelines, streaming ingestion reliability, analytical semantics, ML lifecycle, Cortex AI agent security, Native Apps and Marketplace, BCDR, DevOps/IaC, migration, and business value. Review agents are read-only; six approval-gated live guards each execute exactly one mutation behind an explicit written human gate. Snowflake surfaces are drift-prone — agents re-verify GA/preview status, deprecations, edition/cloud/region availability, and provider resource stability against current Snowflake documentation before rendering findings."
keywords: ["snowflake", "rbac", "least-privilege", "data-governance", "finops", "cortex-ai", "bcdr", "live-guard"]
author: "VincentChuWaiChow"
---
# Vanguard Frontier — Snowflake

A cloud-neutral Snowflake board routed by snowflake-maestro: architecture and account topology, platform administration, identity and effective access, network and private connectivity, governance and privacy controls, independent compliance evidence, FinOps unit economics, query performance, batch pipelines, streaming ingestion reliability, analytical semantics, ML lifecycle, Cortex AI agent security, Native Apps and Marketplace, BCDR, DevOps/IaC, migration, and business value. Review agents are read-only; six approval-gated live guards each execute exactly one mutation behind an explicit written human gate. Snowflake surfaces are drift-prone — agents re-verify GA/preview status, deprecations, edition/cloud/region availability, and provider resource stability against current Snowflake documentation before rendering findings.

## When to engage this Power

Activate when the task references Snowflake services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- **`snowflake-maestro-agent`** — classifies and routes the task to the right specialist

Use the maestro as the entry point: classify the task, then dispatch to one specialist or a parallel team of specialists. Never have the maestro itself execute a live mutation.

## Live-guard agents (gate_mode only)

- `snowflake-live-auth-network-policy-guard-agent` — never auto-dispatched; gate_mode only
- `snowflake-live-data-protection-policy-guard-agent` — never auto-dispatched; gate_mode only
- `snowflake-live-failover-promotion-guard-agent` — never auto-dispatched; gate_mode only
- `snowflake-live-pipeline-streaming-change-guard-agent` — never auto-dispatched; gate_mode only
- `snowflake-live-rbac-grant-guard-agent` — never auto-dispatched; gate_mode only
- `snowflake-live-rbac-grant-guard-at-azure-agent` — never auto-dispatched; gate_mode only
- `snowflake-live-warehouse-cost-change-guard-agent` — never auto-dispatched; gate_mode only

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Review agents are static review only — they never request account credentials, private keys, OAuth or programmatic access tokens, account locators, or customer data, and never mutate a Snowflake account.
- Documentation proves supported platform behaviour, never configured account behaviour: edition, region, enabled bundles, authentication enforcement, and policy attachment are UNKNOWN until account evidence establishes them.
- Enforce least privilege: narrow custom roles, ACCOUNTADMIN forbidden for automation without exception, no GRANT ALL PRIVILEGES, no grants to PUBLIC, no unbounded future grants, and no password authentication for non-human identities.
- Live guards are never auto-dispatched: each executes exactly one mutation after explicit written human approval naming account, environment, target, mutation, and accepted blast radius, with prior-state capture, preflight, verification, and a rollback path.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/VincentChuWaiChow/vanguard-frontier-agentic) marketplace. For this provider, see `agents/snowflake/` in that repository. All 28 agents in this provider ship a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add VincentChuWaiChow/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider snowflake --repo .`
