---
name: gcp-live-gke-rollout-guard
description: Gate GKE deployment mutations, node pool upgrades, and cluster control-plane version changes against rollback posture and PDB audit before any production change. Prevents irreversible node pool upgrades from proceeding without PodDisruptionBudget verification, surge settings review, and explicit operator approval.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: delivery
---

# GCP Live GKE Rollout Guard

## Purpose

Act as the guarded live GCP operator for gcp-live-gke-rollout-guard work. Gate GKE deployment mutations, node pool upgrades, and cluster control-plane version changes. Insist on PDB audit and rollback posture evidence before execution, and treat any ambiguous approval or target as a stop condition.

## When to Use

Use this skill when:

- A GKE node pool upgrade is requested (Kubernetes minor or patch version bump)
- A cluster control-plane version change is planned
- A Deployment or DaemonSet rollout is being executed against a production workload
- Surge upgrade settings or max-unavailable values need to be changed
- An operator needs to audit PodDisruptionBudgets before a disruptive node pool operation
- An emergency rollback of a broken rollout is required

## When NOT to Use

Do not use this skill when:

- The target is a non-production cluster with no PDB requirements and no live traffic
- The task is creating a brand-new cluster (no existing workloads at risk)
- The task is purely read-only cluster inspection with no mutation intent
- The task involves Cloud Run, App Engine, or other non-GKE compute

## Pre-Flight Checklist

Before executing any GKE mutation, verify all of the following:

1. **Cluster identity confirmed** — run `gcloud container clusters describe <CLUSTER> --region <REGION> --project <PROJECT>` and confirm the cluster name, version, and region match the intended target.
2. **Active principal confirmed** — run `gcloud auth list` and `gcloud config get-value account` to confirm the active identity has the required role.
3. **Current node pool version and target version captured** — document both before proceeding; confirm the target version is available in the release channel.
4. **PDB audit complete** — run `kubectl get pdb --all-namespaces` and confirm no PDB has `DISRUPTIONS ALLOWED: 0` for workloads running on the affected node pool.
5. **Surge upgrade settings reviewed** — confirm `maxSurge` and `maxUnavailable` settings on the node pool are appropriate for the workload disruption tolerance.
6. **Rollback posture acknowledged** — node pool upgrades cannot be downgraded; operator must explicitly acknowledge this is one-way.
7. **Maintenance window and change window confirmed** — confirm the upgrade is within the approved maintenance window and any required change tickets are approved.
8. **Rollout history captured** — run `kubectl rollout history deployment/<NAME> -n <NAMESPACE>` to document the pre-change state for Deployment rollouts.

## Required Confirmation

The operator must explicitly state all of the following before any mutation is executed:

- "I confirm the cluster is `<CLUSTER_NAME>` in project `<PROJECT_ID>`, region `<REGION>`."
- "I confirm the target version is `<TARGET_VERSION>` and I understand node pool upgrades cannot be downgraded."
- "I have reviewed PDB status for all workloads on this node pool and no disruption-blocking PDB is present."
- "I approve this rollout action."

## Execution Steps

1. Capture pre-change state: cluster version, node pool version, all PDB states, Deployment rollout history.
2. Confirm active principal and IAM role (`roles/container.clusterAdmin` for mutation).
3. Present the planned change and its blast radius to the operator for explicit approval.
4. Execute the mutation:
   - Node pool upgrade: `gcloud container node-pools upgrade <POOL> --cluster <CLUSTER> --region <REGION> --project <PROJECT>`
   - Cluster control-plane upgrade: `gcloud container clusters upgrade <CLUSTER> --master --cluster-version <VERSION> --region <REGION> --project <PROJECT>`
   - Deployment rollout: `kubectl set image deployment/<NAME> <CONTAINER>=<IMAGE> -n <NAMESPACE>`
5. Monitor rollout progress: `kubectl rollout status deployment/<NAME> -n <NAMESPACE>` or `gcloud container operations describe <OPERATION_ID>`.
6. Verify all nodes reach `Ready` status and all workloads are running post-upgrade.

## Rollback Procedure

- **Deployment rollback** (reversible): `kubectl rollout undo deployment/<NAME> -n <NAMESPACE>`
- **Node pool upgrade** (NOT reversible): A completed node pool upgrade cannot be downgraded. If the upgrade fails mid-way, manual node recreation or a new node pool at the previous version is required.
- **Control-plane upgrade** (NOT reversible): Control-plane version cannot be rolled back. If the upgrade causes issues, you must address them on the upgraded version or redeploy the cluster.
- Document the incident and open a GCP Support case if node pool corruption is suspected.

## Post-Change Verification

1. Run `gcloud container clusters describe <CLUSTER>` — confirm `currentMasterVersion` matches target.
2. Run `gcloud container node-pools describe <POOL> --cluster <CLUSTER>` — confirm `version` matches target.
3. Run `kubectl get nodes` — confirm all nodes show `Ready` with the new version.
4. Run `kubectl get pods --all-namespaces` — confirm no pods in `CrashLoopBackOff` or `Pending` state.
5. Run `kubectl get pdb --all-namespaces` — confirm all PDBs still show healthy disruption budgets.
6. Verify application health via service-level health checks, error rate, and latency metrics in Cloud Monitoring.

## Response Shape

1. Cluster and node pool identity confirmation
2. Current cluster/node pool version vs. target
3. PDB audit for affected workloads
4. Rollout strategy and surge settings
5. Approval status
6. Proposed or executed rollout action
7. Post-rollout verification steps
