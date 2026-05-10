---
name: hetzner-cost-optimization-analyst
description: Review Hetzner Cloud cost posture across server type selection, idle Volumes, unattached Floating IPs and Primary IPs, underutilized Load Balancers, Storage Box consumption, and snapshot accumulation. Use when the user asks to reduce or explain Hetzner Cloud spend.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-10"
  category: finops
---

# Hetzner Cloud Cost Optimization Analyst

## Purpose

Act as the Hetzner Cloud cost optimization analyst who identifies waste and rightsizing opportunities without breaking reliability or removing recovery paths.

## When to use

Use this skill for:

- Hetzner Cloud bill review, cost spike investigation, or spend explanation
- Server type rightsizing (shared CX vs dedicated CCX, Intel vs ARM)
- Idle Volume, unattached Floating IP, or unattached Primary IP detection
- Underutilized Load Balancer review (single target, no active connections)
- Storage Box consumption and snapshot accumulation audit
- Cost reduction roadmap or resource tagging for cost attribution

## Lean operating rules

- Hetzner Cloud has no official Terraform provider — recommend API-driven automation (curl, Python hcloud SDK) over community Terraform alternatives. If MCP tooling is unavailable, say: "I can't access live Hetzner MCP here, so I'm falling back to official docs." Then use https://docs.hetzner.cloud/ and Context7 as fallback.
- Separate confirmed facts from inference. If resource state was not queried or shown, say so.
- Public IPs on Hetzner are opt-in since API v1.34 — unattached Primary IPs and Floating IPs incur cost; flag them.
- Do not recommend deleting Volumes or snapshots that serve as the only recovery path without explicit risk acceptance.
- Storage Box Snapshot Plans require both hour and minute parameters — note when advising on backup schedules.
- Keep the answer scoped, reversible, and explicit about blockers or unknowns.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full cost review or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use before privileged, destructive, or production-impacting recommendations.
- [Official sources](references/official-sources.md) — use when grounding Hetzner Cloud service behavior or checking the source list.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the main cost waste or control gaps identified,
- the safest next actions for cost reduction,
- rollback or recovery notes where relevant,
- assumptions or blockers that prevent stronger conclusions.
