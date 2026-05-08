#!/usr/bin/env bash
# guards/velero-restore.sh — RBAC pre-flight for kubernetes-live-velero-restore-guard-agent
# Source lib/common.sh before running this file.

run_guard_velero_restore() {
  begin_guard "velero-restore"

  local SA="system:serviceaccount:vanguard-system:vanguard-velero-restore-guard"

  # -------------------------------------------------------------------------
  # Universal must-not-be-yes
  # -------------------------------------------------------------------------
  run_universal_must_not "$SA"

  # -------------------------------------------------------------------------
  # Domain-specific must-not-be-yes
  # (from agents/kubernetes/kubernetes-live-velero-restore-guard-agent/references/rbac-pre-flight.md)
  # All Velero CRDs are absent in vanilla kind — use assert_cannot_or_skip.
  # -------------------------------------------------------------------------
  printf '%b  -- domain-specific must-not-be-yes (Velero CRDs, SKIP if absent) --%b\n' "$_CYAN" "$_RESET"

  # Schedule writes — operator install only
  assert_cannot_or_skip create schedules.velero.io -n velero "--as=$SA"
  assert_cannot_or_skip patch schedules.velero.io -n velero "--as=$SA"
  assert_cannot_or_skip delete schedules.velero.io -n velero "--as=$SA"

  # BackupStorageLocation writes — security-critical (s3 credentials)
  assert_cannot_or_skip patch backupstoragelocations.velero.io -n velero "--as=$SA"
  assert_cannot_or_skip delete backupstoragelocations.velero.io -n velero "--as=$SA"

  # Backup deletion — rollback option loss
  assert_cannot_or_skip delete backups.velero.io -n velero "--as=$SA"
  assert_cannot_or_skip patch backups.velero.io -n velero "--as=$SA"

  # Velero control plane (standard resources — not CRD-dependent)
  assert_cannot patch deployments -n velero "--as=$SA"
  assert_cannot get secrets -n velero "--as=$SA"

  # -------------------------------------------------------------------------
  # Domain-specific must-be-yes
  # Velero CRDs — skip not fail if absent
  # -------------------------------------------------------------------------
  printf '%b  -- domain-specific must-be-yes (Velero CRDs, SKIP if absent) --%b\n' "$_CYAN" "$_RESET"

  assert_can_or_skip create restores.velero.io -n velero "--as=$SA"
  assert_can_or_skip create backups.velero.io -n velero "--as=$SA"
  assert_can_or_skip list backups.velero.io -n velero "--as=$SA"
  assert_can_or_skip list backupstoragelocations.velero.io -n velero "--as=$SA"
  assert_can_or_skip list restores.velero.io -n velero "--as=$SA"

  report_guard "velero-restore"
}
