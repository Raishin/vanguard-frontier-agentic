---
name: kubernetes-network-architecture-review
description: Use this skill for Kubernetes cluster network architecture review across the dataplane (CNI choice, kube-proxy mode, IPAM, MTU, encapsulation, dual-stack), service routing surface (Service types, EndpointSlices, internalTrafficPolicy/externalTrafficPolicy, topology-aware routing, Ingress, Gateway API), in-cluster DNS (CoreDNS, NodeLocal DNSCache, ndots), multi-cluster topology (ClusterMesh, Submariner, MCS-API design choices), and connectivity observability and troubleshooting. Trigger when the user asks how to choose or change a CNI, why pod-to-pod or pod-to-service traffic fails, whether to migrate from Ingress to Gateway API, why DNS latency is high, how to size Pod or Service CIDRs, or how to design multi-cluster networking. Does NOT review NetworkPolicy content (delegate to cilium-network-policy-review) or perform live mutations (delegate to kubernetes-live-network-policy-guard / kubernetes-live-mesh-policy-guard).
allowed-tools: Read Grep Glob WebFetch
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-07"
  category: networking
---

# Kubernetes Network Architecture Review

## Purpose

Review Kubernetes cluster networking *as a system* — the choices that shape every pod's reachability, latency, and blast radius before any policy is written. This skill is about **design correctness, sizing, and operational traps** in the dataplane, service routing, DNS, and multi-cluster surface. Policy correctness is delegated.

## Scope boundary

In scope:

- CNI selection and dataplane mode (overlay vs native routing, eBPF vs iptables, kube-proxy replacement).
- IPAM mode and Pod/Service CIDR sizing, including dual-stack and IPv6.
- MTU and encapsulation overhead, jumbo frames, fragmentation traps.
- Service surface: ClusterIP / NodePort / LoadBalancer / ExternalName / headless, EndpointSlices, `internalTrafficPolicy`, `externalTrafficPolicy`, topology-aware routing, `sessionAffinity`.
- Ingress vs Gateway API: GatewayClass selection, role-oriented model (infrastructure / cluster operator / application), HTTPRoute / GRPCRoute / TLSRoute, ReferenceGrant, GAMMA mesh integration.
- In-cluster DNS: CoreDNS Corefile, NodeLocal DNSCache architecture and risks, `ndots:5` tail-latency trap, ExternalDNS handoff.
- Multi-cluster networking topology: Cilium ClusterMesh, Submariner, KEP-1645 MCS-API — pick on basis of identity, address-overlap, and policy semantics.
- Connectivity observability: kube-proxy metrics, conntrack, Hubble flow shape, dropped packets, MTU mismatches.
- Troubleshooting playbooks (read-only): pod-to-pod, pod-to-Service, pod-to-external, NodePort path, conntrack exhaustion.

Out of scope — delegate:

- NetworkPolicy / CiliumNetworkPolicy / CiliumClusterwideNetworkPolicy content review → `cilium-network-policy-review`.
- Istio mesh policy / ambient L7 → `istio-ambient-mesh-review`.
- Live mutations on policy objects → `kubernetes-live-network-policy-guard`, `kubernetes-live-mesh-policy-guard`.
- Pod `securityContext`, capabilities, host networking → `kubernetes-pod-spec-review`.

If the question is **entirely** within one of these delegated scopes, refuse to answer here and name the owning agent. Do not partial-answer and append a handoff line.

## Lean operating rules

