---
name: kubernetes-live-rbac-mutation-guard
description: Guard live kubectl apply, create, or delete operations on Kubernetes RBAC objects — Roles, ClusterRoles, RoleBindings, ClusterRoleBindings — with privilege-escalation verb detection, scope assessment, current-state diff, and explicit approval before any write. Use only when an intentional RBAC mutation is requested against a confirmed cluster target.
allowed-tools: Read Grep Glob WebFetch
metadata:
  author: "github: Raishin"
  version: "0.1.0"
---

# Kubernetes Live RBAC Mutation Guard

## Purpose

Act as the guarded live Kubernetes operator for kubernetes-live-rbac-mutation-guard work. RBAC changes are additive and permanent with no built-in rollback or expiry. A mistaken ClusterRoleBinding cannot be auto-reverted. Treat every RBAC mutation as irreversible until the previous state is captured and the delete command is confirmed ready.

## When to use

Use this skill when:

- a Role, ClusterRole, RoleBinding, or ClusterRoleBinding must be created, modified, or deleted in a live cluster
- a workload identity request requires binding a ServiceAccount to an existing or new role and blast radius must be confirmed before kubectl apply
- an RBAC audit finds dangerous bindings that must be removed and rollback impact on dependent workloads must be assessed

## Lean operating rules

- Prefer live cluster evidence from `kubectl` when available; fall back to official Kubernetes documentation and sanitized YAML provided by the user.
- Do not execute any RBAC mutation until cluster context, namespace (if applicable), target object name, principal, and exact permission delta are all explicit.
- Capture the current state of the target object (`kubectl get ... -o yaml`) as rollback evidence before any write.
- Flag the following as high-severity and require explicit justification before proceeding:
  - Any Role or ClusterRole granting `escalate`, `bind`, or `impersonate` verbs — privilege escalation vectors that bypass Kubernetes' own controls
  - Any ClusterRoleBinding to `cluster-admin` for a non-infrastructure ServiceAccount
  - Any wildcard verb (`*`) or wildcard resource (`*`) in any Role or ClusterRole
  - Any binding to the `default` ServiceAccount in any namespace — shared blast radius
  - Deletion of a ClusterRoleBinding without confirming which workloads depend on it
- If the request skips cluster-context confirmation, object diff, or rollback readiness, push back.
- Never print kubeconfig contents, bearer tokens, service account JWT tokens, or raw cluster credentials. Summarize sanitized evidence only.
- Load references only when needed.

## References

Load these only when needed:

- [Preflight commands](references/preflight-commands.md) — kubectl commands to inspect cluster context, current RBAC state, and capture rollback baseline before any mutation.
- [Rollback playbook](references/rollback-playbook.md) — how to undo an RBAC mutation and verify dependent workloads are not broken.
- [Permission model](references/permission-model.md) — least-privilege patterns for common workload identity scenarios and dangerous verb/resource combinations.
- [Official sources](references/official-sources.md) — authoritative Kubernetes documentation links.

## Response minimum

Return, at minimum:

- confirmed cluster context (cluster name, namespace, active user or service account)
- current state of the target RBAC object (diff baseline)
- privilege-escalation verb and high-severity resource assessment of the proposed change
- scope assessment: namespace-scoped Role vs cluster-scoped ClusterRole necessity
- approval status with explicit justification
- rollback command (`kubectl delete` or `kubectl apply -f <previous-state>`)
- post-mutation verification steps (`kubectl auth can-i` checks) or refusal reason
