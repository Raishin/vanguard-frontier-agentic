# Official Sources

Load these only when needed.

## Kubernetes upstream

- [Services, Load Balancing, Networking concepts](https://kubernetes.io/docs/concepts/services-networking/) — the canonical entry point and topic taxonomy.
- [Service](https://kubernetes.io/docs/concepts/services-networking/service/) — Service types, selectors, ports.
- [EndpointSlices](https://kubernetes.io/docs/concepts/services-networking/endpoint-slices/) — replaces the legacy Endpoints object; powers kube-proxy and Cilium socket-LB.
- [Service Internal Traffic Policy](https://kubernetes.io/docs/concepts/services-networking/service-traffic-policy/) — `internalTrafficPolicy: Cluster | Local` semantics.
- [Topology Aware Routing](https://kubernetes.io/docs/concepts/services-networking/topology-aware-routing/) — `service.kubernetes.io/topology-mode: Auto`.
- [Dual-stack](https://kubernetes.io/docs/concepts/services-networking/dual-stack/) — `ipFamilies` and `ipFamilyPolicy`.
- [Virtual IPs and Service Proxies (kube-proxy reference)](https://kubernetes.io/docs/reference/networking/virtual-ips/) — iptables, IPVS, nftables, kernelspace modes.
- [DNS for Services and Pods](https://kubernetes.io/docs/concepts/services-networking/dns-pod-service/) — canonical DNS naming, search-list construction.
- [NodeLocal DNSCache](https://kubernetes.io/docs/tasks/administer-cluster/nodelocaldns/) — DaemonSet architecture, conntrack rationale, OOM risks.
- [Pod Network](https://kubernetes.io/docs/concepts/cluster-administration/networking/) — cluster networking model.
- [Network Plugins reference](https://kubernetes.io/docs/concepts/extend-kubernetes/compute-storage-net/network-plugins/) — CNI integration.

## Gateway API (sig-network)

- [Gateway API home](https://gateway-api.sigs.k8s.io/) — resource taxonomy, role-oriented model, conformance.
- [API stability and channels (Standard / Experimental)](https://gateway-api.sigs.k8s.io/concepts/versioning/) — what's GA vs experimental.
- [GAMMA service mesh (Standard since v1.1.0)](https://gateway-api.sigs.k8s.io/mesh/gamma/) — routes attached directly to Services.
- [Implementations](https://gateway-api.sigs.k8s.io/implementations/) — controller list and conformance reports.

## Cilium

- [Cilium documentation home](https://docs.cilium.io/en/stable/).
- [Networking concepts (routing, IPAM, masquerading, MTU)](https://docs.cilium.io/en/stable/network/concepts/).
- [Kube-proxy replacement](https://docs.cilium.io/en/stable/network/kube-proxy-replacement/) — eBPF socket-LB, requirements, mode (Strict / Probe), fallback paths.
- [System requirements](https://docs.cilium.io/en/stable/operations/system-requirements/) — kernel feature matrix.
- [ClusterMesh](https://docs.cilium.io/en/stable/network/clustermesh/) — multi-cluster service discovery and identity.
- [Hubble Observability](https://docs.cilium.io/en/stable/observability/hubble/).

## CoreDNS

- [Plugin index (Corefile structure)](https://coredns.io/plugins/) — every plugin and its options.
- [`kubernetes` plugin](https://coredns.io/plugins/kubernetes/) — `pods insecure | verified`, `endpoint_pod_names`, `ttl`, `fallthrough`, `autopath`.
- [`forward` plugin](https://coredns.io/plugins/forward/) — `max_concurrent`, health-checking, policy.
- [`cache` plugin](https://coredns.io/plugins/cache/) — TTL, success/denial/prefetch.

## Multi-cluster

- [KEP-1645: Multi-Cluster Services API](https://github.com/kubernetes/enhancements/blob/master/keps/sig-multicluster/1645-multi-cluster-services-api/README.md) — `ServiceExport`, `ServiceImport`.
- [Submariner](https://submariner.io/) — CNI-agnostic multi-cluster gateway and Globalnet.
- [Istio multi-cluster](https://istio.io/latest/docs/setup/install/multicluster/) — primary-remote, multi-primary topologies.

## Cluster autoscaling for DNS

- [`cluster-proportional-autoscaler`](https://github.com/kubernetes-sigs/cluster-proportional-autoscaler) — the canonical CoreDNS replica autoscaler.

## Status of the Linux Foundation Kubernetes Network Engineer program

As of 2026-05-07, the Linux Foundation's "[Kubernetes Network Engineer Program](https://training.linuxfoundation.org/kubernetes-network-engineer-program/)" page reports that the CNCF and Linux Foundation Education are *starting to work on* a Certified Kubernetes Network Engineer (CKNE) certification. **The published page does not yet list curriculum domains, learning objectives, or course modules.** This skill is therefore grounded in upstream Kubernetes / Gateway API / Cilium / CoreDNS documentation and not in the (unpublished) CKNE blueprint. When the LF publishes the CKNE domain list, the skill should be re-verified and the `last_verified` date bumped.
