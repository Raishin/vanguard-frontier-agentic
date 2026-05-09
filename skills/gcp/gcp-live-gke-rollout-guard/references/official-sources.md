# Official Sources — GCP Live GKE Rollout Guard

Authoritative GCP documentation for GKE rollout, node pool, and upgrade operations.

## Core References

- **Node Pools Overview** — https://cloud.google.com/kubernetes-engine/docs/concepts/node-pools
  Explains node pool architecture, managed upgrades, and surge upgrade settings.

- **Upgrading a Cluster** — https://cloud.google.com/kubernetes-engine/docs/how-to/upgrading-a-cluster
  Step-by-step guide for upgrading GKE control-plane and node pool versions, including surge upgrade configuration.

- **Pod Disruption Budgets** — https://cloud.google.com/kubernetes-engine/docs/how-to/pod-disruption-budgets
  How to configure and audit PDBs for workloads before disruptive node pool operations.

- **Rolling Updates for Deployments** — https://cloud.google.com/kubernetes-engine/docs/how-to/rolling-updates
  Configuring rolling update strategy, maxSurge, maxUnavailable, and rollback behavior for Deployments.

- **Release Channels** — https://cloud.google.com/kubernetes-engine/docs/concepts/release-channels
  Available versions per channel (Rapid, Regular, Stable) and how to select a target upgrade version.

- **Cluster Maintenance Windows** — https://cloud.google.com/kubernetes-engine/docs/how-to/maintenance-windows-and-exclusions
  Configuring and enforcing maintenance windows to control when auto-upgrades and manual upgrades can proceed.

- **GKE Audit Logging** — https://cloud.google.com/kubernetes-engine/docs/how-to/audit-logging
  Enabling and querying Cloud Audit Logs for GKE control-plane and data-plane operations.

- **Workload Availability During Upgrades** — https://cloud.google.com/kubernetes-engine/docs/how-to/maintaining-workload-availability
  Guidance on PodDisruptionBudgets, topology spread constraints, and anti-affinity rules to maintain availability during node pool upgrades.
