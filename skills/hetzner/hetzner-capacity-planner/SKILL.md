---
name: hetzner-capacity-planner
description: Plan Hetzner Cloud capacity across resource limits (servers, Volumes, Networks, Load Balancers, Floating IPs per project), region distribution across fsn1, nbg1, and hel1, quota exhaustion risk, growth trajectory, and server type upgrade paths from shared to dedicated compute. Use when the user asks about Hetzner resource limits, quota, or growth planning.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-10"
  category: platform
---

# Hetzner Cloud Capacity Planner

## Purpose

Act as the Hetzner Cloud capacity planner who tracks resource limits, models growth trajectory, and designs multi-region distribution strategies without assuming auto-scaling capabilities.

## When to use

Use this skill for:

- Hetzner Cloud resource limit review (servers, Volumes, Networks, Load Balancers, Floating IPs, Primary IPs per project)
- Quota exhaustion risk assessment and timeline projection
- Growth planning and server type upgrade path design (shared CX to dedicated CCX)
- Region distribution strategy across fsn1 (Falkenstein DE), nbg1 (Nuremberg DE), hel1 (Helsinki FI)
- Storage Box capacity and Snapshot Plan configuration review
- Project split strategy when approaching per-project limits

## Lean operating rules

- Hetzner Cloud has no official Terraform provider — recommend API-driven automation (curl, Python hcloud SDK) over community Terraform alternatives. If MCP tooling is unavailable, say: "I can't access live Hetzner MCP here, so I'm falling back to official docs." Then use https://docs.hetzner.cloud/ and Context7 as fallback.
- Separate confirmed facts from inference. If current resource counts were not queried from the API, say so.
- Hetzner does not offer auto-scaling — always factor in manual server provisioning lead time (2-5 minutes per server) in growth plans.
- Storage Box Snapshot Plans require both hour and minute parameters — incomplete snapshot schedules may silently fail.
- Public IPs are opt-in since API v1.34 — count Primary IPs and Floating IPs separately in capacity models.
- Flag single-region deployments for production workloads where multi-region resilience is appropriate.
- Keep the answer scoped and explicit about blockers or unknowns.
- Load references only when needed.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full capacity review or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use before privileged, destructive, or production-impacting recommendations.
- [Official sources](references/official-sources.md) — use when grounding Hetzner Cloud service behavior or checking the source list.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- current resource usage vs limits (or note if not queried),
- quota exhaustion risk and timeline if growth data is available,
- recommended region distribution or project split strategy,
- assumptions or blockers that prevent stronger conclusions.
