# IAM Permissions — Alibaba Cloud Live ACK Rollout Guard

## Minimum Read Permissions (Discovery and Audit)

| Policy | Purpose |
|--------|---------|
| `AliyunCSReadOnlyAccess` | Read cluster state, node pool inventory, version status, and workload configuration |

## Required Mutation Permissions (Live Operations)

| Policy | Purpose | Scope Guidance |
|--------|---------|---------------|
| `AliyunCSFullAccess` | Perform node pool scaling, cluster version upgrades, and deployment rollout mutations | Scope narrowly to the target cluster ID; never grant account-wide without explicit justification |

## Narrowing Guidance

- Bind `AliyunCSFullAccess` to the RAM role scoped to the specific cluster resource ARN where possible.
- For pre-flight audits, `AliyunCSReadOnlyAccess` is sufficient — do not use `AliyunCSFullAccess` for read-only operations.
- Use RAM conditions to restrict mutations to the target region and cluster ID.
- Prefer RAM roles with STS token assumption over long-lived RAM user access keys for operational access.

## Anti-Patterns — Never Grant

- `AdministratorAccess` — account-wide full control; never appropriate for a cluster guard agent.
- `AliyunCSFullAccess` account-wide without cluster-level scoping — blast radius is every ACK cluster in the account.
- Long-lived RAM user access keys for live cluster mutations — use STS-assumed roles with short TTL instead.
- Direct `cluster-admin` ClusterRoleBinding inside the cluster without accompanying RAM audit trail — bypasses ActionTrail.

## Audit Trail

All ACK mutations (node pool upgrades, cluster version changes, scaling operations) are logged in **ActionTrail** under the `cs` service. Ensure ActionTrail is enabled and delivering to OSS or SLS for the target region. Query events with `EventName` containing `ModifyCluster`, `ScaleCluster`, or `UpgradeCluster`.
