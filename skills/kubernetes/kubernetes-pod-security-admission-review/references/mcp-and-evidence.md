# Evidence Path and Tooling

## Evidence path

1. Prefer live cluster evidence (`kubectl get namespaces --show-labels`, `kubectl get pods -n <ns> -o yaml`, and the cluster's `AdmissionConfiguration` if accessible).
2. Fall back to the official Kubernetes documentation: Pod Security Admission, Pod Security Standards, and namespace-label enforcement guide.
3. Ask only for sanitized namespace YAML, sanitized pod spec excerpts (focus on `securityContext`, `volumes`, `hostNetwork`, `hostPID`, `hostIPC`), and the cluster's PSA admission configuration when current-state proof matters.
4. Label conclusions as `live evidence`, `documentation-based`, `sanitized user evidence`, or `inference`.

## Useful live-evidence commands

```shell
# View PSA labels on every namespace
kubectl get namespaces --show-labels | grep -E 'pod-security|^NAME'

# Detailed namespace labels and annotations
kubectl get namespace <ns> -o yaml | grep -A20 metadata

# Check whether running pods would still admit at a stricter profile
# (use kubectl-pod-security plugin or apply dry-run with the new label)
kubectl label namespace <ns> pod-security.kubernetes.io/warn=restricted --overwrite --dry-run=server

# Audit-mode violations end up in the audit log (cluster-admin access required)
# Look for pod-security violations in apiserver audit log

# Cluster default PSA configuration (if user has access to control plane)
kubectl -n kube-system get pod -l component=kube-apiserver -o yaml | grep -A20 admission

# Pod security context inspection
kubectl get pod -n <ns> <pod> -o jsonpath='{.spec.securityContext}'
kubectl get pod -n <ns> <pod> -o jsonpath='{.spec.containers[*].securityContext}'

# List pods that would fail restricted profile
kubectl get pods -A -o jsonpath='{range .items[?(@.spec.containers[*].securityContext.privileged==true)]}{.metadata.namespace}/{.metadata.name}{"\n"}{end}'
```

## Cluster state to confirm before review

- **Kubernetes version** (`kubectl version`) — PSA stable in 1.25; profile semantics evolve; pin `enforce-version` to a specific minor.
- **Cluster default profile** (cluster's `AdmissionConfiguration`) — when a namespace has no label, this is what applies.
- **Cluster exemptions** — the `AdmissionConfiguration` can exempt usernames, runtime classes, and namespaces by name (different from per-namespace label override).
- **Whether PSP (PodSecurityPolicy) admission is still active** — PSP was removed in 1.25 but some clusters run a PSP-equivalent webhook. Migration tools include `kubectl-psp-to-psa`.
- **Whether other admission policies (Kyverno, OPA Gatekeeper) layer on top** — PSA is the floor; other engines can be stricter but must not weaken it.

## Sanitization rules

- Never request kubeconfig contents or apiserver audit log access.
- Replace identifiable namespace names and pod names with placeholders unless the user provides them.
- Do not print pod environment variables, init container args, or volume secret content.
