# Workflow and Output Contract

## Workflow

### Step 1 — Identify the policy format

Cilium supports three formats with different scopes and capabilities:

1. **`NetworkPolicy`** (`networking.k8s.io/v1`) — Kubernetes-native, namespace-scoped, L3/L4 only.
2. **`CiliumNetworkPolicy`** (`cilium.io/v2`, "CNP") — namespace-scoped, L3-L7 (HTTP, Kafka, DNS), FQDN matching, ICMP, identity-based selectors via Cilium endpoint identities.
3. **`CiliumClusterwideNetworkPolicy`** (`cilium.io/v2`, "CCNP") — cluster-wide, same capabilities as CNP, applies across all namespaces.

A namespace can have multiple policies of all three formats simultaneously. The effective policy is the **union** of allows: any policy that allows traffic permits it.

Reference: [Network Policy overview](https://docs.cilium.io/en/stable/network/kubernetes/policy/).

### Step 2 — Verify default-deny posture in the affected namespace

Cilium follows the Kubernetes NetworkPolicy semantic: pods with **at least one ingress policy selecting them** become deny-by-default for ingress; pods with **at least one egress policy selecting them** become deny-by-default for egress. Without any policy selecting a pod, all traffic is allowed.

Critical findings:

- Removing the only ingress `NetworkPolicy` selecting a workload — the workload becomes reachable from any pod, any namespace, any cluster (if ClusterMesh).
- Adding a workload to a namespace that has no namespace-wide `default-deny` policy — the new workload is allow-by-default.

Recommended baseline: a `default-deny-all` `NetworkPolicy` per namespace plus explicit `CiliumNetworkPolicy` resources that allow specific intra-namespace and cross-namespace flows.

### Step 3 — Audit L7 rules and Envoy proxy requirement

`CiliumNetworkPolicy` and `CiliumClusterwideNetworkPolicy` support L7 rules via Cilium's embedded Envoy:

- `toPorts.rules.http` — method, path, host, header matching.
- `toPorts.rules.kafka` — Kafka API key matching, topic-level allow.
- `toPorts.rules.dns` — DNS FQDN allowlist for egress.

L7 rules require the Envoy proxy to be enabled. Without Envoy, policy with L7 fields **either fails admission or is enforced only at L3/L4**, depending on Cilium version. Confirm before relying on L7.

Stress-tests:

- L7 HTTP rule with `path: /admin` but the policy applies to a namespace where pods talk via gRPC — the HTTP path matcher does nothing for HTTP/2 stream multiplexing.
- L7 DNS rule with FQDN `*.example.com` — wildcard match is supported for DNS but the destination port still matters; verify port 53 UDP/TCP allowed at L4.

Reference: [L7 Policy in Cilium](https://docs.cilium.io/en/stable/security/policy/language/#layer-7-examples).

### Step 4 — Audit egress (the exfiltration path)

Egress is the most-overlooked side of network policy. Critical findings:

- `egress` rules with `toCIDRSet: [{cidr: 0.0.0.0/0}]` and no `except` for internal CIDRs (RFC 1918, link-local, cloud metadata service IPs like `169.254.169.254`) — allows pod to reach the cloud metadata service and exfiltrate cloud credentials.
- `egress` with `toEndpoints: []` (empty selector) — the empty selector matches **everything** in Cilium semantics; this is broader than `toEndpoints` not being present at all.
- `egress` allowing `toFQDNs.matchPattern: '*'` — wildcard DNS matching with no narrow allowlist.

Recommended baseline: explicit `toEndpoints` for in-cluster, `toCIDRSet` with `except` for the cloud metadata CIDR, `toFQDNs` for known external services.

### Step 5 — Audit `CiliumEgressGatewayPolicy`

`CiliumEgressGatewayPolicy` assigns a SNAT egress IP for selected pods exiting the cluster — used when external systems require a stable source IP for firewall allowlisting.

Stress-tests:

- Two `CiliumEgressGatewayPolicy` resources with the same `egressIP` — both policies match different pods, both rewrite to the same source IP, and the response routing on the gateway node breaks for one or both. The result is intermittent connection drops.
- `egressIP` not actually assigned to a NIC on the chosen gateway node — Cilium silently fails to apply, traffic falls back to default node SNAT.
- `nodeSelector` matches multiple nodes — only one acts as gateway; failover is not automatic.
- `destinationCIDRs: ['0.0.0.0/0', '::/0']` — every external connection from the selected pods is SNATed; a more narrow CIDR is usually appropriate.
- Missing `nodeSelector` — policy applies to all nodes, which is rarely the intent.

Reference: [Cilium Egress Gateway](https://docs.cilium.io/en/stable/network/egress-gateway/egress-gateway/).

### Step 6 — Audit ClusterMesh policy semantics

When ClusterMesh is enabled, identity-based policy selectors (`namespaceSelector`, `endpointSelector`) match across cluster boundaries. Two semantics are possible:

1. **`policy-default-local-cluster: false` (default in older versions)** — selectors match endpoints in any peer cluster. A `namespaceSelector: {kubernetes.io/metadata.name: prod}` matches `prod` in this cluster AND `prod` in every peer cluster.
2. **`policy-default-local-cluster: true` (default in 1.16+)** — selectors match only the local cluster unless the policy explicitly sets `cluster: <peer-cluster>` on the selector.

**A flag flip changes every existing policy's effective scope simultaneously.** Cilium ships `cilium clustermesh inspect-policy-default-local-cluster` specifically to preview which policies would be affected.

Stress-tests:

- ClusterMesh deployment with mixed clusters at different `policy-default-local-cluster` settings — confusing semantics; one cluster's policy may match peer endpoints while another's does not.
- Migrating from `false` to `true` — every policy that previously matched cross-cluster identities now silently stops matching them. **This is a documented operational landmine.**

Reference: [Cilium ClusterMesh](https://docs.cilium.io/en/stable/network/clustermesh/) and [`cilium clustermesh inspect-policy-default-local-cluster`](https://docs.cilium.io/en/stable/cmdref/cilium_clustermesh_inspect-policy-default-local-cluster/).

### Step 7 — Use Hubble to verify enforcement

Static policy review is not enough. Use Hubble to confirm what the policy actually does:

```shell
# Watch ingress drops to a workload — should be empty if allow rules are correct
hubble observe --to-namespace <ns> --to-pod <pod-prefix> --verdict DROPPED --last 1000

# Watch egress allows from a workload — confirms the workload reaches expected destinations
hubble observe --from-namespace <ns> --from-pod <pod-prefix> --verdict FORWARDED --last 100

# DNS resolution by FQDN policy
hubble observe --type dns --last 100
```

Reference: [Hubble Observability](https://docs.cilium.io/en/stable/observability/hubble/).

### Step 8 — Stress-test operational hygiene

- Prefer `CiliumNetworkPolicy` over `NetworkPolicy` when L7 is needed — converting back later is harder than starting with the richer format.
- Prefer named `endpointSelector` labels over IP CIDRs for in-cluster traffic — IPs change, labels survive pod recreation.
- Prefer `toFQDNs` over `toCIDRSet` for external services with stable hostnames — DNS rotation no longer breaks the policy.
- Avoid `CiliumClusterwideNetworkPolicy` for namespace-scoped concerns — cluster-wide blast radius.
- Test policy changes in a dev or staging cluster first — eBPF program reload happens asynchronously, and a misordered apply during rollout can briefly break traffic.

## Output

Return:

- **target**: which policy format and which scope,
- **evidence level**: `live evidence` / `documentation-based` / `sanitized user evidence` / `inference`,
- **default-deny posture** in the namespace(s),
- **L7 enforcement assessment**: Envoy proxy enabled / required, whether L7 rules will actually run,
- **egress posture**: cloud-metadata service blocked, CIDR scope, FQDN allowlist hygiene,
- **ClusterMesh assessment** when applicable (cross-cluster semantics, `policy-default-local-cluster` value),
- **risk findings** (with severity: high / medium / low),
- **safest next actions** with sample manifest changes and `hubble observe` commands to verify,
- **rollback plan**: how to revert the change without leaving pods unreachable,
- **assumptions and missing facts**.

## Security notes

- Never recommend removing a default-deny policy without a confirmed replacement that explicitly allows required flows.
- Never recommend `toCIDRSet: [{cidr: 0.0.0.0/0}]` without an `except` block covering the cloud metadata service IP and any other sensitive internal CIDRs.
- Never recommend changing `policy-default-local-cluster` without first running `cilium clustermesh inspect-policy-default-local-cluster` and reviewing every affected policy.
- Do not print Cilium ClusterMesh peer Secrets or agent service account tokens.
