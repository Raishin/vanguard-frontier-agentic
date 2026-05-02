# Workflow and Output Contract

## Review Workflow

### Step 1 — Identify the workload type

Determine whether the input is a Pod, Deployment, StatefulSet, DaemonSet, Job, or CronJob spec. The review scope differs:

- **Pod / Deployment** — full probe, resource, securityContext, topology spread review
- **StatefulSet** — same as Deployment plus PVC template review, ordered startup considerations
- **DaemonSet** — probe review less critical; focus on host namespace usage, privileged mode, resource limits
- **Job / CronJob** — no readiness probe required; focus on `activeDeadlineSeconds`, `backoffLimit`, resource limits

### Step 2 — Probe review

```yaml
# Minimum production-ready probe configuration
livenessProbe:
  httpGet:
    path: /healthz
    port: 8080
  initialDelaySeconds: 15
  periodSeconds: 20
  failureThreshold: 3       # >=3 to tolerate GC pauses
  timeoutSeconds: 5

readinessProbe:
  httpGet:
    path: /ready
    port: 8080
  initialDelaySeconds: 10
  periodSeconds: 10
  failureThreshold: 3
  timeoutSeconds: 3

startupProbe:               # required if startup > 30s
  httpGet:
    path: /healthz
    port: 8080
  failureThreshold: 30      # 30 * periodSeconds(10) = 300s max startup
  periodSeconds: 10
```

**Flags:**
- Missing `livenessProbe` on a long-running container — HIGH
- Missing `readinessProbe` on a Deployment that receives traffic — HIGH
- `livenessProbe.failureThreshold: 1` or `2` — HIGH (kills pod during GC pause)
- `readinessProbe.initialDelaySeconds` < known startup time — HIGH (probe fails before app ready)
- Missing `startupProbe` when app startup > 30s — MEDIUM

### Step 3 — Resource QoS review

```yaml
resources:
  requests:
    cpu: "250m"
    memory: "256Mi"
  limits:
    cpu: "500m"    # CPU limits cause throttling; consider removing if not required
    memory: "512Mi"
```

**QoS tier resolution:**

| Condition | QoS Class | Risk |
|-----------|-----------|------|
| `requests == limits` for all containers | Guaranteed | Lowest eviction priority |
| `requests` set, `limits` not equal | Burstable | Evicted under node pressure |
| No `requests`, no `limits` | BestEffort | First evicted under any pressure |

**Flags:**
- No `resources.requests` — MEDIUM (BestEffort QoS, evicted first)
- `limits.memory` without `requests.memory` — MEDIUM (Burstable, OOM killed under node pressure)
- `limits.cpu` set to a value significantly lower than typical usage — MEDIUM (CPU throttle)
- Critical workload without Guaranteed QoS (`requests != limits`) — MEDIUM

### Step 4 — securityContext review

```yaml
# Pod-level
spec:
  securityContext:
    runAsNonRoot: true
    runAsUser: 1000
    seccompProfile:
      type: RuntimeDefault   # K8s 1.22+

# Container-level
containers:
  - name: app
    securityContext:
      allowPrivilegeEscalation: false
      readOnlyRootFilesystem: true
      capabilities:
        drop:
          - ALL
        add: []              # only add specific caps if truly required
```

**Flags:**
- Missing `runAsNonRoot: true` — HIGH (runs as root by default)
- `allowPrivilegeEscalation: true` or missing — HIGH
- Missing `readOnlyRootFilesystem: true` — MEDIUM (writable filesystem enables malware persistence)
- Missing `capabilities.drop: [ALL]` — MEDIUM
- `privileged: true` — CRITICAL (host-level access)
- Missing `seccompProfile` — LOW (defaults to unconfined syscall access)

### Step 5 — Image pull policy and tag review

```yaml
# Correct for digest-pinned images
image: myregistry/myapp@sha256:abc123...
imagePullPolicy: IfNotPresent

# Correct for latest or mutable tags
image: myregistry/myapp:latest
imagePullPolicy: Always
```

**Flags:**
- `latest` tag with `imagePullPolicy: IfNotPresent` — HIGH (stale image after first pull)
- `latest` tag at all — MEDIUM (non-deterministic deployments)
- No image digest pinning for critical workloads — LOW

### Step 6 — Secret and ConfigMap consumption review

```yaml
# PREFERRED: Volume mount (secret not in env, not in describe output)
volumes:
  - name: db-creds
    secret:
      secretName: db-credentials
containers:
  - volumeMounts:
      - name: db-creds
        mountPath: /etc/secrets
        readOnly: true

# ACCEPTABLE: Specific env var from secret key
env:
  - name: DB_PASSWORD
    valueFrom:
      secretKeyRef:
        name: db-credentials
        key: password

# AVOID: Bulk-mount exposes ALL secret keys including unused ones
envFrom:
  - secretRef:
      name: db-credentials
```

**Flags:**
- `envFrom.secretRef` bulk-mount — MEDIUM (all keys exposed to process env and kubectl describe)
- `env.valueFrom.secretKeyRef` — ACCEPTABLE (only named key exposed)
- Secret as environment variable (either method) — NOTE (appears in /proc/self/environ)

### Step 7 — Topology spread and affinity review

```yaml
# Preferred: topology spread (K8s 1.19+)
topologySpreadConstraints:
  - maxSkew: 1
    topologyKey: topology.kubernetes.io/zone
    whenUnsatisfiable: DoNotSchedule
    labelSelector:
      matchLabels:
        app: myapp

# Also check podAntiAffinity for legacy configs
affinity:
  podAntiAffinity:
    preferredDuringSchedulingIgnoredDuringExecution:
      - weight: 100
        podAffinityTerm:
          topologyKey: kubernetes.io/hostname
          labelSelector:
            matchLabels:
              app: myapp
```

**Flags:**
- Multi-replica Deployment (>1 replica) with no `topologySpreadConstraints` and no `podAntiAffinity` — MEDIUM
- `topologySpreadConstraints` present but `topologyKey: kubernetes.io/hostname` only (no zone spread) — LOW
- `whenUnsatisfiable: ScheduleAnyway` on a critical workload — LOW (spread not enforced)

### Step 8 — Termination grace period review

```yaml
spec:
  terminationGracePeriodSeconds: 60  # increase for gRPC, database draining
```

**Flags:**
- Default 30s for gRPC servers with long-lived streams — MEDIUM
- Default 30s for database pods (PostgreSQL, MySQL) that need checkpoint time — MEDIUM
- `terminationGracePeriodSeconds: 0` — HIGH (immediate SIGKILL, no graceful shutdown)

---

## Output Format

Return findings in this structure:

### Finding: `<short title>`

| Field | Value |
|-------|-------|
| Severity | CRITICAL / HIGH / MEDIUM / LOW |
| Field path | `spec.containers[0].livenessProbe` |
| Evidence | documentation-based / live evidence / inference |
| Description | What is wrong and why it matters |
| Remediation | YAML snippet or command |

---

### Overall Verdict

| Category | Status |
|----------|--------|
| Probes | PASS / FAIL |
| Resource QoS | PASS / FAIL |
| Security context | PASS / FAIL |
| Image hygiene | PASS / FAIL |
| Secret consumption | PASS / FAIL |
| Topology spread | PASS / FAIL |
| Termination grace | PASS / FAIL |

**Production-ready:** YES / NO / CONDITIONAL (list conditions)
