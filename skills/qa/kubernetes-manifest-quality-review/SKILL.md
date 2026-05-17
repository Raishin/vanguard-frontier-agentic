---
name: kubernetes-manifest-quality-review
description: Use this skill when the user provides raw Kubernetes YAML manifests or asks to review K8s manifests for quality, security, or policy compliance — covering Deployment, StatefulSet, DaemonSet, Service, Ingress, NetworkPolicy, RBAC, and CRD resources.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-17"
  category: delivery
  lifecycle: experimental
---

# Kubernetes Manifest Quality Review

## Purpose

This skill reviews raw Kubernetes YAML manifests for quality, security, and policy-compliance defects. It covers Deployment, StatefulSet, DaemonSet, Service, Ingress, NetworkPolicy, RBAC, and CRD resources. The review is entirely static — it reads YAML files and never applies manifests to a cluster, never contacts the Kubernetes API, and never requests kubeconfig, service account tokens, or cloud credentials.

## Lean operating rules

### Schema and structure

- `apiVersion` or `kind` missing — CRITICAL: the manifest cannot be applied; flag and stop review of that resource.
- Deprecated API versions (e.g., `extensions/v1beta1`, `networking.k8s.io/v1beta1`, `policy/v1beta1` PodSecurityPolicy) — HIGH: these will be rejected by newer clusters.
- Missing required labels (`app`, `app.kubernetes.io/name`, `app.kubernetes.io/version`) on Pods and workload controllers — MEDIUM: impairs observability, selector targeting, and policy enforcement.
- No `namespace` specified (reliance on default namespace) — MEDIUM: encourages lateral movement and policy bypass; everything should be explicitly namespaced.

### Pod security (Pod Security Standards)

- `securityContext.runAsRoot: true` on a container, or no `runAsNonRoot: true` at pod or container level — HIGH: processes run as UID 0 inside the container.
- `privileged: true` on a container security context — CRITICAL: the container has near-host-root access.
- `allowPrivilegeEscalation: true` or field absent (it defaults to `true` unless `privileged: false` is set) — HIGH: child processes can gain more privileges than the parent.
- `hostNetwork: true`, `hostPID: true`, `hostIPC: true` on the pod spec — CRITICAL: the pod shares the host network stack, process table, or IPC namespace, enabling broad host compromise.
- `capabilities.add` containing `SYS_ADMIN`, `NET_ADMIN`, `ALL`, `SYS_PTRACE`, or `DAC_OVERRIDE` — CRITICAL: these capabilities provide near-root privilege; drop all capabilities and add only what is specifically required.
- `readOnlyRootFilesystem: false` or field absent on a container — MEDIUM: a writable root filesystem makes container compromise easier; set to `true` and use `emptyDir` or volume mounts for mutable paths.
- `seccompProfile` absent at pod or container level — MEDIUM: no syscall filtering, increasing the kernel attack surface; use `RuntimeDefault` or a custom profile.

### Image hygiene

- Image tag is `:latest` or absent — HIGH: non-reproducible deployments; a rollout can silently pull a different image than what was tested.
- No image digest pinning for production manifests — MEDIUM: tag mutability allows supply-chain substitution; prefer `image@sha256:<digest>`.
- Image pulled from an unverified public registry (e.g., Docker Hub) with no `imagePullPolicy: IfNotPresent` or digest — MEDIUM: arbitrary public images without integrity verification.

### Resource governance

- `resources.requests` and `resources.limits` both absent on a container — HIGH: the container is unschedulable on resource-constrained nodes and can starve co-located workloads.
- Memory limit set without a CPU limit — MEDIUM: CPU throttling surprise; the container can be throttled heavily with no visible error.
- Ephemeral storage limit absent on containers known to produce logs or temp files — LOW: unbounded ephemeral storage can exhaust node disk and evict other pods.

### Health probes

- `livenessProbe` missing — HIGH: the kubelet cannot detect application deadlocks or crash-loop conditions and restart the container.
- `readinessProbe` missing — HIGH: the endpoint controller sends traffic to the pod before the application is ready, causing errors during startup and rolling updates.
- Probe using `exec` command with no `timeoutSeconds` specified — MEDIUM: exec probes default to a 1-second timeout; a slow command silently causes probe failures and restarts.

### Networking and exposure

- Service type `LoadBalancer` or `NodePort` without a comment or annotation documenting the business justification — MEDIUM: these expose services externally or on every node port; ClusterIP is sufficient for internal services.
- Ingress resource with no TLS block configured — HIGH: traffic between the client and the ingress controller is unencrypted.
- No `NetworkPolicy` resource restricts pod ingress or egress in the namespace — MEDIUM: the default Kubernetes network model is allow-all; without a NetworkPolicy every pod can reach every other pod.
- Ingress annotation `nginx.ingress.kubernetes.io/use-proxy-protocol` or similar annotation that forwards arbitrary upstream headers into backend requests from untrusted input — CRITICAL: enables SSRF and header injection.

### RBAC and service accounts

- `ClusterRole` with verb `*` on resource `*` or on `secrets` — CRITICAL: any principal bound to this role has full cluster read/write access.
- `RoleBinding` or `ClusterRoleBinding` whose subject is `system:anonymous` or `system:unauthenticated` — CRITICAL: unauthenticated callers inherit these permissions.
- `automountServiceAccountToken: true` (or field absent, which defaults to `true`) on pods that do not contact the Kubernetes API — HIGH: the token is mounted at a known path and exploitable if the container is compromised.
- RBAC role granting `get` or `list` on `secrets` beyond what the workload demonstrably needs — HIGH: broadens blast radius of a credential compromise.

### Secrets and config

- Plaintext credentials (passwords, tokens, connection strings) in `env.value` on a container or in `ConfigMap.data` — CRITICAL: credentials visible in manifests committed to source control or stored in etcd in plaintext.
- `Secret` with `type: Opaque` and a base64-encoded value that decodes to an empty string — MEDIUM: placeholder secret that will cause application startup failures and suggests secrets management is not wired up.

## References

Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum

Return, at minimum:
- Schema and API version findings
- Pod security findings (PSS Restricted/Baseline comparison)
- Image hygiene findings
- Resource governance findings
- Health probe findings
- Networking and exposure findings
- RBAC and service account findings
- Secrets and config findings
- Severity-labelled finding list (CRITICAL / HIGH / MEDIUM / LOW)
- Safe next actions
