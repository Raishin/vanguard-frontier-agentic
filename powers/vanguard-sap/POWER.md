---
name: "vanguard-sap"
displayName: "Vanguard Frontier — SAP"
description: "Curated SAP agents for S/4HANA, BTP, Integration Suite, ABAP Cloud, and transport management with clean-core review and guarded mutation gates. Routes via sap-maestro to specialist agents for landscape discovery, clean-core debt review, and guarded transport imports. Transport mutations require named approver, change ticket, target-system confirmation, and completed SoD check before dispatch."
keywords: ["sap", "s4hana", "btp", "abap", "clean-core", "transport-management", "live-guard"]
author: "Raishin"
---
# Vanguard Frontier — SAP

Curated SAP agents for S/4HANA, BTP, Integration Suite, ABAP Cloud, and transport management with clean-core review and guarded mutation gates. Routes via sap-maestro to specialist agents for landscape discovery, clean-core debt review, and guarded transport imports. Transport mutations require named approver, change ticket, target-system confirmation, and completed SoD check before dispatch.

## When to engage this Power

Activate when the task references SAP services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- **`sap-maestro-agent`** — classifies and routes the task to the right specialist

Use the maestro as the entry point: classify the task, then dispatch to one specialist or a parallel team of specialists. Never have the maestro itself execute a live mutation.

## Live-guard agents (gate_mode only)

- `sap-live-readonly-landscape-discovery-agent` — never auto-dispatched; gate_mode only

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Never auto-dispatch the transport-import operator agent — require explicit approval, change ticket, named approver, and completed SoD check before any tp or CTS import command.
- Clean-core debt review is static only — never connects to a live SAP system and never accepts embedded credentials or production system IDs.
- Landscape discovery agents use read-only roles only (BTP subaccount viewer, CF SpaceAuditor/OrgAuditor, ABAP display user) — refuse any request that maps to a create/update/delete/deploy/assign/rotate action.
- All SAP API surfaces and release contracts drift between versions; verify current SAP API Business Hub documentation before applying any remediation recommendation.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/Raishin/vanguard-frontier-agentic) marketplace. For this provider, see `agents/sap/` in that repository. All 19 agents in this provider ship a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add Raishin/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider sap --repo .`
