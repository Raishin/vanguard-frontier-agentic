#!/usr/bin/env bash
# guards/network-policy.sh — RBAC pre-flight for kubernetes-live-network-policy-guard-agent
# Source lib/common.sh before running this file.

run_guard_network_policy() {
  begin_guard "network-policy"

  local SA="system:serviceaccount:vanguard-system:vanguard-network-policy-guard"

  # -------------------------------------------------------------------------
  # Universal must-not-be-yes
  # -------------------------------------------------------------------------
  run_universal_must_not "$SA"

  # -------------------------------------------------------------------------
  # Domain-specific must-not-be-yes
  # (from agents/kubernetes/kubernetes-live-network-policy-guard-agent/references/rbac-pre-flight.md)
  # -------------------------------------------------------------------------
  printf '%b  -- domain-specific must-not-be-yes --%b\n' "$_CYAN" "$_RESET"

  # Cluster-wide policy writes — opt-in only; Cilium CRDs may not exist in kind
  assert_cannot_or_skip create ciliumclusterwidenetworkpolicies.cilium.io "--as=$SA"
  assert_cannot_or_skip patch ciliumclusterwidenetworkpolicies.cilium.io "--as=$SA"
  assert_cannot_or_skip delete ciliumclusterwidenetworkpolicies.cilium.io "--as=$SA"

  # Delete on namespaced policies — rollback is via apply -f baseline, not delete
  assert_cannot_or_skip delete ciliumnetworkpolicies.cilium.io --all-namespaces "--as=$SA"
  assert_cannot delete networkpolicies.networking.k8s.io --all-namespaces "--as=$SA"

  # Cilium agent ConfigMap
  assert_cannot patch configmaps/cilium-config -n kube-system "--as=$SA"

  # -------------------------------------------------------------------------
  # Domain-specific must-be-yes
  # -------------------------------------------------------------------------
  printf '%b  -- domain-specific must-be-yes --%b\n' "$_CYAN" "$_RESET"

  # Cilium CRDs — skip not fail if CRDs absent
  assert_can_or_skip create ciliumnetworkpolicies.cilium.io --all-namespaces "--as=$SA"
  assert_can_or_skip patch ciliumnetworkpolicies.cilium.io --all-namespaces "--as=$SA"
  assert_can_or_skip create ciliumegressgatewaypolicies.cilium.io --all-namespaces "--as=$SA"
  assert_can_or_skip list ciliumnetworkpolicies.cilium.io --all-namespaces "--as=$SA"
  assert_can_or_skip list ciliumendpoints.cilium.io --all-namespaces "--as=$SA"

  # Core NetworkPolicy (always present)
  assert_can create networkpolicies.networking.k8s.io --all-namespaces "--as=$SA"

  report_guard "network-policy"
}