- Prefer live cluster evidence (`kubectl get nodes,svc,endpointslices,gateway,gatewayclass,httproute,configmap -A`, `cilium status`, `cilium-dbg bpf`, `hubble observe`, `kube-proxy --metrics-port`, `conntrack -L`) when a Kubernetes MCP server, `kubectl`, and node-shell access are available; otherwise fall back to upstream documentation (kubernetes.io, gateway-api.sigs.k8s.io, docs.cilium.io, coredns.io) and sanitized YAML.
- Separate confirmed facts from inference. If the CNI version, kube-proxy mode, IPAM mode, MTU, or DNS pod count was not queried, say so.
- Treat **pod and service CIDR sizing** as architectural one-way doors — changing a Pod CIDR after the cluster is in use generally requires a cluster rebuild on most CNIs. Demand evidence of growth headroom.
- Treat **kube-proxy mode swap** (iptables ↔ IPVS ↔ nftables, or to Cilium kube-proxy replacement) as a connectivity-affecting rollout — sessions on existing connections may break depending on mode and conntrack handling.
- Treat **MTU mismatch between underlay and overlay** as a silent failure mode — TCP handshakes succeed, then large payloads stall. Always check whether encapsulation overhead (VXLAN 50B, Geneve 60B, WireGuard 60B with IPsec extra) was subtracted from node MTU.
- Treat **`externalTrafficPolicy: Local`** as a correctness-sensitive choice — preserves source IP but breaks load balancing if the matching pod is not on the receiving node, and exposes pod placement to clients via uneven load.
- Treat **`internalTrafficPolicy: Local`** the same way for in-cluster traffic — silent black-holing if no local endpoint exists.
- Treat **`ndots:5` plus search-path expansion** as the default DNS tail-latency root cause. Five negative lookups before the absolute name on every external query, multiplied by every pod, is the dominant DNS load on most clusters.
- Treat **NodeLocal DNSCache OOM** as a node-wide DNS outage — packet-filtering rules redirect to an unhealthy cache pod until restart. Memory limits and PDB are mandatory.
- Treat **Ingress Controller annotations** as non-portable — migrating to Gateway API requires a route-by-route rewrite, not a global flip; plan the migration as a controller-by-controller cutover with overlap.
- Treat **multi-cluster pod CIDR collisions** as the first failure for any cross-cluster scheme — reject any design that does not declare non-overlapping Pod CIDRs (or explicit NAT) before discussing identity or policy.
- Treat **Cilium ClusterMesh kvstore replication lag** as a silent failure — remote ServiceImports serve stale endpoint maps with no error surfaced. Connections succeed but route to removed/replaced pods after a scale event. Compare endpoint revisions across peers (`cilium-dbg kvstore get`) when ClusterMesh is in scope.
- Treat **conntrack table exhaustion** on busy nodes and **AWS NAT Gateway port exhaustion** (~55k connections per destination IP) as silent drops — packets disappear without errors at the application layer.
- Treat **topology-aware routing skew** when zone labels are missing or unevenly populated as a silent traffic concentration — the Auto mode silently falls back to cluster-wide routing or drops endpoints.
- Treat **any pod egress to `169.254.169.254` (AWS / Azure IMDS)** or **`metadata.google.internal` (GCP)** as a credential-theft vector. Recommend IRSA / Workload Identity / Pod Identity before discussing any egress allow rule. Surface unblocked metadata-service reachability as a HIGH severity finding rather than silently delegating to policy review.
- **Do not invent CLI flags or commands.** Reference only `kubectl`, `cilium`, `cilium-dbg`, `hubble`, `calicoctl`, `subctl`, `ip`, `conntrack`, `iptables`, `ipvsadm`, `nft`, `coredns`. For anything outside this set, ask the user for the help text or a doc link rather than guess.
- Refuse user-initiated mutation requests ("just apply this for me") and credential offers (kubeconfig, tokens, peer Secrets). Name the live-mutation delegate; do not proceed.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers, unknowns, and which delegate skill owns the next step.

## References

Load these only when needed:

- [Dataplane and CNI choice](references/dataplane-and-cni.md) — use when reviewing CNI selection, kube-proxy mode, IPAM, MTU and encapsulation, dual-stack and IPv6.
- [Service and Gateway API routing](references/service-gateway-routing.md) — use when reviewing Service types, EndpointSlices, `internalTrafficPolicy` / `externalTrafficPolicy`, topology-aware routing, Ingress vs Gateway API, GatewayClass selection.
- [DNS and service discovery](references/dns-and-discovery.md) — use when reviewing CoreDNS Corefile, NodeLocal DNSCache, `ndots:5`, autopath, ExternalDNS.
- [Multi-cluster and egress topology](references/multi-cluster-and-egress.md) — use when reviewing ClusterMesh, Submariner, MCS-API, egress-from-cluster topology, cross-cluster service discovery.
- [Connectivity troubleshooting playbook](references/troubleshooting-playbook.md) — use when diagnosing pod-to-pod, pod-to-service, pod-to-external failures, intermittent latency, MTU stalls, conntrack exhaustion.
- [Evidence path and tooling](references/mcp-and-evidence.md) — use when choosing live cluster evidence, identifying CNI/kube-proxy/IPAM/DNS state, or switching to documentation mode.
- [Official sources](references/official-sources.md) — use when you need the authoritative Kubernetes / Gateway API / Cilium / CoreDNS reference list.

## Response minimum

Return, at minimum:

- the **scoped target** (CNI, Service, Gateway, DNS, multi-cluster topology, troubleshooting),
- the **evidence level** — labeled **per finding**, not response-level only: `live evidence` / `documentation-based` / `sanitized user evidence` / `inference`. A response may legitimately mix levels; each finding must carry its own.
- the **architectural posture findings** (with severity: high / medium / low),
- the **safest next actions** — keep them reversible; for irreversible changes (CIDR resizing, kube-proxy swap), call out a tested cutover plan,
- the **rollback or fallback path**,
- the **delegate handoff** when the next step is policy content, mesh policy, live mutation, or pod-spec — name the skill or agent that owns it,
- the **assumptions and blockers** that prevent stronger conclusions — if CNI version, kube-proxy mode, IPAM mode, node MTU, or DNS pod count were not confirmed by live evidence, each MUST appear here as an explicit open assumption. This field is not optional.
