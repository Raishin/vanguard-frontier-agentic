---
name: velero-backup-restore-guard
description: Use this skill when guarding Velero backup schedule changes, restore operations, BackupStorageLocation mutations, or volume snapshot configuration. Trigger on any request to run a velero restore, delete a Schedule, change a BSL default, or modify backup retention.
allowed-tools: Read Grep Glob WebFetch
metadata:
  author: "github: Raishin"
  version: "0.1.0"
---

# Velero Backup/Restore Guard

## Purpose

Guard live Velero operations — restore execution, schedule deletion, BackupStorageLocation changes, and volume snapshot configuration — against data loss, scope creep, and missing rollback posture. A Velero restore is destructive: it overlays resources onto the cluster. Every guarded operation requires confirmed cluster context, explicit scope, current state capture, and explicit platform-team sign-off before any mutation executes.

## Lean operating rules

- Confirm cluster context (`kubectl config current-context`) and target namespace before any Velero operation — ambiguous context is a hard stop.
- Capture current state of the target Backup, Schedule, or BSL (`velero backup describe <name> --details`, `kubectl get schedule <name> -o yaml`) before every write — Velero has no built-in undo.
- For restore operations: require `includedNamespaces` to be explicitly scoped; a cluster-wide restore (`includedNamespaces: []`) requires explicit platform-team sign-off.
- Recommend `velero restore create --dry-run` before every non-emergency restore; treat missing dry-run as a hard stop in non-emergency scenarios.
- Block deleting a Schedule that is the only backup for a production namespace unless an alternative backup source is confirmed.
- Block changing a BSL `default: true` without confirming no in-progress backups and reviewing the impact on all dependent Schedules.
- Check pre-backup hook coverage on stateful workloads (PostgreSQL, MySQL, Kafka) — missing quiesce hooks mean inconsistent backups.
- Label all claims as live evidence, documentation-based, or inference.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md)
- [Safety checklist](references/safety-checklist.md)

## Response minimum

- Confirmed cluster context and target scope
- Current state of the Backup/Schedule/BSL (evidence level)
- Hard-stop assessment (is this a blocked operation?)
- Explicit platform-team sign-off status
- Recommended dry-run or safe-path command
- Rollback posture
- Post-operation verification steps
