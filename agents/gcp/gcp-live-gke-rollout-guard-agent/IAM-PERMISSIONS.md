# IAM Permissions — GCP Live GKE Rollout Guard

## Minimum Read Roles (Discovery and Audit)

| Role | Purpose |
|------|---------|
| `roles/container.developer` | Read cluster, node pool, and workload state; describe deployments and PodDisruptionBudgets |
| `roles/container.viewer` | View cluster configuration, version channels, and node pool details |

## Required Mutation Roles (Live Operations)

| Role | Purpose | Scope Guidance |
|------|---------|---------------|
| `roles/container.clusterAdmin` | Perform node pool upgrades, cluster version changes, and rollout mutations | Bind at the specific project level, never at folder or org level |

## Narrowing Guidance

- Bind `roles/container.clusterAdmin` to the service account at the **project** level, not folder or organization.
- For read-only pre-flight audits, `roles/container.viewer` is sufficient — do not use `clusterAdmin` for reads.
- If using Workload Identity, ensure the Kubernetes service account is bound to a GCP service account with the minimum required role.
- Prefer using **Google Groups** or **Workload Identity** over direct per-user bindings for cluster operators.

## Anti-Patterns — Never Grant

- `roles/owner` — grants full resource control including billing and IAM; never appropriate for a GKE guard agent.
- `roles/editor` — overly broad; grants write access to nearly all GCP services.
- `roles/container.admin` at org or folder level — blast radius is every GKE cluster in the hierarchy.
- Direct `cluster-admin` ClusterRoleBinding inside the cluster without accompanying GCP IAM — bypasses the GCP audit trail.
- Service account keys stored in plaintext — use Workload Identity or impersonation instead.

## Audit Trail

All GKE mutations (node pool upgrades, cluster version changes) are logged in **Cloud Audit Logs** under `container.googleapis.com`. Ensure `DATA_READ` and `DATA_WRITE` audit log types are enabled for the container service in the target project.
