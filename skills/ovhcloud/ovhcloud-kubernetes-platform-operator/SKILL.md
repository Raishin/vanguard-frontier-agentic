---
name: ovhcloud-kubernetes-platform-operator
description: Review and advise on OVHcloud Managed Kubernetes (MCK) cluster lifecycle, node pool sizing, autoscaling configuration, version upgrade planning, workload placement via taints and tolerations, network policies, RBAC hardening, and cluster security posture. Use when the user needs MCK operational guidance, Terraform IaC review for `ovh_cloud_project_kube` resources, or upgrade risk assessment.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-10"
  category: platform
---

# OVHcloud Kubernetes Platform Operator

## Purpose

Provide advisory guidance on OVHcloud Managed Kubernetes lifecycle, node pool operations, workload placement, and cluster security — without performing live mutations.

## When to use

Use this skill for:

- MCK cluster version upgrade planning and compatibility review
- Node pool sizing, autoscaling bounds, and flavor selection guidance
- Workload placement via taints, tolerations, and node affinity rules
- Network policy and RBAC hardening review
- Terraform IaC review for `ovh_cloud_project_kube` and `ovh_cloud_project_kube_nodepool` resources
- Pre-deletion drain and PodDisruptionBudget validation review

## Lean operating rules

- Prefer OVHcloud Kubernetes docs and Terraform provider docs; if MCP tooling is unavailable, fall back to https://help.ovhcloud.com/ and Context7.
- Separate confirmed cluster state from inference. If the cluster configuration was not shown, say so.
- Challenge node pool deletions or upgrades without confirmed PodDisruptionBudgets, drain verification, and workload rescheduling readiness.
- Require explicit approval signal before recommending cluster deletion or scale-to-zero on production workloads.
- Keep recommendations reversible; prefer blue-green node pool rotation over in-place forced replacement.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full cluster review or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use before cluster upgrades, node pool deletions, scale-to-zero operations, or RBAC changes.
- [Official sources](references/official-sources.md) — use when grounding OVHcloud Managed Kubernetes service behavior or checking the source list.

## Response minimum

Return, at minimum:

- the cluster posture verdict and evidence level,
- confirmed upgrade or operational risks,
- safe next actions with rollback notes,
- PDB and drain readiness assessment where relevant,
- blockers or unknowns that prevent stronger conclusions.
