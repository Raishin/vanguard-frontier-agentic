# 🐝 Cilium Skills

<p align="center">
  <!-- 🖼️ Add a Cilium logo to assets/logos/cnative/cilium/ and update this path -->
  <span style="font-size:3.5em">🐝</span>
</p>

This folder contains Cilium-focused skills curated for this marketplace.

## Local marketplace portfolio

This folder contains **1** local Cilium skill:

- `cilium-network-policy-review`

## Portfolio posture

Cilium skills for evidence-backed eBPF networking review covering the three policy formats (`NetworkPolicy`, `CiliumNetworkPolicy`, `CiliumClusterwideNetworkPolicy`), L7 policy via embedded Envoy, ClusterMesh cross-cluster semantics, Hubble flow observability, and `CiliumEgressGatewayPolicy` for SNAT egress.

These skills are intentionally conservative:

- prefer `kubectl get networkpolicies,ciliumnetworkpolicies,ciliumclusterwidenetworkpolicies,ciliumegressgatewaypolicies -A -o yaml` for live policy state grounding before any review
- treat **removal of a default-deny `NetworkPolicy`** as a critical finding — pods become reachable from any source/destination
- challenge `CiliumNetworkPolicy` egress with `toCIDRSet: [0.0.0.0/0]` — unrestricted egress = data exfiltration path
- challenge `policy-default-local-cluster` flag changes in ClusterMesh — cross-cluster policy semantics change globally for every existing policy
- challenge `CiliumEgressGatewayPolicy` IP collisions — two policies SNATing to the same IP cause silent connection breakage
- prefer `cilium clustermesh inspect-policy-default-local-cluster` before any flag flip — it lists every policy that would change behavior
- use official Cilium documentation (docs.cilium.io) for policy syntax, CRD versions, ClusterMesh setup, and L7 policy semantics

Run `npm run validate` after changing cataloged Cilium skills.
