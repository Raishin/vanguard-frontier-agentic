# IAM Permissions — Alibaba Cloud Live RDS PolarDB Mutation Guard

## Minimum Read Permissions (Discovery and Audit)

| Policy | Purpose |
|--------|---------|
| `AliyunRDSReadOnlyAccess` | Describe RDS instance metadata, backup retention, parameter groups, and connection configurations without mutation rights |
| `AliyunPolardbReadOnlyAccess` | Describe PolarDB cluster metadata, backup retention, proxy configuration, and GDN topology without mutation rights |

## Required Mutation Permissions (Live Operations)

| Policy | Purpose | Scope Guidance |
|--------|---------|---------------|
| `AliyunRDSFullAccess` | Delete RDS instances, modify spec, or change backup policy | Must confirm exact instance ID and verified backup retention before assuming this policy; never granted as standing access |
| `AliyunPolardbFullAccess` | Delete PolarDB clusters, modify spec, or change backup policy | Must confirm exact cluster ID and verified backup retention before assuming this policy; never granted as standing access |

## Narrowing Guidance

- Confirm the exact instance/cluster ID and region before any mutation. Instance IDs are unique per region.
- `AliyunRDSFullAccess` and `AliyunPolardbFullAccess` should be assumed via STS for specific approved operations only — never standing policy attachments.
- Use RAM conditions to restrict database mutations to specific instance IDs or regions where the RAM policy language permits.
- Prefer read-only policies for all pre-flight backup verification and dependency audit operations.

## Anti-Patterns — Never Grant

- `AdministratorAccess` — account-wide full control; never appropriate for a database guard agent.
- `AliyunRDSFullAccess` or `AliyunPolardbFullAccess` as standing attachments to any RAM user or operational role.
- Long-lived RAM user access keys with database full access — if compromised, the attacker can delete all database instances.
- Granting database full access to CI/CD pipeline service accounts without instance-scoped resource conditions.
- Deleting an instance without verifying backup retention period and last successful backup timestamp.

## Audit Trail

All RDS/PolarDB mutations (instance deletion, spec modification, backup policy changes) are logged in **ActionTrail** under the `rds` and `polardb` services. Ensure ActionTrail is enabled for the target region and delivering to OSS or SLS. Query events with `EventName` containing `DeleteDBInstance`, `DeleteDBCluster`, `ModifyDBInstanceSpec`, or `ModifyBackupPolicy`. Monitor for unexpected instance deletions via CloudMonitor RDS event alarms.
