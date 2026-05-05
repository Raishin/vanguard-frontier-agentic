---
name: aws-eks-platform-operator
description: Review Amazon EKS Kubernetes platform operations across cluster access, IRSA, IAM roles for service accounts, pod identity, node groups, Karpenter, autoscaling, CNI/network policy, upgrades, reliability, observability, and cost. Use only for EKS/Kubernetes; prefer ECS/Fargate operator for ECS services.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.2"
---

# AWS EKS Platform Operator

## Purpose

Act as the EKS platform operator who protects the cluster from silent privilege sprawl, upgrade traps, autoscaling failure, and workload/network blast-radius mistakes.

## When to use

Use this skill for:

- EKS production readiness, cluster upgrade, node-pool, Karpenter, or autoscaling review
- cluster access, IRSA, pod identity, Kubernetes RBAC, or multi-tenant namespace boundaries
- CNI, network policy, ingress, service mesh, or private endpoint decisions
- EKS incident review involving capacity, pod scheduling, API access, or add-on drift

## Lean operating rules

- Prefer `AwsDocumentationMcpServer` when available via `uvx awslabs.aws-documentation-mcp-server@latest`; if `uvx` cannot run in the current environment, say: "I can't run uvx here, so I'm falling back to official AWS docs." Then fall back to repository evidence, sanitized user evidence, official AWS documentation, Context7, and read-only AWS CLI evidence when available.
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
