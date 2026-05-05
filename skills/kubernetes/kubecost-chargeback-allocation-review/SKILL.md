---
name: kubecost-chargeback-allocation-review
description: Use this skill when reviewing a Kubecost or OpenCost installation for enterprise chargeback readiness. Trigger when the user asks whether cost allocation is accurate, whether label taxonomy is complete enough for chargeback, whether idle cost is properly attributed, whether the cost API is secured, or whether savings recommendations are being actioned.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
---

# Kubecost Chargeback and Allocation Review

## Purpose

Review a Kubecost (or OpenCost) deployment for cost allocation accuracy, label taxonomy completeness, shared cost model selection, idle cost attribution policy, budget alert coverage, cost API authentication posture, and savings recommendation hygiene. Enterprise chargeback requires that every dollar spent can be attributed to a team, cost center, or product — gaps in label coverage, authentication, or idle allocation produce inaccurate charge-backs and hide engineering waste.

## Lean operating rules

- Prefer user-provided Kubecost allocation API output, Helm values, and `kubectl` label query results as primary evidence; official Kubecost and OpenCost docs are the authoritative fallback.
- Treat the Kubecost cost allocation API or frontend exposed without SSO/ingress authentication as a HIGH finding — any pod in the cluster can enumerate other teams' spend.
- Treat more than 20% of pod costs appearing in the "uncategorized" or "__unallocated__" bucket as a HIGH finding — chargeback to business units is impossible for that spend.
- Treat idle cost absorbed centrally (not attributed to namespace owners) as a MEDIUM finding — it hides waste from the engineering teams responsible for right-sizing.
- Treat PV (persistent volume) costs excluded from allocation as a MEDIUM finding — stateful teams face an invisible blind spot in their bill.
- Treat no budget alerts configured for any namespace or team as a MEDIUM finding — teams have no cost signal until the end-of-month invoice.
- Treat HIGH-priority savings recommendations unactioned for more than 30 days as a HIGH finding — direct, measurable cash waste with a documented fix path.
- Distinguish OpenCost (no multi-cluster single-pane, no team RBAC) from Kubecost Enterprise (multi-cluster, RBAC, advanced savings) when scope matters for the use case.

## References

Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md)

## Response minimum

- Scoped target (cluster name, Kubecost version, OpenCost vs Kubecost) and evidence level
- Cost allocation accuracy verdict (all cost components enabled or missing)
- Label taxonomy completeness (% uncategorized, missing labels)
- Shared cost model and idle cost attribution policy
- Budget alert coverage (configured / absent / threshold)
- Cost API authentication posture
- Top savings recommendations status
- Safe next actions and open questions
