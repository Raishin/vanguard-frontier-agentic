---
name: alibaba-live-rds-polardb-mutation-guard
description: Gate RDS/PolarDB instance deletion, spec downgrade, and backup policy removal — database deletion without verified backup is permanently destructive.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: database
---

# Alibaba Cloud Live RDS/PolarDB Mutation Guard

## Purpose

Act as the guarded live Alibaba Cloud operator for alibaba-live-rds-polardb-mutation-guard work. Gate every RDS or PolarDB instance deletion, spec downgrade, and backup policy removal with a complete backup verification and explicit operator approval. Treat database deletion without verified backup as an irreversible data-loss event.

## When to Use

Use this skill when:

- An RDS or PolarDB instance deletion is being requested
- A spec downgrade (instance type reduction) is being planned for an RDS or PolarDB instance
- A backup policy is being removed or modified to reduce retention period or disable automated backups
- An operator needs to verify backup status and restore readiness before any destructive database operation
- A database endpoint or high-availability configuration change is being made during business hours

## When NOT to Use

Do not use this skill when:

- The task is a read-only RDS/PolarDB audit with no mutation intent
- The task involves only SQL query execution or data-level operations
- The task involves creating a new RDS/PolarDB instance (no existing data at risk)
- The task involves minor parameter group changes that do not affect availability or data retention

## Key Risk Facts

- **RDS/PolarDB instance deletion** removes all instance data, parameter groups, and backups associated with the instance immediately upon deletion (backups may have a brief retention window depending on configuration — verify explicitly). This action cannot be reversed.
- **Spec downgrade during peak hours** causes connection resets and query timeouts during the instance resize window. Downgrading to an instance type that cannot handle current load causes cascading failures in dependent applications.
- **Backup policy removal** leaves the database unprotected for the period between the last backup and the next backup cycle. If the instance fails during this window, data from that period is lost.
- **All three actions** require the 6-step live-guard gate: (1) identity confirm, (2) backup verification, (3) blast radius assessment, (4) explicit approval, (5) execution, (6) post-change verification.

## Pre-Flight Checklist

Before executing any RDS or PolarDB mutation, verify all of the following:

1. **Instance identity confirmed** — confirm the exact instance ID, region, engine type (MySQL/PG/SQL Server/MariaDB for RDS; MySQL/PG/Oracle for PolarDB), and account. Run `aliyun rds DescribeDBInstances` or check PolarDB console to confirm.
2. **Backup existence verified** — confirm a recent automated or manual backup exists. Check: backup creation time, backup status (Completed), backup size, and retention period. A backup that cannot be listed is not a verified backup.
3. **Restore tested or documented** — for critical instances, verify the backup is restorable by checking the last restore test date. If never tested, flag as high risk.
4. **Current spec and load assessed** — for downgrade: compare current CPU/memory/IOPS utilization to the target spec limits. If current utilization is within 20% of target spec limits, the downgrade is high risk.
5. **Blast radius assessed** — which applications depend on this instance? what is the connection string? what downstream services will fail during the maintenance window?
6. **Change window confirmed** — spec downgrades and deletions should be executed during a planned maintenance window with application owners notified.

## Required Confirmation

The operator must explicitly state all of the following before any mutation is executed:

- "I confirm the instance is `<INSTANCE_ID>` of type `<ENGINE>` in region `<REGION>` in account `<ACCOUNT_ID>`."
- "I have verified a backup exists: backup ID `<BACKUP_ID>`, created at `<TIMESTAMP>`, status `<Completed>`."
- "I have assessed the blast radius: `<DESCRIPTION OF DEPENDENT APPLICATIONS>`."
- "I understand that instance deletion is permanent and all data will be irrecoverable without a verified backup."
- "I approve this `<deletion / spec downgrade / backup policy change>`."
- For deletion: "I confirm the backup is restorable and all dependent applications have been notified."
- For spec downgrade: "I confirm the current peak utilization is `<RATE>%` of the target spec and the change window is `<WINDOW>`."

## Execution Steps

1. Capture pre-change instance state: instance metadata, current spec, backup list.
2. Verify backup status and restore readiness.
3. Present the planned action, backup verification results, and blast radius to the operator for explicit approval.
4. Execute the mutation:
   - Delete instance: `aliyun rds DeleteDBInstance --DBInstanceId <ID>` or PolarDB equivalent.
   - Spec downgrade: `aliyun rds ModifyDBInstanceSpec` or PolarDB `ModifyDBNodeClass` — schedule during maintenance window.
   - Backup policy change: `aliyun rds ModifyBackupPolicy` or PolarDB equivalent.
5. Confirm the action is completed and document the result.

## Rollback Procedure

- **Backup policy change** (reversible): Restore the previous backup policy settings immediately.
- **Spec downgrade** (reversible): Re-upgrade to the previous spec via `aliyun rds ModifyDBInstanceSpec`. Note: re-upgrade takes time and a connection reset occurs again.
- **Instance deletion** (NOT reversible without backup): If a backup exists, restore from backup to a new instance. Restoration takes time proportional to database size. The original endpoint is gone — applications must be reconfigured to the new instance endpoint.
- **Instance deletion without backup** (NOT reversible): There is no recovery path. Contact Alibaba Cloud Support immediately — recovery is not guaranteed.

## Post-Change Verification

1. For deletion: confirm instance is removed from the console and DNS no longer resolves.
2. For spec downgrade: confirm the new instance spec is active and monitor CPU/memory/IOPS metrics for the first 30 minutes.
3. For backup policy change: confirm the new policy is active and the next backup is scheduled.
4. Check ActionTrail for the database mutation event.
5. Monitor CloudMonitor for application-level connection errors or latency spikes.

## Response Shape

1. Instance ID, engine type, region, and account confirmed
2. Backup existence and restore readiness verified
3. Blast radius and dependent application assessment
4. Change window and load assessment (for spec downgrade)
5. Operator confirmation received
6. Execution confirmation
7. Post-change monitoring results
