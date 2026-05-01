# Workflow and Output Contract

## Workflow

### Step 1 — Identify the target scope

PSA configuration lives in two places:

1. **Per-namespace labels** — `pod-security.kubernetes.io/<mode>: <profile>` and `pod-security.kubernetes.io/<mode>-version: <version>`.
2. **Cluster `AdmissionConfiguration`** — applies a default profile to namespaces that don't carry a label, and exempts specific namespaces, users, or runtime classes globally.

Confirm which scope the review covers — a namespace label change is reversible by flipping the label; a cluster `AdmissionConfiguration` change requires control-plane access and a kube-apiserver restart.

Reference: [Pod Security Admission](https://kubernetes.io/docs/concepts/security/pod-security-admission/) and [Enforce standards via namespace labels](https://kubernetes.io/docs/tasks/configure-pod-container/enforce-standards-namespace-labels/).

### Step 2 — Identify the active profile and mode

Three profiles, defined in [Pod Security Standards](https://kubernetes.io/docs/concepts/security/pod-security-standards/):

1. **`privileged`** — unrestricted. No security context constraints. Equivalent to no admission control. Appropriate ONLY for system-level workloads (CNI, CSI drivers, monitoring agents).
2. **`baseline`** — minimally restrictive. Disallows known privilege escalations. Allows most legitimate application workloads with minimal modification. Recommended floor for application namespaces.
3. **`restricted`** — heavily restricted, follows current pod hardening best practices. Requires `runAsNonRoot: true`, `seccompProfile: RuntimeDefault`, no capabilities except `NET_BIND_SERVICE`, no host paths or host network, etc.

Three modes, applied via labels:

1. **`enforce`** — admission denied on violation. Pods that violate are rejected.
2. **`audit`** — admission allowed; violation recorded in API server audit log.
3. **`warn`** — admission allowed; violation returned to the user as a warning header (visible in `kubectl apply` output).

Each mode/profile combination can have an independent version pin: `pod-security.kubernetes.io/enforce-version: v1.30`.

### Step 3 — Audit profile-mode combinations

Common configurations and their findings:

- **`enforce: restricted`** — production-tier hardening. Verify all running pods admit; otherwise, the next pod restart will fail.
- **`enforce: baseline`, `audit: restricted`, `warn: restricted`** — common migration pattern. Hard floor at baseline, with restricted violations surfaced for cleanup.
- **`enforce: baseline`** alone — application namespace minimum. Confirm PSP-equivalent constraints are not assumed by other tooling.
- **`enforce: privileged`** — only acceptable for system namespaces with documented justification.
- **No PSA labels at all** — namespace falls back to cluster default. **Critical finding** if cluster default is `privileged` (the Kubernetes default unless changed).
- **`audit` and/or `warn` set but no `enforce`** — security violations are logged, not blocked. Useful as observability but not as control.

### Step 4 — Check whether existing workloads would still admit

Before flipping a namespace from `baseline` to `restricted`, verify every running pod meets the stricter profile. Use `--dry-run=server` to ask the API server to evaluate without applying:

```shell
kubectl label namespace <ns> pod-security.kubernetes.io/enforce=restricted --overwrite --dry-run=server
```

This returns warnings for any pod that would be rejected. Review those pods before applying the label.

Alternative: set `warn: restricted` first, watch for warnings in apply outputs and audit logs, fix workloads, then promote to `enforce: restricted`.

Stress-tests:

- Pods with `securityContext.runAsUser: 0` — restricted profile rejects.
- Pods with `securityContext.privileged: true` — baseline AND restricted reject.
- Pods with `securityContext.allowPrivilegeEscalation: true` — restricted rejects (baseline allows by default).
- Pods with `volumes.hostPath` — baseline rejects (only specific paths allowed).
- Pods with `securityContext.capabilities.add` containing anything beyond `NET_BIND_SERVICE` — restricted rejects.
- Pods with `securityContext.seccompProfile` not set or set to `Unconfined` — restricted rejects (must be `RuntimeDefault` or `Localhost`).

### Step 5 — Audit version pinning

`enforce-version`, `audit-version`, `warn-version` pin the profile semantics to a specific Kubernetes minor. Findings:

- **No version pin** — profile follows cluster's Kubernetes version. Each upgrade may tighten requirements.
- **`*-version: latest`** — explicitly tracks the latest profile. Same risk as no pin but with more honesty.
- **`*-version: v1.24`** on a 1.30 cluster — pinned to an old, less strict version. May allow workloads the current docs say should be denied.

Recommended: pin to the cluster's current minor (`v1.30` on a 1.30 cluster) and explicitly bump the pin during cluster upgrade reviews.

### Step 6 — Audit cluster-level exemptions

`AdmissionConfiguration` exemptions:

```yaml
exemptions:
  usernames: ["system:admin"]
  runtimeClasses: ["sandboxed"]
  namespaces: ["kube-system"]
```

Findings:

- `usernames` exemption with broad bindings — the exempted user can run any pod regardless of namespace label.
- `runtimeClasses` exemption for non-sandboxed runtimes — exempts pods using that runtime class entirely.
- `namespaces` exemption for `kube-system` and operator namespaces — common, but every operator should be reviewed for whether it actually needs to run pods that violate baseline.

### Step 7 — Audit migration from PodSecurityPolicy

PSP was removed in Kubernetes 1.25. If the user is migrating:

- Use `kubectl-psp-to-psa` (or equivalent) to translate existing PSP definitions into PSA labels.
- The translation is not always exact — PSP allowed per-Pod conditions; PSA is per-namespace.
- Some PSP capabilities (e.g., specific `runAsUser` ranges, specific FSGroup ranges) cannot be expressed in PSA — Kyverno or OPA Gatekeeper is needed for these.
- Verify the old PSP webhook is removed AFTER PSA is enforced — running both simultaneously can produce conflicting decisions.

Reference: [Migrating from PodSecurityPolicy](https://kubernetes.io/docs/tasks/configure-pod-container/migrate-from-psp/).

### Step 8 — Stress-test operational hygiene

- Prefer `enforce: baseline` minimum for application namespaces; `enforce: restricted` for tiers without legacy workloads.
- Prefer pinned `*-version` matching cluster minor.
- Prefer namespace-by-namespace promotion (`warn` → `audit` → `enforce`) over cluster-wide flips.
- Prefer per-workload `securityContext` hardening over namespace exemption when one workload needs special access.
- Verify that other admission policies (Kyverno, OPA Gatekeeper) extend rather than weaken PSA — a downstream policy that allows what PSA denies still results in the pod being rejected by PSA first.

## Output

Return:

- **target**: namespace, namespace set, or cluster `AdmissionConfiguration`,
- **evidence level**: `live evidence` / `documentation-based` / `sanitized user evidence` / `inference`,
- **active configuration**: profile and mode per scope, with version pin,
- **admission impact**: which currently-running pods would be rejected at the proposed profile,
- **exemption posture**: cluster-level exemptions and per-namespace label overrides,
- **risk findings** (with severity: high / medium / low),
- **safest next actions** with sample manifest changes and the recommended `warn` → `audit` → `enforce` rollout,
- **rollback plan**: how to remove or weaken the label if running workloads break,
- **assumptions and missing facts**.

## Security notes

- Never recommend `enforce: privileged` for an application namespace.
- Never recommend removing the namespace PSA label without a documented replacement (cluster default or another admission engine).
- Never recommend exempting a namespace cluster-wide without confirming the workloads inside cannot be hardened.
- Do not print pod environment variables, init container args, or any pod content beyond the security context.
