---
name: huawei-live-cce-rollout-guard
description: Gate CCE deployment mutations, node pool upgrades, and cluster version changes against rollback posture and workload disruption budget before any production change. Prevents irreversible CCE cluster upgrades from proceeding without workload PDB verification, addon compatibility check, node pool drain confirmation, and explicit operator approval.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-08"
  category: delivery
---

# Huawei Live CCE Rollout Guard

## Purpose

Act as the guarded live Huawei Cloud operator for huawei-live-cce-rollout-guard work. Gate CCE deployment mutations, node pool upgrades, and cluster version changes. Insist on workload PDB audit, node pool drain verification, addon compatibility check, and rollback posture evidence before execution. Treat any ambiguous approval or target as a stop condition.

## When to Use

Use this skill when:

- A CCE node pool upgrade is requested (Kubernetes minor or patch version bump)
- A CCE cluster version change is planned
- A Deployment or DaemonSet rollout is being executed against a production CCE workload
- A node pool is being scaled down and drain verification is required
- An addon upgrade (CoreDNS, NGINX Ingress, others) is requested
- An operator needs to audit workload disruption budgets before a disruptive node pool operation
- An emergency rollback of a broken CCE rollout is required

## When NOT to Use

Do not use this skill when:

- The target is a non-production CCE cluster with no PDB requirements and no live traffic
- The task is creating a brand-new CCE cluster (no existing workloads at risk)
- The task is purely read-only cluster inspection with no mutation intent
- The task involves FunctionGraph, ECS, or other non-CCE compute

## Pre-Flight Checklist

Before executing any CCE mutation, verify all of the following:

1. **Cluster identity confirmed** — describe the target cluster via CCE console or hcloud CLI and confirm the cluster name, version, and region match the intended target.
2. **Enterprise project confirmed** — confirm the cluster belongs to the correct enterprise project and the active principal has the required CCE FullAccess in that enterprise project.
3. **Active principal confirmed** — confirm the active Huawei Cloud IAM user or agency token has the required CCE FullAccess policy bound.
4. **Current node pool version and target version captured** — document both before proceeding; confirm the target version is supported by Huawei CCE.
5. **Workload PDB audit complete** — list all PodDisruptionBudgets across all namespaces and confirm no PDB has `DISRUPTIONS ALLOWED: 0` for workloads on the affected node pool.
6. **Node pool drain posture confirmed** — for scale-down operations, confirm all pods on the target nodes can be rescheduled and no pods have `nodeName` hard pinning.
7. **Addon compatibility verified** — if upgrading cluster version, check addon compatibility table for CoreDNS, NGINX Ingress, and any other installed addons against the target Kubernetes version.
8. **Rollback posture acknowledged** — CCE cluster version downgrades are not supported; operator must explicitly acknowledge this is one-way.
9. **Maintenance window confirmed** — confirm the upgrade is within the approved maintenance window and any required change tickets are approved.

## Required Confirmation

The operator must explicitly state all of the following before any mutation is executed:

- "I confirm the cluster is `<CLUSTER_NAME>` in enterprise project `<ENTERPRISE_PROJECT>`, account `<ACCOUNT_ID>`, region `<REGION>`."
- "I confirm the target version is `<TARGET_VERSION>` and I understand CCE cluster version downgrades are not supported."
- "I have reviewed PDB status for all workloads on this node pool and no disruption-blocking PDB is present."
- "I have verified addon compatibility with the target cluster version."
- "I approve this rollout action."

## Execution Steps

1. Capture pre-change state: cluster version, node pool version, all PDB states, addon versions.
2. Confirm active principal and IAM policy (CCE FullAccess scoped to target enterprise project for mutation).
3. Present the planned change and its blast radius to the operator for explicit approval.
4. Execute the mutation via CCE console or hcloud CLI:
   - Node pool upgrade: Update node pool version via CCE console > Cluster > Node Pools > Upgrade.
   - Cluster version upgrade: CCE console > Cluster > Upgrade or hcloud cce cluster-upgrade.
   - Deployment rollout: `kubectl set image deployment/<NAME> <CONTAINER>=<IMAGE> -n <NAMESPACE>`.
5. Monitor rollout progress: `kubectl rollout status deployment/<NAME> -n <NAMESPACE>`.
6. Verify all nodes reach `Ready` status and all workloads are running post-upgrade.

## Rollback Procedure

- **Deployment rollback** (reversible): `kubectl rollout undo deployment/<NAME> -n <NAMESPACE>`
- **Node pool upgrade** (NOT reversible): A completed node pool upgrade cannot be downgraded. If the upgrade fails mid-way, manual node recreation or a new node pool at the previous version is required.
- **Cluster version upgrade** (NOT reversible): Cluster version cannot be rolled back. Address issues on the upgraded version or contact Huawei Cloud support.
- Document the incident and open a Huawei Cloud Support ticket if node pool corruption is suspected.

## Post-Change Verification

1. Describe the cluster — confirm current version matches target.
2. List node pools — confirm version matches target.
3. Run `kubectl get nodes` — confirm all nodes show `Ready` with the new version.
4. Run `kubectl get pods --all-namespaces` — confirm no pods in `CrashLoopBackOff` or `Pending` state.
5. Run `kubectl get pdb --all-namespaces` — confirm all PDBs still show healthy disruption budgets.
6. Verify addon health: `kubectl get pods -n kube-system` — confirm CoreDNS, NGINX Ingress pods are running.
7. Verify application health via CES (Cloud Eye Service) alarms and LTS error rate metrics.

## Response Shape

1. Cluster version confirmed
2. Node pool inventory
3. Workload PDB audit
4. Addon version compatibility
5. Approval status
6. Executed action
7. Post-rollout verification
