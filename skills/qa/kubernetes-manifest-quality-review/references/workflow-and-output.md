# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized files (no real secret values, no kubeconfig, no service account tokens, no cloud credentials — replace sensitive values with placeholders):
- Workload manifests: Deployment, StatefulSet, DaemonSet YAML
- Service and Ingress YAML
- NetworkPolicy YAML
- RBAC resources: Role, ClusterRole, RoleBinding, ClusterRoleBinding YAML
- CRD definitions if relevant
- Any Kustomize base and overlay files

If NetworkPolicy resources are not provided, the egress/ingress audit findings are stated as `inference` — say so and ask for them.

### Step 2 — Schema and API version audit

Validate that every manifest has `apiVersion` and `kind` present. Check for deprecated or removed API versions:

```yaml
# HIGH — removed in Kubernetes 1.22
apiVersion: extensions/v1beta1
kind: Ingress

# HIGH — networking.k8s.io/v1beta1 Ingress removed in 1.22
apiVersion: networking.k8s.io/v1beta1
kind: Ingress

# HIGH — policy/v1beta1 PodSecurityPolicy removed in 1.25
apiVersion: policy/v1beta1
kind: PodSecurityPolicy
```

Check that required labels are present on Pod templates and workload controllers: `app`, `app.kubernetes.io/name`, `app.kubernetes.io/version`. Flag missing `namespace` on all resources.

### Step 3 — Pod security audit (PSS Restricted/Baseline comparison)

Evaluate each Pod spec against the Pod Security Standards Restricted profile:

```yaml
# CRITICAL — privileged container
securityContext:
  privileged: true

# CRITICAL — host namespaces
hostNetwork: true
hostPID: true
hostIPC: true

# HIGH — runAsRoot or missing runAsNonRoot
securityContext:
  runAsUser: 0
  # or: runAsNonRoot absent

# HIGH — allowPrivilegeEscalation unset or true
securityContext:
  allowPrivilegeEscalation: true

# CRITICAL — dangerous capabilities
securityContext:
  capabilities:
    add: ["SYS_ADMIN"]

# MEDIUM — writable root filesystem
securityContext:
  readOnlyRootFilesystem: false
  # or: field absent

# MEDIUM — no seccomp profile
securityContext:
  # seccompProfile absent
```

For each container in the pod, note whether the field is set at the pod level, the container level, or both. Container-level settings override pod-level settings.

### Step 4 — Image hygiene audit

Check every container and init container image reference:

```yaml
# HIGH — mutable tag, non-reproducible
image: nginx:latest
image: myapp   # tag absent

# MEDIUM — no digest pinning
image: nginx:1.25.3   # tag present but no @sha256 digest

# MEDIUM — unverified public registry, no digest
image: docker.io/library/nginx:1.25.3
```

For production-grade manifests, recommend digest-pinned images:
```yaml
image: nginx:1.25.3@sha256:<digest>
```

### Step 5 — Resource governance audit

Check every container for `resources.requests` and `resources.limits`:

```yaml
# HIGH — no requests or limits
containers:
  - name: app
    image: myapp:1.0.0
    # resources absent

# MEDIUM — memory limit set without CPU limit
resources:
  limits:
    memory: 512Mi
  requests:
    cpu: 100m
    memory: 256Mi
  # limits.cpu absent
```

Check for ephemeral storage limits on containers known to produce log output or temporary files.

### Step 6 — Health probe audit

Check every container for `livenessProbe` and `readinessProbe`:

```yaml
# HIGH — missing livenessProbe
containers:
  - name: app
    # livenessProbe absent

# HIGH — missing readinessProbe
containers:
  - name: app
    # readinessProbe absent

# MEDIUM — exec probe with no timeoutSeconds
livenessProbe:
  exec:
    command: ["/bin/check"]
  # timeoutSeconds absent, defaults to 1 second
```

### Step 7 — Networking and exposure audit

Review Service types, Ingress TLS, NetworkPolicy coverage, and Ingress annotations:

```yaml
# MEDIUM — external exposure without documented justification
kind: Service
spec:
  type: LoadBalancer   # or NodePort

# HIGH — Ingress without TLS
kind: Ingress
spec:
  # tls block absent

# MEDIUM — no NetworkPolicy found in namespace (default allow-all)

# CRITICAL — SSRF-enabling Ingress annotation
metadata:
  annotations:
    nginx.ingress.kubernetes.io/use-proxy-protocol: "true"
```

If no NetworkPolicy resources are provided for the namespace, state that the default-allow posture is inferred and ask for NetworkPolicy files.

### Step 8 — RBAC and secrets audit

Review ClusterRole, Role, RoleBinding, ClusterRoleBinding, and Secret resources:

```yaml
# CRITICAL — wildcard verbs on wildcard resources
rules:
  - apiGroups: ["*"]
    resources: ["*"]
    verbs: ["*"]

# CRITICAL — unauthenticated subject
subjects:
  - kind: Group
    name: system:unauthenticated

# HIGH — automount enabled on pods that do not need API access
automountServiceAccountToken: true   # or field absent

# HIGH — broad secret access
rules:
  - resources: ["secrets"]
    verbs: ["get", "list"]

# CRITICAL — plaintext credentials in env
env:
  - name: DB_PASSWORD
    value: "mysecretpassword"

# MEDIUM — empty-string secret value
data:
  password: ""   # decodes to empty
```

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence: manifests pass baseline / manifests have blocking security defects / manifests need remediation before production>

## Evidence level
<manifest files provided | partial manifests only | inference for missing resources>

## Findings

### CRITICAL
- [C1] <resource name> — <finding>: <description> — <remediation>

### HIGH
- [H1] <resource name> — <finding>: <description> — <remediation>

### MEDIUM
- [M1] <resource name> — <finding>: <description> — <remediation>

### LOW
- [L1] <resource name> — <finding>: <description> — <remediation>

## Safe next actions
1. <action>
2. <action>

## Open questions
- <question requiring user clarification>
```

---

## Security notes

- Never request or accept kubeconfig, service account tokens, cloud credentials, or actual secret values. Ask for sanitized manifests with placeholder values in Secret resources.
- This is a static review: do not apply manifests, run `kubectl`, or contact any cluster.
- A `privileged: true` container, `hostNetwork/hostPID/hostIPC: true`, or a ClusterRole with `*` verbs on `*` resources is the highest-impact finding class. Lead with it.
- `RoleBinding` to `system:unauthenticated` or `system:anonymous` is a critical exposure; tell the user to remove it immediately.
- Plaintext credentials in `env.value` or `ConfigMap.data` should be replaced with `secretKeyRef` references; never recommend committing real credentials even in base64.
- Do not recommend disabling probes or relaxing securityContext fields to pass short-term validation — recommend the correct secure configuration and explain the rationale.
