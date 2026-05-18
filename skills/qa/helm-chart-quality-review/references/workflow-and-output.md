# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized files (no live credentials, no kubeconfig, no cluster tokens — replace secrets with placeholders):
- `Chart.yaml` — name, version, appVersion, dependencies
- `values.yaml` — default values and their inline documentation
- `values.schema.json` — JSON Schema validation for values (if present)
- `templates/` — all template manifests (Deployment, StatefulSet, DaemonSet, Service, ConfigMap, Secret, RBAC resources, ServiceAccount, HPA, PDB, etc.)
- `tests/` — helm test hook manifests
- CI configuration (`ct.yaml`, `.github/workflows/*.yml`, `.gitlab-ci.yml`, or equivalent) if available

If `values.schema.json` is absent, note it and flag as MEDIUM. If `tests/` is absent, note it and flag as MEDIUM. If CI configuration is not provided, state findings about chart-testing CI as `inference`.

---

### Step 2 — Linting and template correctness audit

Check for structural and syntactic correctness.

- Missing required `Chart.yaml` fields (`apiVersion`, `name`, `version`) → flag as blocking; the chart fails `helm lint`.
- Chart `version` not semver-compliant (e.g. `1.0` instead of `1.0.0`) → LOW.
- Undefined template variables that would cause `helm template` to fail → HIGH.
- `helm template` renders manifests with empty required fields (image tag empty, pod name empty) → HIGH.
- No `NOTES.txt` providing post-install next steps → LOW.

---

### Step 3 — Values hygiene audit

Check for default value problems and schema coverage.

- Hardcoded image tags set to `:latest` with no override mechanism → HIGH (breaks reproducibility and rollback).
- Image digests hardcoded without a user-overridable `image.tag` or `image.digest` field → HIGH.
- Sensitive default values: empty password (`password: ""`), literal `admin` or `password` as a default credential → CRITICAL. Users deploy defaults to production without noticing.
- Required values with no `values.schema.json` type or pattern constraint → MEDIUM; `helm install` accepts arbitrary input with no validation.
- Deeply nested values with no inline comment documentation → LOW; operators cannot understand what to tune without reading templates.

---

### Step 4 — Template security audit

Check container and pod security configuration.

```yaml
# CRITICAL — container runs as root
securityContext:
  runAsUser: 0

# CRITICAL — privileged mode grants near-root kernel access
securityContext:
  privileged: true

# CRITICAL — grants all Linux capabilities
securityContext:
  capabilities:
    add: ["ALL"]

# CRITICAL — host namespace sharing
spec:
  hostNetwork: true
  hostPID: true
  hostIPC: true
```

- `runAsRoot: true` or `runAsNonRoot` absent from both pod-level and container-level securityContext → HIGH.
- `allowPrivilegeEscalation` not set to `false` → HIGH; a child process can acquire more privileges than its parent.
- `capabilities.add: [SYS_ADMIN]` or `[NET_ADMIN]` → CRITICAL.
- `capabilities.add: [ALL]` → CRITICAL.
- `privileged: true` → CRITICAL.
- `hostNetwork: true`, `hostPID: true`, or `hostIPC: true` → CRITICAL for each.
- `readOnlyRootFilesystem` absent or set to `false` → MEDIUM; the container filesystem is writable, enabling in-place modification of binaries.
- Secrets (passwords, tokens, keys) rendered as plain-text data in a ConfigMap instead of a Secret resource → CRITICAL; any workload that can read ConfigMaps in the namespace can read the value.

---

### Step 5 — Resource governance audit

Check resource requests, limits, and workload scaling policy.

- `resources.requests` or `resources.limits` absent from any container spec → HIGH; without limits, a misbehaving pod triggers node over-subscription and may cause OOM kills on neighbouring workloads.
- No `PodDisruptionBudget` for a StatefulSet or singleton Deployment → MEDIUM; node drains can take the workload to zero replicas.
- No `HorizontalPodAutoscaler` where the workload is expected to handle variable load → LOW.

---

### Step 6 — Health and observability audit

Check probe configuration.

- `livenessProbe` absent from any container → HIGH; the kubelet cannot detect a hung container, and a failed pod can receive live traffic indefinitely.
- `readinessProbe` absent → HIGH; rolling updates proceed without confirming the new pod is ready to serve traffic.
- `startupProbe` absent for containers with slow or variable startup times → MEDIUM; the liveness probe fires before the container is ready, causing crash loops.
- Probe `timeoutSeconds`, `failureThreshold`, or `periodSeconds` at Kubernetes defaults with no documented rationale → LOW; defaults may be too aggressive or too lenient for the workload.

---

### Step 7 — Testability audit

Check helm test coverage and chart-testing CI integration.

- No `tests/` directory → MEDIUM; helm test integration is absent. The chart has no post-install verification that can be run by `helm test`.
- `tests/` present but test manifests only assert pod existence (`kubectl get pod`) and do not verify service reachability or a functional endpoint → LOW; existence confirms the pod started, not that the service responds correctly.
- No CI integration for chart-testing — no `ct lint-and-install`, no `helm lint` + `helm template` step in CI configuration → MEDIUM; the chart is not regression-tested on install across a range of values.

---

### Step 8 — RBAC and service account audit

Check role scope and service account token exposure.

```yaml
# CRITICAL — ClusterRoleBinding to default SA gives cluster-wide access
# to every workload in the namespace
subjects:
  - kind: ServiceAccount
    name: default
    namespace: my-app
roleRef:
  kind: ClusterRole
  name: admin
```

- `ClusterRoleBinding` to the `default` service account → CRITICAL.
- `ClusterRole` used where a `Role` scoped to a single namespace would suffice → HIGH; blast radius of a compromised workload is the entire cluster.
- `serviceAccount.automountServiceAccountToken` not set to `false` when the workload makes no Kubernetes API calls → HIGH; the service account token is mounted into the pod and exploitable by any process that can read the filesystem.

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence: chart passes review with no critical issues / chart has critical defects that must be fixed before deployment / chart has high-severity defects requiring attention>

## Evidence level
<chart source provided | values only | partial (no templates) | inference>

## Findings

### CRITICAL
- [C1] <finding title>: <description> — <remediation>

### HIGH
- [H1] <finding title>: <description> — <remediation>

### MEDIUM
- [M1] <finding title>: <description> — <remediation>

### LOW
- [L1] <finding title>: <description> — <remediation>

## Safe next actions
1. <action>
2. <action>

## Open questions
- <question requiring user clarification>
```

---

## Security notes

- Never request kubeconfig, cluster credentials, cloud provider tokens, or live values files containing secrets. Ask for sanitized versions with placeholder values.
- Static review only — never install a chart, never run `helm upgrade` or `kubectl apply`, never contact a Kubernetes cluster.
- A container running as root, with `privileged: true`, or with `hostNetwork: true` is the highest-impact template security finding. Lead with it.
- A `ClusterRoleBinding` to the `default` service account grants cluster-wide access to every workload in the namespace. Treat it as CRITICAL and flag immediately.
- Secrets in ConfigMap instead of a Secret resource are exposed to all workloads in the namespace that have read access to ConfigMaps. Flag as CRITICAL.
- Do not recommend workarounds that maintain the defect (e.g. "add a comment explaining why privileged is needed" is not remediation for `privileged: true`).
