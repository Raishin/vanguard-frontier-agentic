---
name: "vanguard-terraform"
displayName: "Vanguard Frontier — Terraform"
description: "Curated Terraform agents for plan/apply review, state safety, deletion protection, and blast-radius assessment of IaC changes. Routes via terraform-maestro to the terraform-reviewer for plan analysis. Plan review is required before any apply targeting real infrastructure; state-modifying commands require explicit approval."
keywords: ["terraform", "iac", "plan-review", "state-safety", "deletion-protection", "blast-radius"]
author: "VincentChuWaiChow"
---
# Vanguard Frontier — Terraform

Curated Terraform agents for plan/apply review, state safety, deletion protection, and blast-radius assessment of IaC changes. Routes via terraform-maestro to the terraform-reviewer for plan analysis. Plan review is required before any apply targeting real infrastructure; state-modifying commands require explicit approval.

## When to engage this Power

Activate when the task references Terraform, IaC, plan, apply, or state. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).

## Routing pattern

- **`terraform-maestro-agent`** — classifies and routes the task to the right specialist

Use the maestro as the entry point: classify the task, then dispatch to one specialist or a parallel team of specialists. Never have the maestro itself execute a live mutation.

## Live-guard agents (gate_mode only)

- *(none — this provider has no live-mutation guards in the catalog)*

Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.

## Invariants

- Plan review (terraform plan output) must precede any apply on real infrastructure.
- Resource destruction and replacement (terraform plan: '-/+') require explicit confirmation with backup verification.
- State-modifying commands (terraform state rm, mv, push) require explicit approval — they bypass plan review.
- Workspace context (workspace, var-file, backend) must be confirmed before running plan or apply.

## Where the agents live

Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/VincentChuWaiChow/vanguard-frontier-agentic) marketplace. For this provider, see `agents/terraform/` in that repository. All 9 agents in this provider ship a Kiro adapter (`harnesses/kiro-ide.agent.md`, `kiro-cli.agent.json`).

## Companion install paths

- **Claude Code:** `/plugin marketplace add VincentChuWaiChow/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`
- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** `npx vfa-export-agents --platform <harness> --provider terraform --repo .`
