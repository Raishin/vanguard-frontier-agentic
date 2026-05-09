# IAM Permissions — Huawei Live CCE Rollout Guard

## Minimum Read Roles (Discovery and Audit)

| Policy | Purpose |
|--------|---------|
| `CCE ReadOnlyAccess` | Read cluster, node pool, workload state; describe deployments and disruption budgets |

## Required Mutation Roles (Live Operations)

| Policy | Purpose | Scope Guidance |
|--------|---------|---------------|
| `CCE FullAccess` | Perform node pool upgrades, cluster version changes, and rollout mutations | Bind scoped to target cluster via enterprise project; never grant account-wide |

## Narrowing Guidance

- Bind `CCE FullAccess` to the IAM principal scoped to the **target enterprise project** containing the cluster, not account-wide.
- For read-only pre-flight audits, `CCE ReadOnlyAccess` is sufficient — do not use `CCE FullAccess` for reads.
- Use agency-based access (agency trust relationships) for cross-account CCE operations rather than direct user credentials.
- Prefer IAM user groups bound to the target enterprise project for cluster operators over direct per-user policies.

## Anti-Patterns — Never Grant

- `FullAccess` system policy at account level — grants complete control over all services in the account.
- `CCE FullAccess` without enterprise project scoping — blast radius covers every CCE cluster in the account.
- Storing kubeconfig credentials in plaintext — use temporary credentials via agency or IAM token exchange.
- `cluster-admin` ClusterRoleBinding inside the cluster without accompanying Huawei IAM — bypasses the CTS audit trail.

## Audit Trail

All CCE mutations (node pool upgrades, cluster version changes, addon updates) are logged in **Cloud Trace Service (CTS)**. Ensure CTS is enabled for the CCE service in the target region. Query: `CTS > Cloud Trace > CCE > node_pool_upgrade / cluster_upgrade`.
