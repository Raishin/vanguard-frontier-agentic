---
name: alibaba-live-ack-rollout-guard
description: Gate ACK deployment mutations, node pool scaling, and cluster version upgrades against rollback posture and workload disruption budget. Prevents irreversible cluster version upgrades from proceeding without PodDisruptionBudget verification, node drain confirmation, and explicit operator approval.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: delivery
---

# Alibaba Cloud Live ACK Rollout Guard

## Purpose

Act as the guarded live Alibaba Cloud operator for alibaba-live-ack-rollout-guard work. Gate ACK deployment mutations, node pool scaling, and cluster version upgrades. Insist on PDB audit and rollback posture evidence before execution, and treat any ambiguous approval or target as a stop condition.

## When to Use

Use this skill when:

- An ACK cluster version upgrade is requested (Kubernetes minor or patch version bump)
- A node pool is being scaled up or down (especially when removing nodes)
- A Deployment or DaemonSet rollout is being executed against a production workload
- Node pool configuration changes are planned (instance type, image version, count)
- An operator needs to audit PodDisruptionBudgets before a disruptive node pool operation
- An emergency rollback of a broken rollout is required

## When NOT to Use

Do not use this skill when:

- The target is a non-production cluster with no PDB requirements and no live traffic
- The task is creating a brand-new cluster (no existing workloads at risk)
- The task is purely read-only cluster inspection with no mutation intent
- The task involves Function Compute, SAE, or other non-ACK compute

## Cluster Type Awareness

ACK supports three cluster types — mutation procedures differ per type:

- **Managed cluster**: Control plane managed by Alibaba Cloud. Node pool upgrades and scaling are the primary mutation surface.
- **Dedicated cluster**: Full control plane access. Both control plane and data plane versions must be managed.
- **Serverless cluster (ASK)**: No node pool concept. Pod-level scaling only — ECI instances are provisioned on demand. Version upgrades are less common but follow the same approval gate.

Always confirm cluster type before recommending a mutation path.

## Pre-Flight Checklist

Before executing any ACK mutation, verify all of the following:

1. **Cluster identity confirmed** — query the ACK API or Alibaba Cloud console to confirm the cluster ID, name, type, and region match the intended target.
2. **Active RAM principal confirmed** — confirm the active identity has the required RAM policy (`AliyunCSFullAccess` scoped to target cluster) for the operation.
3. **Current cluster version and node pool version captured** — document both before proceeding; confirm the target version is available for the cluster type.
4. **PDB audit complete** — run `kubectl get pdb --all-namespaces` and confirm no PDB has `DISRUPTIONS ALLOWED: 0` for workloads running on the affected node pool.
5. **Node drain posture verified** — for scale-in operations, confirm all nodes to be removed can be safely drained (no pods with no toleration for eviction, no local storage).
6. **Rollback posture acknowledged** — cluster version upgrades cannot be downgraded; operator must explicitly acknowledge this is one-way.
7. **Maintenance window confirmed** — confirm the upgrade is within the approved change window.
8. **Rollout history captured** — run `kubectl rollout history deployment/<NAME> -n <NAMESPACE>` to document the pre-change state for Deployment rollouts.

## Required Confirmation

The operator must explicitly state all of the following before any mutation is executed:

- "I confirm the cluster is `<CLUSTER_ID>` (`<CLUSTER_NAME>`) of type `<managed/dedicated/serverless>` in region `<REGION>`."
- "I confirm the target version is `<TARGET_VERSION>` and I understand cluster version upgrades cannot be downgraded."
- "I have reviewed PDB status for all workloads on this node pool and no disruption-blocking PDB is present."
- "I approve this rollout action."

## Execution Steps

1. Capture pre-change state: cluster version, node pool version, all PDB states, Deployment rollout history.
2. Confirm active RAM principal and policy scope.
3. Present the planned change and its blast radius to the operator for explicit approval.
4. Execute the mutation via the ACK console, Alibaba Cloud CLI (`aliyun cs`), or kubectl as appropriate.
5. Monitor rollout progress: `kubectl rollout status deployment/<NAME> -n <NAMESPACE>` or poll the ACK task status via API.
6. Verify all nodes reach `Ready` status and all workloads are running post-upgrade.

## Rollback Procedure

- **Deployment rollback** (reversible): `kubectl rollout undo deployment/<NAME> -n <NAMESPACE>`
- **Node pool scaling scale-in** (partially reversible): New nodes can be added back, but drained workloads need to be rescheduled.
- **Cluster version upgrade** (NOT reversible): A completed cluster version upgrade cannot be downgraded. If the upgrade causes issues, address them on the upgraded version or contact Alibaba Cloud support.
- Document the incident and open an Alibaba Cloud support ticket if cluster corruption is suspected.

## Post-Change Verification

1. Confirm cluster version matches target via ACK console or API.
2. Run `kubectl get nodes` — confirm all nodes show `Ready` with the new version.
3. Run `kubectl get pods --all-namespaces` — confirm no pods in `CrashLoopBackOff` or `Pending` state.
4. Run `kubectl get pdb --all-namespaces` — confirm all PDBs still show healthy disruption budgets.
5. Verify application health via CloudMonitor metrics: error rate and latency for affected workloads.

## Response Shape

1. Cluster type and version confirmed
2. Node pool inventory and version status
3. PDB audit for affected workloads
4. Rollout strategy
5. Approval status
6. Executed action
7. Post-rollout verification
