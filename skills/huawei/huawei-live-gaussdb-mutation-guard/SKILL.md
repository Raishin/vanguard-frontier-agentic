---
name: huawei-live-gaussdb-mutation-guard
description: Gate GaussDB/RDS instance deletion, spec downgrade, and backup policy removal — database deletion is permanently destructive and MLPS Level 3 data destruction triggers mandatory 24-hour incident reporting.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: database
---

# Huawei Live GaussDB Mutation Guard

## Purpose

Act as the guarded live Huawei Cloud operator for huawei-live-gaussdb-mutation-guard work. Gate GaussDB/RDS instance deletion, spec downgrade, and backup policy removal. Insist on CBR backup verification, MLPS Level 3 incident obligation assessment, and rollback posture evidence before execution. Treat any ambiguous approval, unverified backup, or MLPS Level 3 workload as a stop condition.

## When to Use

Use this skill when:

- A GaussDB or RDS instance is being deleted
- A GaussDB or RDS instance flavor is being downgraded (spec reduction)
- A GaussDB or RDS backup policy is being removed, reduced in frequency, or disabled
- A PITR (point-in-time recovery) log backup is being disabled
- A CBR vault associated with a database is being deleted or its policy modified
- An MLPS Level 3 classified database is being modified in a way that affects data integrity or availability

## When NOT to Use

Do not use this skill when:

- The task is read-only database inspection with no mutation intent
- The task involves creating a new database instance (no existing data at risk)
- The task involves ECS, CCE, or other non-database compute

## Gate Protocol

This skill requires the 6-step live-guard gate from the maestro. See `skills/huawei/huawei-maestro/SKILL.md` for the full gate protocol. The 6 steps are:

1. **Surface risk** — state the specific risk: permanent data loss, spec-induced query failure, or loss of recovery capability.
2. **State irreversibility** — explicitly state: database deletion is irreversible without backup; spec downgrade may cause in-flight query failures.
3. **Confirm target** — confirm exact instance name, account, enterprise project, and region.
4. **Assess blast radius** — enumerate all applications dependent on this instance, replication targets, and dependent read replicas.
5. **Require rollback path** — verify CBR backup exists and is restorable; document recovery procedure.
6. **Get explicit written confirmation** — require the operator to state instance name, action, and MLPS classification before proceeding.

## Pre-Flight Checklist

Before executing any GaussDB/RDS mutation, verify all of the following:

1. **Instance identity confirmed** — confirm instance name, engine type, version, region, and enterprise project.
2. **CBR backup verified** — confirm a recent (within 24 hours) CBR backup exists and is in a restorable state.
3. **MLPS Level 3 classification checked** — determine if this database is subject to MLPS Level 3; if yes, data destruction triggers mandatory 24-hour incident reporting.
4. **Dependent applications enumerated** — list all application connection strings pointing to this instance or its proxy.
5. **Spec downgrade impact assessed** — for flavor downgrade: identify in-flight queries and connection pool capacity impact.
6. **Backup policy removal impact assessed** — for backup policy removal: document the RPO gap created.
7. **Approval documented** — obtain explicit written operator approval including instance name and action.

## Required Confirmation

The operator must explicitly state all of the following before any mutation is executed:

- "I confirm the target instance is `<INSTANCE_NAME>` in enterprise project `<ENTERPRISE_PROJECT>`, account `<ACCOUNT_ID>`, region `<REGION>`."
- "I confirm the action is `<DESCRIBE_ACTION>` (deletion / spec downgrade / backup policy removal)."
- "I confirm a verified CBR backup exists dated `<BACKUP_DATE>` and is restorable."
- "I confirm this database `[IS / IS NOT]` subject to MLPS Level 3 classification."
- "I approve this database mutation."

For MLPS Level 3 databases, additionally require:
- "I confirm that if this action constitutes data destruction, I will file the mandatory incident report within 24 hours."

## Rollback Procedure

- **Instance deletion** (irreversible without backup): restore from CBR backup via CBR console — restoration creates a new instance; original instance ID is lost.
- **Spec downgrade** (limited reversibility): upgrade spec back to previous flavor; in-flight queries that failed must be retried by the application.
- **Backup policy removal** (reversible): re-enable backup policy via RDS/GaussDB console; backup gap cannot be retroactively filled.
- Document all actions in CTS and the associated change ticket.

## Post-Change Verification

1. For deletion: confirm instance is absent from GaussDB/RDS console.
2. For spec downgrade: confirm new flavor, verify all connections re-established.
3. For backup policy change: confirm backup policy status, verify next scheduled backup.
4. Check CES alarms for database performance anomalies following change.

## Response Shape

1. Instance identity confirmed
2. CBR backup verification status
3. MLPS Level 3 classification
4. Dependent application enumeration
5. Blast radius assessment
6. Approval status
7. Executed action
8. Post-change verification
