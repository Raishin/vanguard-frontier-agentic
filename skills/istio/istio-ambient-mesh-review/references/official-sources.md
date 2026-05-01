# Official Sources

Load these only when needed:

- [Istio documentation home](https://istio.io/latest/docs/) — use as the entry point for any Istio question.
- [Choosing between sidecar and ambient](https://istio.io/latest/docs/overview/dataplane-modes/) — use when deciding mesh mode or auditing a mixed-mode mesh.
- [Ambient mode overview](https://istio.io/latest/docs/ambient/overview/) — use for the layered architecture (ztunnel L4 + optional waypoint L7), HBONE protocol, and zero-trust posture.
- [L4 Authorization Policy in ambient](https://istio.io/latest/docs/ambient/usage/l4-policy/) — use for the L4-only fields ztunnel enforces and the default-ALLOW behavior.
- [Waypoint configuration](https://istio.io/latest/docs/ambient/usage/waypoint/) — use for waypoint deployment, binding via `istio.io/use-waypoint`, and the L7 features that require it.
- [PeerAuthentication API reference](https://istio.io/latest/docs/reference/config/security/peer_authentication/) — use for `STRICT` / `PERMISSIVE` / `DISABLE` semantics, mesh-wide vs namespace-scoped vs workload-scoped placement, and port-level overrides.
- [AuthorizationPolicy API reference](https://istio.io/latest/docs/reference/config/security/authorization-policy/) — use for `ALLOW` / `DENY` / `AUDIT` / `CUSTOM` actions, evaluation order, source/destination matchers, and L4 vs L7 field semantics.
- [RequestAuthentication API reference](https://istio.io/latest/docs/reference/config/security/request_authentication/) — use for JWT validation, `jwksUri`, `issuer`, `audiences`, `forwardOriginalToken`.
- [Gateway API reference](https://istio.io/latest/docs/reference/config/networking/gateway/) — use for ingress/egress gateway TLS modes, port configuration, and `credentialName` SDS pattern.
- [VirtualService API reference](https://istio.io/latest/docs/reference/config/networking/virtual-service/) — use for `match`, `rewrite`, `redirect`, `route` weighting, fault injection, retry, timeout.
- [DestinationRule API reference](https://istio.io/latest/docs/reference/config/networking/destination-rule/) — use for client-side mTLS, load balancing, connection pool, outlier detection, subset definitions.
- [Sidecar API reference](https://istio.io/latest/docs/reference/config/networking/sidecar/) — use for narrowing sidecar `egress.hosts` and reducing config-distribution overhead.
- [Multi-cluster setup guides](https://istio.io/latest/docs/setup/install/multicluster/) — use when the mesh spans clusters (multi-primary, primary-remote, multi-network).
- [istioctl reference](https://istio.io/latest/docs/reference/commands/istioctl/) — use for `istioctl analyze`, `istioctl x ztunnel-config`, `istioctl proxy-config`, `istioctl authz check`.
- [Istio Releases](https://istio.io/latest/news/releases/) — use when version-specific features matter (ambient GA in 1.24, waypoint API stabilization, etc.).

## Grounded insights worth carrying into the skill

- Ambient mode is a **layered architecture**: ztunnel handles L4 zero-trust for every pod in the mesh by default, and waypoint proxies are added only for the workloads that need L7 features (HTTP method/path matching, JWT claim authorization, request header inspection, traffic management).
- **An L7 `AuthorizationPolicy` rule on an ambient namespace with no waypoint is silently ignored.** The API server accepts the policy, but ztunnel only enforces L4 fields. This is the most-cited operational trap in ambient mode.
- The default action when no `AuthorizationPolicy` exists is **ALLOW**. Zero-trust posture requires explicit `DENY` policies or narrow `ALLOW` policies that collectively leave nothing reachable. `DENY` is evaluated before `ALLOW`.
- `PeerAuthentication` mTLS modes inherit from mesh → namespace → workload, with the most-specific policy winning. A mesh-wide `STRICT` policy can be locally weakened by a workload-scoped `DISABLE` policy on a specific port.
- Ambient mode requires **no pod restart** to add a workload to the mesh — labeling the namespace `istio.io/dataplane-mode=ambient` is sufficient. This is operationally simpler than sidecar injection but means changes can propagate faster than reviewers expect.
- The mesh root namespace (default `istio-system`, configurable via `meshConfig.rootNamespace`) is the only place where mesh-wide `PeerAuthentication` and `AuthorizationPolicy` can be authored. Anything there has cluster-wide blast radius.
- Waypoint placement uses Gateway API resources (`gateways.gateway.networking.k8s.io`) labeled `istio.io/waypoint-for`. A namespace-level waypoint protects all workloads in the namespace; a ServiceAccount-level waypoint protects all workloads using that SA; a workload-level waypoint binds via `istio.io/use-waypoint` annotation.
- ztunnel uses the **HBONE protocol** (HTTP/2 over mTLS, port 15008) for ztunnel-to-ztunnel communication. Network policy that blocks 15008 between nodes will break ambient mesh traffic.
- Sidecar mode and ambient mode workloads can communicate within one mesh — Istio bridges between them transparently. Ambient pods see sidecar pod connections as mTLS-authenticated peers.
- `istioctl analyze` runs the same checks Istiod runs at startup and is the safest pre-apply validator. CI pipelines should run it on every Istio config change.
