#!/usr/bin/env bash
# guards/rbac-mutation.sh — RBAC pre-flight for kubernetes-live-rbac-mutation-guard-agent
# Source lib/common.sh before running this file.

run_guard_rbac_mutation() {
  begin_guard "rbac-mutation"

  local SA="system:serviceaccount:vanguard-system:vanguard-rbac-mutation-guard"

  # -------------------------------------------------------------------------
  # Universal must-not-be-yes
  # -------------------------------------------------------------------------
  run_universal_must_not "$SA"

  # -------------------------------------------------------------------------
  # Domain-specific must-not-be-yes
  # (from agents/kubernetes/kubernetes-live-rbac-mutation-guard-agent/references/rbac-pre-flight.md)
  # -------------------------------------------------------------------------
  printf '%b  -- domain-specific must-not-be-yes --%b\n' "$_CYAN" "$_RESET"

  # Cluster-scoped RBAC writes — opt-in only; default refusal
  assert_cannot create clusterroles.rbac.authorization.k8s.io "--as=$SA"
  assert_cannot create clusterrolebindings.rbac.authorization.k8s.io "--as=$SA"
  assert_cannot patch clusterroles.rbac.authorization.k8s.io "--as=$SA"
  assert_cannot patch clusterrolebindings.rbac.authorization.k8s.io "--as=$SA"

  # Privilege-escalation primitives
  assert_cannot escalate roles.rbac.authorization.k8s.io --all-namespaces "--as=$SA"
  assert_cannot bind roles.rbac.authorization.k8s.io --all-namespaces "--as=$SA"
  assert_cannot escalate clusterroles.rbac.authorization.k8s.io "--as=$SA"
  assert_cannot bind clusterroles.rbac.authorization.k8s.io "--as=$SA"
  assert_cannot impersonate users "--as=$SA"
  assert_cannot impersonate groups "--as=$SA"
  assert_cannot impersonate serviceaccounts --all-namespaces "--as=$SA"

  # Delete — rollback is via apply -f baseline
  assert_cannot delete roles.rbac.authorization.k8s.io --all-namespaces "--as=$SA"
  assert_cannot delete rolebindings.rbac.authorization.k8s.io --all-namespaces "--as=$SA"

  # ServiceAccount creation (separate from RBAC; could be used to create a privileged SA)
  assert_cannot create serviceaccounts --all-namespaces "--as=$SA"

  # -------------------------------------------------------------------------
  # Domain-specific must-be-yes
  # -------------------------------------------------------------------------
  printf '%b  -- domain-specific must-be-yes --%b\n' "$_CYAN" "$_RESET"

  assert_can create roles.rbac.authorization.k8s.io --all-namespaces "--as=$SA"
  assert_can patch roles.rbac.authorization.k8s.io --all-namespaces "--as=$SA"
  assert_can create rolebindings.rbac.authorization.k8s.io --all-namespaces "--as=$SA"
  assert_can patch rolebindings.rbac.authorization.k8s.io --all-namespaces "--as=$SA"
  assert_can list rolebindings.rbac.authorization.k8s.io --all-namespaces "--as=$SA"
  assert_can list serviceaccounts --all-namespaces "--as=$SA"

  report_guard "rbac-mutation"
}
