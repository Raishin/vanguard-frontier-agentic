# Official Sources

Load these only when needed:

- [Cilium documentation home](https://docs.cilium.io/en/stable/) — use as the entry point for any Cilium question.
- [Network Policy](https://docs.cilium.io/en/stable/network/kubernetes/policy/) — use for the three policy formats (`NetworkPolicy`, `CiliumNetworkPolicy`, `CiliumClusterwideNetworkPolicy`) and how Cilium distributes them.
- [Policy language reference](https://docs.cilium.io/en/stable/security/policy/language/) — use for `endpointSelector`, `toEndpoints`, `toCIDRSet`, `toFQDNs`, `toServices`, `toEntities`, L7 HTTP/Kafka/DNS rule syntax.
- [Policy enforcement modes](https://docs.cilium.io/en/stable/security/policy/intro/) — use for `default`, `always`, `never` enforcement modes and Cilium's identity-based model.
- [ClusterMesh overview](https://docs.cilium.io/en/stable/network/clustermesh/) — use for multi-cluster service discovery, identity propagation, and cross-cluster policy.
- [`cilium clustermesh inspect-policy-default-local-cluster`](https://docs.cilium.io/en/stable/cmdref/cilium_clustermesh_inspect-policy-default-local-cluster/) — use before any flag flip; lists every policy whose scope would change.
- [Egress Gateway](https://docs.cilium.io/en/stable/network/egress-gateway/egress-gateway/) — use for `CiliumEgressGatewayPolicy` SNAT semantics, gateway node selection, and IP collision behavior.
- [Hubble Observability](https://docs.cilium.io/en/stable/observability/hubble/) — use for flow observation, drop debugging, and policy verification.
- [Hubble CLI reference](https://docs.cilium.io/en/stable/cmdref/hubble/) — use for `hubble observe` filters and output formats.
- [Cilium Ingress / Gateway API](https://docs.cilium.io/en/stable/network/servicemesh/) — use when Cilium service mesh (sidecar-free) is in scope alongside policy.
- [Cilium Service Mesh Beta / GA notes](https://docs.cilium.io/en/stable/network/servicemesh/) — use to understand when Cilium service mesh replaces Istio in the L7 enforcement path.
- [Tetragon documentation](https://tetragon.io/docs/) — use when runtime security observability and enforcement is in scope alongside Cilium network policy.
- [Cilium release notes](https://github.com/cilium/cilium/releases) — use for version-specific behavior changes, especially around `policy-default-local-cluster` defaults.

## Grounded insights worth carrying into the skill

- Cilium supports three policy formats simultaneously in one cluster: native `NetworkPolicy`, `CiliumNetworkPolicy` (CNP) for namespace-scoped L3-L7, and `CiliumClusterwideNetworkPolicy` (CCNP) for cluster-wide L3-L7.
- `CiliumNetworkPolicy` adds capabilities native NetworkPolicy lacks: FQDN matching (`toFQDNs`), L7 HTTP/Kafka/DNS rules, identity-based selectors (Cilium endpoint identities derived from labels), `toEntities` (cluster, world, host, kube-apiserver), and ICMP rules.
- Cilium's effective policy is the **union** of all selecting allows. There is no DENY action — restriction comes from default-deny on selected pods plus explicit allow rules that collectively define the allowed graph.
- A pod becomes deny-by-default only when **at least one ingress policy selects it for ingress** or **at least one egress policy selects it for egress**. Pods with no selecting policy are allow-all in that direction.
- ClusterMesh's `policy-default-local-cluster` flag changes whether identity selectors match endpoints in peer clusters. Setting it to `true` (the newer default in 1.16+) makes selectors local-only unless the policy explicitly opts into cross-cluster matching with `cluster: <name>`. Migrating an existing cluster from `false` to `true` silently breaks every policy that depended on cross-cluster matching.
- `CiliumEgressGatewayPolicy` controls SNAT egress IPs for selected pods. The most common operational pitfall is two policies SNATing to the same `egressIP` — connection-tracking on the gateway node confuses replies, and connections drop intermittently.
- L7 policy fields (HTTP, Kafka, DNS) require Cilium's embedded Envoy proxy. Without Envoy enabled, the L7 fields are either rejected at admission or silently dropped depending on the Cilium version. Always verify Envoy state before relying on L7.
- `toCIDRSet: [{cidr: 0.0.0.0/0}]` with no `except` for the cloud metadata service IP (`169.254.169.254` on AWS/Azure/GCP) is the exfiltration path AWS Capital One famously suffered from. Cilium's `except` clause is the right tool to block it while still allowing general internet egress.
- Hubble flow observation is the only reliable way to verify what Cilium's eBPF programs are actually doing — static policy review can miss conflicts between policies that share endpoint selectors but differ in port or L7 rules.
- Tetragon (eBPF runtime security) is a separate Cilium-affiliated project, not part of Cilium itself. When a review touches runtime syscall monitoring, link to Tetragon docs explicitly rather than assuming Cilium provides it.
