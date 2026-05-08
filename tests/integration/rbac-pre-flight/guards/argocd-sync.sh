#!/usr/bin/env bash
# guards/argocd-sync.sh — RBAC pre-flight for kubernetes-live-argocd-sync-guard-agent
# Source lib/common.sh before running this file.

run_guard_argocd_sync() {
  begin_guard "argocd-sync"

  local SA="system:serviceaccount:vanguard-system:vanguard-argocd-sync-guard"

  # -------------------------------------------------------------------------
  # Universal must-not-be-yes
  # -------------------------------------------------------------------------
  run_universal_must_not "$SA"

  # -------------------------------------------------------------------------
  # Domain-specific must-not-be-yes
  # (from agents/kubernetes/kubernetes-live-argocd-sync-guard-agent/references/rbac-pre-flight.md)
  # All Argo CD CRDs are absent in vanilla kind — use assert_cannot_or_skip.
  # -------------------------------------------------------------------------
  printf '%b  -- domain-specific must-not-be-yes (Argo CD CRDs, SKIP if absent) --%b\n' "$_CYAN" "$_RESET"

  # Delete on Argo CD resources — rollback via apply -f baseline
  assert_cannot_or_skip delete applications.argoproj.io --all-namespaces "--as=$SA"
  assert_cannot_or_skip delete applicationsets.argoproj.io --all-namespaces "--as=$SA"
  assert_cannot_or_skip delete appprojects.argoproj.io --all-namespaces "--as=$SA"

  # AppProject and ApplicationSet writes — GitOps-only territory
  assert_cannot_or_skip create appprojects.argoproj.io -n argocd "--as=$SA"
  assert_cannot_or_skip patch appprojects.argoproj.io -n argocd "--as=$SA"
  assert_cannot_or_skip create applicationsets.argoproj.io -n argocd "--as=$SA"
  assert_cannot_or_skip patch applicationsets.argoproj.io -n argocd "--as=$SA"

  # Argo CD control plane (standard resources — not CRD-dependent)
  assert_cannot patch deployments -n argocd "--as=$SA"
  assert_cannot patch configmaps -n argocd "--as=$SA"
  assert_cannot get secrets -n argocd "--as=$SA"

  # -------------------------------------------------------------------------
  # Domain-specific must-be-yes
  # Argo CD CRDs — skip not fail if absent
  # -------------------------------------------------------------------------
  printf '%b  -- domain-specific must-be-yes (Argo CD CRDs, SKIP if absent) --%b\n' "$_CYAN" "$_RESET"

  assert_can_or_skip list applications.argoproj.io -n argocd "--as=$SA"
  assert_can_or_skip list applicationsets.argoproj.io -n argocd "--as=$SA"
  assert_can_or_skip list appprojects.argoproj.io -n argocd "--as=$SA"
  assert_can_or_skip patch applications.argoproj.io -n argocd "--as=$SA"

  report_guard "argocd-sync"
}
