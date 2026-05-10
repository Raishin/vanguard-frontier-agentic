---
name: scaleway-live-kapsule-rollout-guard
description: Gate and execute Scaleway Kapsule live mutations — Kubernetes version upgrades, node pool creation/deletion/scaling, and cluster configuration changes — with mandatory PDB audit, cluster health verification, explicit approval, and a documented rollback plan. Use when a live Kapsule cluster or node pool mutation is requested. Hard-stops when target cluster ID, region/zone, approval, or rollback plan is absent or ambiguous.
allowed-tools: Read Grep Glob Bash
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-10"
  category: platform
---

# Scaleway Live Kapsule Rollout Guard

## Purpose

Act as the approval-gated gate keeper for Scaleway Kapsule live mutations. Enforce the pre-flight checklist before any cluster or node pool change proceeds.

## When to use

Use this skill when:

- A live Kapsule cluster Kubernetes version upgrade is requested
- A node pool creation, deletion, or scaling operation is requested
- A Kapsule cluster configuration change is requested (e.g., admission plugins, auto-upgrade settings)
- The user wants to verify cluster health before a rollout

**Do NOT use for advisory planning** — use `scaleway-kapsule-platform-operator` for read-only assessment.

## Hard-stop checklist (ALL required before proceeding)

Stop and refuse if any item is unresolved:

1. **Target confirmed**: Cluster ID and region/zone explicitly stated and confirmed by the user
2. **Cluster healthy**: API server reachable, all nodes Ready, no unhealthy node pools
3. **PDB audit complete**: All workload namespaces checked for PodDisruptionBudget coverage; unprotected workloads documented and accepted
4. **Approval received**: Explicit human sign-off token, ticket reference, or written approval provided
5. **Rollback plan documented**:
   - Version upgrade: prior version noted; node pool replacement path identified if rollback requires pool recreation
   - Node pool deletion: workload migration plan confirmed; no unschedulable pods remaining
   - Scaling: resource headroom verified for current workloads at reduced pool size

## Pre-flight commands (reference only — do not execute without approval)

```bash
# Check cluster status
scw k8s cluster get <cluster-id> region=<region>

# List node pools
scw k8s pool list cluster-id=<cluster-id> region=<region>

# Check node readiness (kubectl)
kubectl get nodes -o wide

# Audit PodDisruptionBudgets
kubectl get pdb --all-namespaces

# Check workload disruption tolerance
kubectl get deployment,statefulset --all-namespaces -o wide
```

## Irreversibility warnings

- **Control-plane version upgrade is a one-way door**: Kapsule cannot downgrade a cluster to a previous Kubernetes minor version after upgrade completes.
- **CNI type is immutable**: CNI selection at cluster creation cannot be changed without cluster recreation. Refuse CNI change requests that require cluster recreation without full blast-radius assessment.
- **Node pool deletion is immediate**: All pods on deleted pool nodes are evicted without grace period once the pool deletion is confirmed. Cordon-drain must complete first.

## Lean operating rules

- Prefer Scaleway Kubernetes API for live cluster health evidence; if MCP tooling is unavailable, say: "I can't access live Scaleway MCP here, so I'm falling back to official docs." Then use https://www.scaleway.com/en/docs/kubernetes/ and Context7 as fallback.
- Separate live evidence from inference. Treat `documentation-based` or `inference` claims as insufficient for destructive operations.
- Never request `SCW_ACCESS_KEY` or `SCW_SECRET_KEY` directly. Credentials must be pre-configured in the environment.
- Label all cluster state as `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.
- Load references only when needed; do not pull all guidance into short answers.

## References

Load these only when needed:

- Scaleway Kubernetes docs: https://www.scaleway.com/en/docs/kubernetes/
- Kapsule API reference: https://www.scaleway.com/en/developers/api/kubernetes/
- Terraform k8s_cluster: https://registry.terraform.io/providers/scaleway/scaleway/latest/docs/resources/k8s_cluster
- Terraform k8s_pool: https://registry.terraform.io/providers/scaleway/scaleway/latest/docs/resources/k8s_pool
- Kubernetes PDB: https://kubernetes.io/docs/concepts/workloads/pods/disruptions/

## Response minimum

Return, at minimum:

- go/no-go verdict with all checklist item statuses,
- evidence level for each checklist item,
- specific blockers preventing proceed,
- rollback plan status,
- next safe action (or explicit STOP with reason).
