# Official Sources

Load these only when needed:

- [Pod Security Admission](https://kubernetes.io/docs/concepts/security/pod-security-admission/) — use as the entry point. Covers labels, modes, version pinning, and the admission controller behavior.
- [Pod Security Standards](https://kubernetes.io/docs/concepts/security/pod-security-standards/) — use for the exact field-level requirements of `privileged`, `baseline`, and `restricted` profiles.
- [Enforce Pod Security Standards via namespace labels](https://kubernetes.io/docs/tasks/configure-pod-container/enforce-standards-namespace-labels/) — use for the recommended rollout pattern (`warn` → `audit` → `enforce`).
- [Enforce Pod Security Standards by configuring the built-in admission controller](https://kubernetes.io/docs/tasks/configure-pod-container/enforce-standards-admission-controller/) — use for the cluster-wide `AdmissionConfiguration` syntax and exemption rules.
- [Migrating from PodSecurityPolicy](https://kubernetes.io/docs/tasks/configure-pod-container/migrate-from-psp/) — use when the cluster is moving from PSP to PSA.
- [Kubernetes Security Checklist](https://kubernetes.io/docs/concepts/security/security-checklist/) — use for the broader security context that PSA fits into (RBAC, NetworkPolicy, secrets, etc.).
- [Configure a Security Context for a Pod or Container](https://kubernetes.io/docs/tasks/configure-pod-container/security-context/) — use when reviewing per-pod `securityContext` hardening (the actual fields PSA evaluates).
- [seccomp profile for a container](https://kubernetes.io/docs/tutorials/security/seccomp/) — use for `RuntimeDefault` vs `Localhost` profile semantics that the restricted profile requires.
- [Kubernetes API audit logs](https://kubernetes.io/docs/tasks/debug/debug-cluster/audit/) — use to find admission decisions when PSA mode is `audit`.

## Grounded insights worth carrying into the skill

- Pod Security Admission was introduced in 1.22 (alpha), promoted to beta in 1.23, and shipped stable in 1.25 — replacing the deprecated PodSecurityPolicy in the same release. Any cluster running 1.25 or later does not have PSP available.
- Three profiles enforce a fixed set of pod spec constraints: `privileged` (none), `baseline` (deny known escalations), `restricted` (current best-practice hardening). A namespace can have a different profile per mode (`enforce`, `audit`, `warn`).
- The most common rollout pattern is: set `warn` and `audit` to the target profile, observe admission warnings and audit log violations, fix workloads, then promote `enforce` to the target profile. This avoids breaking running workloads at flip time.
- A namespace with no PSA label inherits the cluster default. The Kubernetes default is `privileged` unless the cluster admin set a stricter default in `AdmissionConfiguration`. Many production clusters silently run with privileged-equivalent admission because no label was set.
- Profile semantics evolve across Kubernetes versions. The `enforce-version`, `audit-version`, `warn-version` labels pin profile semantics to a specific Kubernetes minor. Without pinning, a cluster upgrade can suddenly reject pods that previously admitted. Pinning is recommended.
- Exemptions via `AdmissionConfiguration` (cluster-wide) bypass PSA entirely for the matched username, runtime class, or namespace. These are the broadest escape hatches and should be reviewed regularly. Per-namespace label exemptions (`pod-security.kubernetes.io/enforce: privileged`) are scoped to one namespace and easier to audit.
- The restricted profile requires `runAsNonRoot: true`, `runAsUser != 0`, `allowPrivilegeEscalation: false`, no `capabilities.add` other than `NET_BIND_SERVICE`, `seccompProfile.type: RuntimeDefault` or `Localhost`, no host namespaces, no host paths, no `hostPort`, and no privileged or unsafe sysctls. Many off-the-shelf operators do not meet this.
- Kyverno and OPA Gatekeeper can layer on top of PSA — they evaluate after PSA admission. This means a Kyverno policy that allows what PSA denies cannot rescue the pod; PSA's denial is final. Conversely, Kyverno can deny what PSA allows, providing a stricter-than-PSA layer.
- The `kubectl-psp-to-psa` plugin (community-maintained) translates PSP definitions into the closest equivalent PSA labels. The translation is lossy when PSPs encoded per-pod constraints (e.g., specific `runAsUser` ranges).
- `system:masters` group bypasses all admission controllers including PSA. Only the cluster-control-plane bootstrap should hold this; never bind real workloads to it.
