---
name: "vanguard-python"
displayName: "Vanguard Frontier — Python"
description: "Curated Python agents for python application security, python async concurrency reliability, python business critical automation governance, python container serverless runtime. Routes static review via python-maestro-agent and live control-plane work via python-live-governance-maestro-agent, which alone gates the live-guard operators. Live mutations require approval, target confirmation, evidence capture, and a rollback plan; static specialists never mutate."
keywords: ["python", "asyncio", "pyproject", "dependency-confusion", "static-review"]
author: "Raishin"
---
# Vanguard Frontier — Python

Curated Python agents for python application security, python async concurrency reliability, python business critical automation governance, python container serverless runtime. Routes static review via python-maestro-agent and live control-plane work via python-live-governance-maestro-agent, which alone gates the live-guard operators. Live mutations require approval, target confirmation, evidence capture, and a rollback plan; static specialists never mutate.

## When to engage this Power

Activate when the task references Python services, resources, or operations. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- **`python-maestro-agent`** — classifies and routes the task to the right specialist
- **`python-live-governance-maestro-agent`** — the live control-plane router; the only entry point that may place a live-guard operator in `live-guard-gate`

This board has two planes with two routers. Send static-review work to `python-maestro-agent` — its contract refuses live operations, so it must never be given a mutation task. Send live control-plane work to `python-live-governance-maestro-agent`, which is the only router that gates the live-guard operators below. Classify first, then dispatch to one specialist or a small parallel team; never have either maestro execute a mutation itself.

## Live-guard agents (gate_mode only)

- `python-live-code-remediation-agent` — never auto-dispatched; gate_mode only
- `python-live-data-change-control-agent` — never auto-dispatched; gate_mode only
- `python-live-job-control-agent` — never auto-dispatched; gate_mode only
- `python-live-model-promotion-control-agent` — never auto-dispatched; gate_mode only
- `python-live-release-control-agent` — never auto-dispatched; gate_mode only
- `python-live-rollback-and-recovery-agent` — never auto-dispatched; gate_mode only

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — `python-live-governance-maestro-agent` must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Live-guard agents (the mutating-runtime operators) must never be auto-dispatched; require explicit approval, evidence capture, and a rollback plan, and may only be gated by python-live-governance-maestro-agent. Read-only-runtime and static-review agents on this board are not guards.
- Route static-review tasks through python-maestro-agent and live control-plane tasks through python-live-governance-maestro-agent; never send a live-mutation task to python-maestro-agent, whose contract refuses live operations.
- Mixed-tier board: static specialists analyze configuration without mutating live systems; live-guard agents mutate only under approval, target confirmation, evidence capture, and a pre-approved rollback plan.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/Raishin/vanguard-frontier-agentic) marketplace. For this provider, see `agents/python/` in that repository. All 35 agents in this provider ship a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add Raishin/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider python --repo .`
