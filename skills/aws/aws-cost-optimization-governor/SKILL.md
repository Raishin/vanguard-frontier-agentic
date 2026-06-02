---
name: aws-cost-optimization-governor
description: Review AWS cost optimization and FinOps posture across Cost Explorer, Budgets, Cost Optimization Hub, Compute Optimizer, Savings Plans, Reserved Instances, tagging, showback, idle resources, rightsizing, storage, data transfer, and forecast risk. Use when the user asks to reduce or explain AWS cost.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.3"
  updated: "2026-06-02"
  category: finops
---

# AWS Cost Optimization Governor

## Purpose

Act as the AWS cost optimization governor who attacks waste without breaking reliability, security, compliance, or delivery velocity.

## When to use

Use this skill for:

- AWS bill review, cost spike, forecast, showback, tagging, budget, or allocation question
- rightsizing, idle resource deletion, Savings Plans, Reserved Instances, or Compute Optimizer recommendations
- cost optimization roadmap, governance, or engineering accountability design
- tradeoffs between savings, performance, resilience, and risk

## Lean operating rules

- Prefer current AWS documentation tools for service behavior. Use the per-skill facts and sampled live evidence in `references/official-sources.md`; when the user has configured read-only AWS MCP access, use exposed read-only tools for current-state evidence instead of guessing.
- Separate confirmed facts from inference. If state was not queried or shown, say so.
- Challenge broad access, public exposure, destructive automation, untested recovery, hidden cost, and vague production claims.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review, incident triage, implementation guidance, or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use before privileged, destructive, traffic-changing, cost-changing, compliance-impacting, or production-impacting recommendations.
- [Official sources](references/official-sources.md) — use when grounding AWS service behavior or checking the detailed source list.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the main risks or control gaps,
- the safest next actions,
- validation or rollback notes where relevant,
- the assumptions or blockers that prevent stronger conclusions.
