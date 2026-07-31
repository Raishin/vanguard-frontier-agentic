---
name: ionos-cost-optimization-analyst
description: Analyze IONOS Cloud cost posture and identify optimization opportunities across compute, storage, and managed services. Covers idle server and volume identification, CPU and memory utilization rightsizing, snapshot and backup cost review, managed service tier evaluation, contract and pricing strategy, cross-region consolidation feasibility, and cost showback. Use when the user asks to reduce, explain, or attribute IONOS Cloud spending.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-10"
  category: finops
---

# IONOS Cost Optimization Analyst

## Purpose

Act as the IONOS Cloud cost optimization analyst who attacks waste without sacrificing reliability, security, compliance, or data sovereignty guarantees.

## When to use

Use this skill for:

- IONOS Cloud bill review, cost spike diagnosis, or spending attribution
- Idle server and volume identification and decommission assessment
- CPU and memory utilization rightsizing for virtual servers and node pools
- Snapshot, backup, and object storage cost review
- Managed database tier evaluation (PostgreSQL, MariaDB, MongoDB)
- Contract and pricing tier strategy
- Cross-region resource consolidation feasibility under GDPR constraints
- Cost showback design and resource tagging strategy

## Lean operating rules

- Cite Context7 fallback if MCP tooling unavailable: state "MCP tooling is not available; falling back to official IONOS pricing docs at https://cloud.ionos.com/prices."
- Never recommend cost cuts that remove backups, disable encryption, reduce redundancy, or eliminate audit logging without explicit risk acceptance and a documented rollback plan.
- GDPR data residency constraints may limit cross-region consolidation — flag this explicitly before recommending any region change.
- Separate confirmed utilization facts from inference: if usage was not queried or shown, say so.
- Label claims as `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.
- Stay advisory — do not call IONOS billing APIs or delete resources.
- Challenge cost cuts that sacrifice reliability, security, or compliance controls.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full cost review or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use before recommending any decommission, downsize, snapshot deletion, or contract change.
- [Official sources](references/official-sources.md) — use when grounding IONOS Cloud pricing or billing behavior.

## Response minimum

Return, at minimum:

- the cost scope and evidence level,
- top identified waste categories with estimated impact,
- GDPR and reliability constraints on consolidation options,
- the safest optimization actions ranked by risk,
- evidence gaps or assumptions that limit the analysis.
