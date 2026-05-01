# Evidence Path and Tooling

## Evidence path

1. Prefer live cluster evidence when a Kubernetes MCP server, `kubectl`, the `cilium` CLI, and Hubble are available against the cluster.
2. Fall back to the official Cilium documentation (docs.cilium.io) for policy syntax, CRD schema, and ClusterMesh semantics when live inspection is unavailable.
3. Ask only for sanitized policy YAML, `cilium policy get` output, Hubble flow snippets, or ClusterMesh status output when current-state proof matters.
4. Label conclusions as `live evidence`, `documentation-based`, `sanitized user evidence`, or `inference`.

## Useful live-evidence commands

```shell
# All policy formats across the cluster
kubectl get networkpolicies,ciliumnetworkpolicies,ciliumclusterwidenetworkpolicies -A -o yaml

# Egress gateway policies
kubectl get ciliumegressgatewaypolicies -A -o yaml

# Cilium agent state and policy enforcement
kubectl -n kube-system get pods -l k8s-app=cilium -o name
kubectl -n kube-system exec -it <cilium-pod> -- cilium status
kubectl -n kube-system exec -it <cilium-pod> -- cilium policy get
kubectl -n kube-system exec -it <cilium-pod> -- cilium endpoint list

# Hubble flow observation (live traffic vs policy)
hubble observe --from-namespace <ns> --to-namespace <ns> --verdict DROPPED
hubble observe --to-fqdn <fqdn> --verdict DROPPED --last 1000

# ClusterMesh state
cilium clustermesh status
cilium clustermesh inspect-policy-default-local-cluster -A -o json

# Policy verification — what does Cilium think this pod is allowed to do?
kubectl -n kube-system exec -it <cilium-pod> -- \
  cilium policy trace --src-k8s-pod <ns>/<src-pod> --dst-k8s-pod <ns>/<dst-pod>
```

## Cilium state to confirm before review

- Cilium version (`kubectl -n kube-system exec <cilium-pod> -- cilium version`) — L7 policy support, ClusterMesh features, and CRD versions evolve across releases.
- Envoy proxy enabled — required for L7 policy fields (`toPorts.rules.http`, `toPorts.rules.kafka`, `toPorts.rules.dns`).
- ClusterMesh enabled (`cilium clustermesh status`) — multi-cluster policies are evaluated differently when ClusterMesh is up.
- `policy-default-local-cluster` setting (per cluster, configurable via Helm) — changes whether policies match cross-cluster identities by default.
- IPAM mode (`cluster-pool`, `kubernetes`, `eni`, `azure`, `aws-eni`) — affects the IP pool and any egress gateway IP planning.
- Hubble enabled — required for flow observability and policy debugging.
- Tetragon installed (separate but Cilium-affiliated) — runtime security; relevant when reviewing combined eBPF posture.

## Sanitization rules

- Never request kubeconfig contents, ClusterMesh peer Secrets, or Cilium agent tokens.
- Replace identifiable cluster IDs, peer cluster URLs, public egress IPs (when sensitive), and namespace names with placeholders unless the user provides them.
- Do not print Cilium agent service account tokens.
