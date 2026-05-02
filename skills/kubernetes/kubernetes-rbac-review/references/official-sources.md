# Official Sources

Load these only when needed:

- [Using RBAC Authorization](https://kubernetes.io/docs/reference/access-authn-authz/rbac/) — use for Role, ClusterRole, RoleBinding, ClusterRoleBinding structure, aggregation rules, default roles, and `kubectl auth` usage.
- [RBAC Good Practices](https://kubernetes.io/docs/concepts/security/rbac-good-practices/) — use for least privilege, wildcard cautions, privilege escalation paths, impersonation risks, and workload namespace isolation.
- [Authorization Overview](https://kubernetes.io/docs/reference/access-authn-authz/authorization/) — use when confirming how Kubernetes evaluates requests and which authorizers are stacked.
- [Configure Service Accounts for Pods](https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/) — use for `automountServiceAccountToken`, dedicated ServiceAccount patterns, and token projection.
- [Kubernetes Security Checklist](https://kubernetes.io/docs/concepts/security/security-checklist/) — use for a holistic posture check covering RBAC alongside admission, network policies, and pod security.
- [Bound Service Account Tokens](https://kubernetes.io/docs/reference/access-authn-authz/service-accounts-admin/#bound-service-account-tokens) — use when reviewing projected token lifetimes, audience binding, and migration from legacy auto-mounted tokens.

## Grounded insights worth carrying into the skill

- Kubernetes RBAC is additive: there are no deny rules. Any binding that grants a permission cannot be overridden by another binding.
- `pods/exec` and `pods/attach` are equivalent to remote-shell access on any pod; treat both as high-severity grants requiring explicit justification.
- `pods/portforward` tunnels arbitrary TCP to pod ports; treat as high-severity for database and internal-service workloads.
- `nodes/proxy` grants proxy access to the kubelet API on every node — this is effectively cluster-admin for node-level operations and is rarely justified for any non-system workload.
- `secrets` `get`/`list` access at ClusterRole scope means reading every secret in every namespace — almost always over-privileged for a workload.
- `escalate` on roles/clusterroles, `bind` on bindings, and `impersonate` on users/groups/serviceaccounts are Kubernetes' three dedicated privilege-escalation verbs. Any Role that grants these is high severity regardless of other verb restrictions — they allow the holder to exceed their own permission set.
- The `system:masters` group bypasses all RBAC checks, including admission webhooks; never bind real workloads to it.
- Aggregated ClusterRoles (`aggregationRule`) inherit rules from any ClusterRole that matches the label selector — third-party operators can silently expand them.
- Setting `automountServiceAccountToken: false` on the ServiceAccount (or the Pod spec) is the correct default for workloads that do not call the Kubernetes API.
