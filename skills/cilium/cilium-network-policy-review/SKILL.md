---
name: cilium-network-policy-review
description: Use this skill for Cilium network policy review across the three policy formats (Kubernetes NetworkPolicy, CiliumNetworkPolicy, CiliumClusterwideNetworkPolicy), L7 policy via embedded Envoy, ClusterMesh cross-cluster semantics, Hubble flow observability, and CiliumEgressGatewayPolicy. Trigger when the user asks whether a network policy is too broad, whether default-deny is in place, whether L7 rules will actually be enforced, whether ClusterMesh policy semantics are correct, or whether an egress gateway IP collision is possible.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
---

# Cilium Network Policy Review

## Purpose

Review Cilium policy resources against zero-trust correctness, blast radius, and the operational traps unique to eBPF-backed networking. Cilium's policy surface is broader than native Kubernetes NetworkPolicy — `CiliumNetworkPolicy` adds L7 rules, FQDN matching, ICMP control, and identity-based selectors; `CiliumClusterwideNetworkPolicy` applies cluster-wide; `CiliumEgressGatewayPolicy` controls SNAT egress IPs; and `policy-default-local-cluster` changes how policy evaluates across ClusterMesh.

## Lean operating rules

- Prefer live cluster evidence (`kubectl get networkpolicies,ciliumnetworkpolicies,ciliumclusterwidenetworkpolicies,ciliumegressgatewaypolicies -A -o yaml`, `cilium policy get`, `cilium clustermesh inspect-policy-default-local-cluster`, and Hubble flow observation) when the active client exposes it; otherwise fall back to official Cilium documentation (docs.cilium.io) and sanitized YAML.
- Separate confirmed facts from inference. If Cilium agent state, ClusterMesh peer status, or Hubble flow data was not queried, say so.
- Treat **removal of a default-deny `NetworkPolicy`** in a namespace as a critical finding — pods become reachable from any source/destination unless another policy provides isolation.
- Treat `CiliumNetworkPolicy` egress with `toCIDRSet: [{cidr: 0.0.0.0/0}]` (no `except` for sensitive CIDRs) as a critical finding — unrestricted egress is a documented data exfiltration path.
- Treat any change to `policy-default-local-cluster` in a ClusterMesh deployment as critical-blast-radius — every existing policy's cross-cluster semantics flip simultaneously.
- Challenge `CiliumEgressGatewayPolicy` with the same `egressIP` used in two policies — silent connection breakage when both match.
- Challenge L7 rules in `CiliumNetworkPolicy` for namespaces where Envoy proxy is not enabled — L7 fields require the proxy.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.

## References

Load these only when needed:

- [Evidence path and tooling](references/mcp-and-evidence.md) — use when choosing live cluster evidence, confirming Cilium version and ClusterMesh state, or switching to documentation mode.
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review, applying stress checks across the three policy formats and ClusterMesh, or formatting the final answer.
- [Official sources](references/official-sources.md) — use when you need the detailed Cilium documentation list, CRD schema, and grounded insights.

## Response minimum

Return, at minimum:

- the scoped target (namespace `NetworkPolicy`, namespace `CiliumNetworkPolicy`, cluster-wide `CiliumClusterwideNetworkPolicy`, `CiliumEgressGatewayPolicy`) and evidence level,
- the default-deny posture in the affected namespace(s),
- the L7 enforcement assessment (Envoy proxy enabled / required) and whether L7 rules will actually run,
- the ClusterMesh assessment when applicable (`policy-default-local-cluster` semantics),
- the safest next actions and rollback plan,
- the assumptions or blockers that prevent stronger conclusions.
