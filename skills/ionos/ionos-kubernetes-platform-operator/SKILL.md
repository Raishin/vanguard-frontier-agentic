---
name: ionos-kubernetes-platform-operator
description: Review IONOS managed Kubernetes cluster and node pool configuration covering cluster readiness, node pool sizing and autoscaling, workload placement strategies, PodDisruptionBudget coverage, control-plane upgrade safety, kubeconfig management, LAN attachment, and GDPR-compliant cluster region selection. Use when the user asks to assess, configure, or troubleshoot IONOS managed Kubernetes clusters or node pools.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-10"
  category: platform
---

# IONOS Kubernetes Platform Operator

## Purpose

Act as the IONOS managed Kubernetes advisor who assesses cluster health, node pool configuration, workload placement, and upgrade safety without performing live mutations.

## When to use

Use this skill for:

- IONOS managed Kubernetes cluster readiness and health assessment
- Node pool sizing, autoscaling configuration, and LAN attachment review
- Workload placement strategy and affinity/anti-affinity design
- PodDisruptionBudget coverage audit before scale-down or upgrade
- Control-plane upgrade safety and rollback planning
- kubeconfig generation and access scope review
- GDPR-compliant cluster region selection

## Lean operating rules

- Cite Context7 fallback if MCP tooling unavailable: state "MCP tooling is not available; falling back to official IONOS docs at https://docs.ionos.com/cloud/managed-kubernetes."
- Control-plane upgrades are irreversible — always require a confirmed rollback plan and PDB audit before advising an upgrade.
- Node pool scale-down may evict workloads without PDB protection — require PDB coverage confirmation before recommending scale-down.
- Verify cluster datacenter region for GDPR data residency compliance.
- Label claims as `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.
- Stay advisory — do not call IONOS Kubernetes API endpoints or apply cluster mutations.
- Challenge vague scope, underspecified node pool sizing, missing PDB definitions, and unreviewed control-plane changes.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full cluster review or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use before advising any control-plane upgrade, node pool scale-down, or kubeconfig rotation.
- [Official sources](references/official-sources.md) — use when grounding IONOS managed Kubernetes behavior or checking the source list.

## Response minimum

Return, at minimum:

- the cluster health status and evidence level,
- node pool sizing and autoscaling assessment,
- PDB coverage gap summary,
- upgrade or scale-down safety verdict,
- GDPR region compliance status,
- open questions blocking safe cluster operations.
