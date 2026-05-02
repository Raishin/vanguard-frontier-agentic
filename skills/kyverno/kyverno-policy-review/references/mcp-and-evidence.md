# Evidence Path and Tooling

## Evidence path

1. Prefer live cluster evidence when a Kubernetes MCP server, `kubectl`, or `kyverno` CLI is available.
2. Fall back to official Kyverno documentation (kyverno.io) and the Kubernetes admission control reference when live inspection is unavailable.
3. Ask only for sanitized policy YAML, PolicyReport snippets, or `kyverno apply` output when current-state proof matters. Never request kubeconfig contents, admission webhook bearer tokens, image-signing private keys, or secrets.
4. Label conclusions as `live evidence`, `documentation-based`, `sanitized user evidence`, or `inference`.

## Useful live-evidence commands

```shell
# List all Kyverno policy kinds across the cluster (stable v1 API)
kubectl get validatingpolicies,mutatingpolicies,generatingpolicies,deletingpolicies,imagevalidatingpolicies -A -o yaml

# Legacy kinds (still in use on many clusters)
kubectl get clusterpolicies,policies -A -o yaml

# List all PolicyExceptions — every one is a documented bypass
kubectl get policyexceptions -A -o yaml

# View Kyverno controller deployment and webhook config
kubectl -n kyverno get deploy,svc,validatingwebhookconfiguration,mutatingwebhookconfiguration -o yaml

# View Kyverno admission reports — does the policy actually run?
kubectl get policyreport,clusterpolicyreport -A

# Test a policy locally without applying
kyverno apply policy.yaml --resource resource.yaml

# Test against the live cluster
kyverno apply policy.yaml --cluster

# Generate a native ValidatingAdmissionPolicy from a Kyverno policy (preview)
kyverno migrate-policy policy.yaml --output validatingadmissionpolicy.yaml
```

## Kyverno install state to confirm before review

- Kyverno controller version (`kubectl -n kyverno get deploy kyverno -o jsonpath='{.spec.template.spec.containers[0].image}'`) — newer versions support more CEL expressions and the stable `policies.kyverno.io/v1` API.
- Reports Server enabled (`kubectl -n kyverno get deploy reports-server`) — controls whether PolicyReports are stored externally or in etcd.
- Cleanup controller enabled — required for `DeletingPolicy` resources.
- Admission controller webhook timeout — Kyverno's default is 10s; aggressive policies can stall pod creation.

## Platform-agnostic execution

- Keep examples neutral with placeholders (`<policy-name>`, `<namespace>`, `<image-ref>`) until the user's cluster context and policy state are known.
- Do not request kubeconfig files, image signing keys, Sigstore Rekor entries, or registry credentials in chat.
- If a Kubernetes MCP server, `kubectl`, or `kyverno` CLI is unavailable, say so and fall back to reviewing sanitized YAML and the official Kyverno documentation.
