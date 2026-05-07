# Service and Gateway API Routing

## Step 1 — Identify the routing surface

Capture:

- `kubectl get services -A -o wide` — types, ClusterIPs, externalIPs, ports.
- `kubectl get endpointslices -A` — confirm endpoints actually exist for every ClusterIP. A Service with zero EndpointSlices selects no pods.
- `kubectl get ingress -A` and `kubectl get gateway,gatewayclass,httproute,grpcroute,tlsroute,referencegrant -A` — Ingress and Gateway API surface.
- IngressClass and GatewayClass ownership — `kubectl get ingressclass`, `kubectl get gatewayclass` — which controllers actually exist on this cluster.

## Step 2 — Stress-test Service type selection

| Type | When right | When wrong |
|---|---|---|
| `ClusterIP` (default) | In-cluster only. | Used as a "fix" for east-west reachability when the real issue is NetworkPolicy or DNS. |
| `NodePort` | Bare-metal or dev clusters where no cloud LB exists. | Used in cloud — exposes every node as an entry point, scraping the NodePort range from outside. Rarely the right cloud answer. |
| `LoadBalancer` | Public/private endpoint via the cloud LB controller (or MetalLB / Cilium L2/BGP). | One LB per Service in classic mode is expensive at scale. Consolidate via Ingress/Gateway when feasible. |
| `ExternalName` | DNS CNAME into an external host. | Used as policy-bypass — the pod still does its own DNS resolution; NetworkPolicy still applies to the resolved IP. |
| Headless (`clusterIP: None`) | StatefulSet stable DNS, gRPC client-side LB, custom service discovery. | Used because someone wanted "no kube-proxy" — but then there is no load balancing, only DNS round-robin against EndpointSlices. |

## Step 3 — Stress-test EndpointSlices

EndpointSlices replace the legacy single-Endpoints object and are the data the Service VIP rules / eBPF maps actually consume.

- A Service with `selector` populated but zero matching pods has an empty EndpointSlice — the VIP exists but every connection RSTs or hangs depending on kube-proxy mode. Common cause: Pod label drift after a Helm upgrade.
- A Service without a `selector` (used for static external endpoints) requires a hand-maintained EndpointSlice; without it nothing is routed.
- EndpointSlices report `Ready`, `Serving`, `Terminating` conditions. `externalTrafficPolicy: Local` plus rolling pod restart can transiently hit a node where the only local endpoint is `Terminating`. Verify ProxyTerminatingEndpoints behavior in the cluster's kube-proxy version.
- Topology hints live in EndpointSlices (`hints.forZones`). They are populated by the EndpointSlice controller only when topology-aware routing prerequisites are met (multiple zones, sufficient endpoints per zone).

## Step 4 — `internalTrafficPolicy` and `externalTrafficPolicy`

These two fields are the single most-misunderstood part of Kubernetes networking.

- **`externalTrafficPolicy: Cluster`** (default) — kube-proxy routes external traffic to any pod cluster-wide. Source IP is rewritten to the receiving node. Even load distribution.
- **`externalTrafficPolicy: Local`** — kube-proxy only routes external traffic to pods on the same node as the receiving NodePort/LoadBalancer ingress point. Preserves source IP. **Black-holes traffic if no local endpoint exists** — health-check ports rely on this for cloud LB to drop the node from rotation. If health-check is misconfigured, the LB sends to a node with no pod, and the connection hangs.
- **`internalTrafficPolicy: Cluster`** (default) — same as Cluster externally.
- **`internalTrafficPolicy: Local`** — in-cluster traffic from a pod is routed only to a pod on the same node. **Silent black-hole** if no local endpoint exists; there is no LB rotation to fall back to. Used for node-local agent → app patterns where co-location is guaranteed by DaemonSet.

Stress-tests the review must apply:

- A Service with `externalTrafficPolicy: Local` and a Deployment of `replicas: 2` on a 10-node cluster — 8 nodes black-hole the NodePort. The cloud LB health-check must mark them unhealthy or every 8/10 connections fails.
- `externalTrafficPolicy: Local` with `Local` health-check disabled on the LB — random failures.
- `internalTrafficPolicy: Local` on a Service backing a Deployment with no anti-affinity — silent black-holes when no replica lands on the caller's node.

## Step 5 — Topology-aware routing

`service.kubernetes.io/topology-mode: Auto` (replaces the deprecated `topologyKeys`) tells the EndpointSlice controller to populate `hints.forZones` so kube-proxy prefers same-zone endpoints, reducing cross-zone traffic cost.

