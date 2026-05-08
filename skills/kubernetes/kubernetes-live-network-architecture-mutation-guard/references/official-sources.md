# Official sources

Authoritative upstream documentation for the surfaces this guard mutates.

## RBAC and authorization

- [RBAC Good Practices](https://kubernetes.io/docs/concepts/security/rbac-good-practices/) — the foundational guidance this guard implements: minimal privilege, no wildcards, no `cluster-admin` for routine work, no `system:masters` group membership.
- [Using RBAC Authorization](https://kubernetes.io/docs/reference/access-authn-authz/rbac/) — the API reference for `Role` / `ClusterRole` / `RoleBinding` / `ClusterRoleBinding`.
- [`kubectl auth can-i`](https://kubernetes.io/docs/reference/kubectl/generated/kubectl_auth/kubectl_auth_can-i/) — the pre-flight matrix syntax.
- [User Impersonation](https://kubernetes.io/docs/reference/access-authn-authz/authentication/#user-impersonation) — the recommended pattern: low-privileged operator with `impersonate` rights on the agent's ServiceAccount.

## Service traffic policy and topology-aware routing

- [Service Internal Traffic Policy](https://kubernetes.io/docs/concepts/services-networking/service-traffic-policy/) — `internalTrafficPolicy` and `externalTrafficPolicy` semantics.
- [Topology Aware Routing](https://kubernetes.io/docs/concepts/services-networking/topology-aware-routing/) — `service.kubernetes.io/topology-mode: Auto` annotation and `spec.trafficDistribution` field.

## CoreDNS

- [CoreDNS `kubernetes` plugin](https://coredns.io/plugins/kubernetes/) — the in-cluster DNS plugin reference.
- [CoreDNS `reload` plugin](https://coredns.io/plugins/reload/) — Corefile change detection (default 30s poll).
- [CoreDNS `loop` plugin](https://coredns.io/plugins/loop/) — recursion-loop detection (silent failure mode if absent).
- [CoreDNS `health` plugin](https://coredns.io/plugins/health/) — liveness probe target.

## NodeLocal DNSCache

- [Using NodeLocal DNSCache in Kubernetes Clusters](https://kubernetes.io/docs/tasks/administer-cluster/nodelocaldns/) — install manifest, the `__PILLAR__` token substitutions, and the kube-proxy / Cilium kube-proxy replacement preconditions.

## Gateway API

- [Gateway API](https://gateway-api.sigs.k8s.io/) — the SIG site (authoritative).
- [`Gateway` resource](https://gateway-api.sigs.k8s.io/api-types/gateway/) — the listener model and `allowedRoutes` semantics.
- [`HTTPRoute`](https://gateway-api.sigs.k8s.io/api-types/httproute/) — HTTP route reference.
- [`GRPCRoute`](https://gateway-api.sigs.k8s.io/api-types/grpcroute/) — gRPC route reference. **GA / Standard channel since Gateway API v1.1.0.**
- [`ReferenceGrant`](https://gateway-api.sigs.k8s.io/api-types/referencegrant/) — cross-namespace reference authorization.
- [Gateway API Versioning](https://gateway-api.sigs.k8s.io/concepts/versioning/) — Standard vs Experimental channel semantics.

## Cilium ClusterMesh

- [Setting up Cluster Mesh](https://docs.cilium.io/en/stable/network/clustermesh/clustermesh/) — peer Secret structure, namespace, and CA trust setup.
- [ClusterMesh Troubleshooting](https://docs.cilium.io/en/stable/operations/troubleshooting/) — `cilium clustermesh status`, `cilium-dbg troubleshoot clustermesh`, and the KVStoreMesh `kvstoremesh-dbg troubleshoot` command.
- [KVStoreMesh command reference](https://docs.cilium.io/en/stable/cmdref/clustermesh-apiserver_kvstoremesh/) — `--clustermesh-cache-ttl` (default `0s` means "never revoked"), `--global-ready-timeout` (default 10m).
