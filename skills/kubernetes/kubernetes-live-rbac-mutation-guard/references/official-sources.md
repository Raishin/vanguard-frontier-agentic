# Official Sources

Load these only when needed:

- [Using RBAC Authorization](https://kubernetes.io/docs/reference/access-authn-authz/rbac/) — use for Role/ClusterRole structure, aggregation rules, `kubectl auth can-i`, privilege escalation prevention (`escalate`, `bind`, `impersonate`), and default ClusterRole reference.
- [RBAC Good Practices](https://kubernetes.io/docs/concepts/security/rbac-good-practices/) — use for wildcard cautions, escalation path analysis, ServiceAccount least privilege, impersonation risks, and namespace isolation.
- [kubectl auth reference](https://kubernetes.io/docs/reference/kubectl/generated/kubectl_auth/) — use for `kubectl auth can-i`, `kubectl auth whoami`, and `kubectl auth reconcile` syntax.
- [Configure Service Accounts](https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/) — use for `automountServiceAccountToken`, dedicated ServiceAccount patterns, and token projection volume.
- [Kubernetes Security Checklist](https://kubernetes.io/docs/concepts/security/security-checklist/) — use for a holistic posture check covering RBAC, pod security, network policies, and admission.

## Grounded insights worth carrying into the skill

- `kubectl apply --dry-run=client` validates the YAML locally but does **not** check against the API server's admission webhooks or existing RBAC state. Always follow with a review of the proposed rules.
- Kubernetes audit logs are the authoritative record of what was done under a binding. Ensure audit logging is enabled and retained before any RBAC mutation.
- `kubectl auth reconcile -f rbac.yaml` applies RBAC from file while **preserving** extra permissions not in the file — it is not an idempotent replace. Use `kubectl apply` with server-side apply (`--server-side`) for deterministic state.
- Deleting a ClusterRoleBinding does not immediately revoke access for pods with cached tokens. The cached service account token remains valid until it expires (default 1 hour for projected tokens, longer for legacy auto-mounted tokens). Plan maintenance windows accordingly.
- The `system:masters` group is hardcoded in the Kubernetes API server and bypasses all RBAC and admission webhook checks. Never use it for real workloads; it exists only for emergency break-glass recovery.
- Aggregated ClusterRoles (`aggregationRule`) inherit rules from any ClusterRole matching the label selector. Third-party Helm charts that add aggregation labels can silently expand your aggregated ClusterRoles after installation.