Traps:

- Auto mode requires sufficient endpoints per zone — too few endpoints, hints are not populated, no zone preference. Behavior silently changes when the deployment scales up.
- A Service with topology hints behaves like `internalTrafficPolicy: Local` from a cost perspective but without the black-hole risk — same-zone preferred, fall back to cross-zone if local endpoints unhealthy.
- Topology hints do not respect `externalTrafficPolicy: Local` — they are independent fields.

## Step 6 — Ingress vs Gateway API

Gateway API is the successor to Ingress. The migration is per-route, not per-cluster.

What's GA today (per the [Gateway API site](https://gateway-api.sigs.k8s.io/) and the [versioning page](https://gateway-api.sigs.k8s.io/concepts/versioning/)):

- `Gateway`, `GatewayClass`, `HTTPRoute` — Standard channel, graduated to `v1` in the v1.0.0 release.
- `GRPCRoute` — **Standard channel since v1.1.0** (`v1`). Earlier sources that label it experimental refer to the v1.0 line.
- `ReferenceGrant` — Standard channel (`v1beta1`).
- GAMMA service-mesh integration (routes attach directly to a Service rather than a Gateway) — Standard channel since v1.1.0.

Experimental channel as of the v1.x line — verify against the controller's conformance for the version in use:

- `TLSRoute`, `TCPRoute`, `UDPRoute`, `BackendTLSPolicy`, `BackendTrafficPolicy`.

Always confirm by listing the CRDs the controller actually installed: `kubectl get crd -l gateway.networking.k8s.io/bundle-version`.

The role-oriented model:

- **Infrastructure provider** owns `GatewayClass`.
- **Cluster operator** owns `Gateway` instances (and the policies that bound them).
- **Application developer** owns `HTTPRoute` / `GRPCRoute` / `TLSRoute` and attaches them to a Gateway via `parentRefs`.

Migration stress-tests:

- Ingress Controller annotations (`nginx.ingress.kubernetes.io/...`, `cert-manager.io/cluster-issuer`, `external-dns.alpha.kubernetes.io/...`) are non-portable. Migrating an Ingress to an HTTPRoute requires translating each annotation into either a built-in HTTPRoute field, a `BackendTLSPolicy`, or a controller-specific extension.
- Cross-namespace references in HTTPRoute (`backendRefs` to a Service in a different namespace) require an explicit `ReferenceGrant` in the target namespace — without it, the reference is denied. This is a deliberate security posture; do not work around it with cluster-scoped routes.
- Two GatewayClasses on the same cluster (e.g. `cilium` and `nginx`) — routes must select via `parentRefs.name`/`namespace`. Avoid having multiple Gateways listen on the same hostname.
- TLS termination at the Gateway with `backendRefs` over plain HTTP — re-encryption to the backend requires `BackendTLSPolicy` (Experimental) or a service mesh; without it, in-cluster traffic is plaintext.
- GAMMA mesh — HTTPRoute attached to a Service (not a Gateway) reshapes east-west traffic. Available implementations differ on which fields they honor; verify against the controller's GAMMA conformance report.

## Step 7 — Session affinity, sourceIPHash, and gRPC

- `Service.spec.sessionAffinity: ClientIP` plus `sessionAffinityConfig.clientIP.timeoutSeconds` keeps a client IP pinned to one endpoint via conntrack/IPVS hash. Useful for stateful protocols, hostile to deployment rollover (sessions stick to the old pod until timeout).
- gRPC over a single HTTP/2 connection multiplexes RPCs over one TCP connection; a single Service VIP without client-side LB sticks every RPC to one backend. Headless Service plus client-side resolver (gRPC-Go / xDS) is the canonical answer; an L7 Gateway (gRPC-aware) is the other.
- Long-lived WebSocket connections plus rolling deployment plus session affinity — every old-pod connection survives until the client disconnects. Plan termination grace and client-reconnect strategy.

## Output for this section

- Service surface inventory and types,
- EndpointSlice health (selectors actually match pods),
- `internalTrafficPolicy` and `externalTrafficPolicy` correctness against pod placement and replica count,
- topology-aware routing readiness,
- Ingress vs Gateway API posture, GA-channel resources in use vs experimental,
- migration path with annotation-to-route mapping for the controller in scope,
- session affinity and gRPC routing posture,
- findings, severity, and the next-step delegate (e.g. NetworkPolicy review if a route depends on cross-namespace traffic).
