# Multi-Cluster Topology and Egress

This reference is about *topology* — how clusters discover and address each other, and how pods leave the cluster — not about cross-cluster *policy*. Cross-cluster policy semantics (e.g. Cilium ClusterMesh `policy-default-local-cluster`) belong to `cilium-network-policy-review`.

## Step 1 — Capture the multi-cluster posture

- Number of clusters and their relationship: peers (mesh), hub-and-spoke, hierarchical.
- Pod CIDRs per cluster and Service CIDRs per cluster — non-overlap is the prerequisite for every multi-cluster scheme.
- Cross-cluster discovery mechanism in use: Cilium ClusterMesh, Submariner, Linkerd multi-cluster, Istio multi-primary or primary-remote, KEP-1645 Multi-Cluster Services API (MCS-API), or hand-rolled `ExternalName` chains.
- Identity boundary: shared OIDC (workload identity federation), per-cluster identities with explicit trust, or no shared identity (in which case workload-identity review is required before cross-cluster authn is meaningful).
- Underlay reachability: are pods directly routable across clusters (BGP, AWS VPC peering, VPN), or is traffic gateway-mediated (Submariner gateway, Istio east-west gateway)?

## Step 2 — Stress-test pod / service CIDR overlap

The first failure of every multi-cluster scheme is overlapping CIDRs. Reject any design that does not declare:

- Per-cluster Pod CIDR ranges that do not intersect any peer's Pod CIDR.
- Per-cluster Service CIDR ranges that do not intersect any peer's Service CIDR (relevant for ClusterMesh and MCS-API where a Service IP from cluster A may be routed from cluster B).
- A reservation plan with documented headroom — adding a third cluster later usually requires rebuilding overlapping clusters.

If overlap is unavoidable (e.g. existing brownfield clusters), the only correct answer is per-cluster NAT — Submariner's Globalnet and Istio's east-west gateway both implement variants of this, at the cost of losing client-IP visibility cross-cluster.

## Step 3 — Stress-test the discovery mechanism choice

| Mechanism | Strength | Real failure mode |
|---|---|---|
| Cilium ClusterMesh | eBPF-direct, no extra hops; identity propagation; Hubble flow visibility cross-cluster. | Requires Cilium on every cluster; CA trust setup; the `policy-default-local-cluster` flag flip is a documented landmine (see `cilium-network-policy-review`). |
| Submariner | CNI-agnostic, can stitch heterogeneous clusters; Globalnet handles CIDR overlap. | Gateway nodes are choke points (single tunnel pair per gateway); MTU subtraction for the IPsec tunnel must be planned; service discovery via Lighthouse adds DNS hops. |
| Istio multi-primary / primary-remote | Mesh-native; mTLS-everywhere; locality-aware load balancing. | Operationally complex; East-West Gateway is another L7 hop; cross-cluster health-checking depends on Istio version. |
| Linkerd multi-cluster | Simple model (mirrored Services); no central control. | Mirror Service per cross-cluster Service grows the API server load; pod identity federation requires explicit trust setup. |
| KEP-1645 MCS-API (`ServiceExport`, `ServiceImport`) | Vendor-neutral; standardized. | Implementations vary in maturity; check that the chosen controller (mcs-api-controller, AWS Cloud Map MCS, GKE Multi-Cluster Services) actually supports the channel needed. |
| Hand-rolled `ExternalName` chains | No new infra. | No identity propagation; no health awareness; DNS rotation is the only failover; this is a smell, not a design. |

## Step 4 — Stress-test egress topology

How pods leave the cluster shapes IAM scoping, firewall rules, and observability.

Patterns:

- **Default node SNAT** — pods exit via the node's primary interface, source IP is the node IP. Simple, but every pod looks the same to external systems; firewall allowlists must allow the entire node range.
- **CiliumEgressGatewayPolicy** — selected pods SNAT through a designated gateway node with a stable egress IP. Allows fine-grained external allowlisting. Stress-tests for this pattern belong to `cilium-network-policy-review`.
- **AWS VPC CNI / Azure CNI native** — pods get VPC IPs and route directly to a NAT Gateway / NAT Router; egress IP is the NAT GW's. IAM must be scoped on a workload-identity basis since the source IP is shared.
- **Per-namespace egress proxy** — an outbound HTTP/SOCKS proxy in each namespace; pods are forced through it via `HTTP_PROXY` env or NetworkPolicy. Adds a single audit point but is a single failure point.
- **No egress** — air-gapped clusters where every external dependency is mirrored inside the cluster (registries, package mirrors, time servers). The only safe IAM posture for highly regulated workloads, but operationally heavy.

Stress-tests the review must apply:

- A pod with `HTTP_PROXY` set but `NO_PROXY` not configured to include `.svc.cluster.local` and the cluster CIDR — every in-cluster call goes through the egress proxy and back, doubling east-west latency.
- An egress NAT Gateway shared across clusters in the same VPC — billing is per-GB on AWS; cross-AZ egress through a single AZ's NAT GW is a documented cost trap.
- A managed cluster that disables NodeLocalDNS *and* uses VPC DNS — DNS queries become the dominant egress traffic; NAT GW cost spikes proportionally.
- A workload that does TLS to an external service through the egress proxy without the proxy being a CONNECT-method TLS proxy — the proxy can only see SNI, not L7, and policy claims of "L7 control" are false.

## Step 5 — East-west vs north-south

Keep the language honest in any review:

- **North-south** — traffic crossing the cluster boundary. Owners: Ingress / Gateway / LoadBalancer Service / egress topology.
- **East-west** — traffic between pods in the same cluster (or cross-cluster but inside the multi-cluster trust domain). Owners: kube-proxy / Cilium dataplane / Service mesh.
- **Diagonal** — a pod calling out to a managed cloud service (RDS, Cosmos DB, OCI Autonomous DB) or a SaaS API. This is north-south even though it does not feel like it; cloud network policy applies, not just NetworkPolicy.

Many reviews conflate these. Naming the direction explicitly clarifies which controllers, which policy formats, and which observability pane apply.

## Output for this section

- Cluster relationship map and CIDR plan (no overlap),
- discovery mechanism and identity boundary,
- egress topology and source-IP plan,
- findings on overlap, gateway choke points, IAM scoping for shared egress,
- explicit handoffs: policy correctness → `cilium-network-policy-review` or `istio-ambient-mesh-review`; IAM scoping → workload-identity-review; cloud-side network architecture → AWS/Azure/OCI network architect agents.
