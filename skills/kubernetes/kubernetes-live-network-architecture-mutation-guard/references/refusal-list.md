# Hard refusal list — one-way doors

This document is the explicit `REFUSE` list. The agent must respond with `REFUSED — <rule>` and no execution attempt for any of the following. The cluster-side RBAC binding shipped with this skill also denies these verbs at the API server level — defense in depth.

The format for each entry is: **what is refused**, **why it's a one-way door**, **what the user should do instead**, **cluster-side blast radius if the prompt-level refusal is bypassed**.

---

## CNI replacement or uninstall

**Refused operations**: any `kubectl apply / delete` of a CNI's `DaemonSet`, ConfigMap, or CRD set (Cilium, Calico, Flannel, Weave, Antrea, Cilium chaining mode change). Includes `helm uninstall cilium`, `cilium uninstall`, equivalent for other CNIs.

**Why**: Replacing the CNI requires draining and re-IPAMing every Pod in the cluster. Many CNIs hold node state that the replacement does not understand. Cilium-to-Calico, Calico-to-Cilium, and any hybrid migration are full cluster rebuilds in practice, not cluster operations.

**Instead**: The architecture-review agent (`kubernetes-network-architecture-review-agent`) can produce a CNI-replacement cutover plan. Execution must be human-led with workload drain windows, fresh node groups, and a fallback cluster.

**Cluster-side blast radius if bypassed**: Pod-to-Pod connectivity stops for every workload until the replacement IPAM stabilises. NetworkPolicy enforcement disappears. mTLS in some service meshes (ambient Cilium-aware modes) breaks until the new CNI is fully up.

---

## kube-proxy mode swap

**Refused operations**: any change to `kube-proxy` `mode` ConfigMap (`iptables` ↔ `ipvs` ↔ `nftables`), and any change to or from Cilium kube-proxy replacement.

**Why**: Existing TCP connections rely on stable conntrack entries. Swapping the mode invalidates the in-kernel rules during the transition. Sessions on `sessionAffinity: ClientIP` Services may persist past the swap and route to the wrong endpoint. Some flows survive, some do not, deterministically by neither protocol nor application.

**Instead**: Plan a mode change as a per-node-pool rolling drain with explicit cordon, conntrack flush on the receiving traffic boundaries, and full session-tracking cutover. The architecture-review agent can produce the plan.

**Cluster-side blast radius if bypassed**: Service traffic stalls for some workloads, succeeds for others; debugging is hours-to-days because the failure is mode-transition state, not config.

---

## Node MTU change

**Refused operations**: any change to node interface MTU, CNI overlay MTU (Cilium `--mtu`, Calico `veth_mtu`, Flannel `MTU`), or VXLAN / Geneve / WireGuard encapsulation MTU.

**Why**: TCP handshake (small packets) succeeds, then the first response above the new path-MTU stalls because Path-MTU-Discovery ICMP is filtered by most cloud underlays. A wrong MTU causes silent payload-stall failure mode — connections look "alive" but never deliver.

**Instead**: Architecture review can produce the correct MTU calculation. Apply changes during a planned maintenance window with the encapsulation overhead pre-computed (VXLAN 50B, Geneve 60B, WireGuard 60B with IPsec extra) and verified per-node with `ping -M do -s <bytes>`.

**Cluster-side blast radius if bypassed**: Every Pod-to-Pod request larger than the new path-MTU stalls until the user discovers and reverts. Tail-latency dashboards spike; logs show no errors because TCP doesn't surface MTU drops.

---

## Pod CIDR or Service CIDR resize

**Refused operations**: any modification to `kube-controller-manager` flags (`--cluster-cidr`, `--service-cluster-ip-range`), Cilium IPAM CIDR pool resize that overlaps existing pool, any kube-apiserver flag flip on Service CIDR.

**Why**: Pod CIDR is allocated to nodes at node registration; existing nodes cannot be re-IPAMed without restart. Service CIDR is encoded into every existing Service's `spec.clusterIP`. Changing these requires a cluster rebuild for most CNIs.

**Instead**: Plan capacity at cluster creation with growth headroom. If overlap is unavoidable, use per-cluster NAT (Submariner Globalnet, ClusterMesh `policy-default-local-cluster`) — these are *workarounds*, not resizes.

**Cluster-side blast radius if bypassed**: CIDR collisions silently route traffic to the wrong workloads in multi-cluster setups. Services with allocated ClusterIPs outside the new range become unreachable. New Pods either fail to allocate IPs or get IPs that conflict with existing routes.

---

## Namespace deletion

**Refused operations**: `kubectl delete namespace ...` for any namespace.

**Why**: Namespace deletion cascades to every resource in the namespace. Deleting `kube-system`, `cilium`, `istio-system`, `linkerd`, `gateway-system`, `gke-gateway-system`, `kube-public`, `kube-node-lease` removes the cluster's control-plane operator. Even deleting a workload namespace is irreversible without a backup; if the namespace contained a `PersistentVolumeClaim`, the underlying `PersistentVolume` is lost when the PVC is finalized.

**Instead**: Delete specific resources within a namespace. Use `kubectl delete -n <ns> -l <selector>` with explicit selectors. Validate with `--dry-run=client -o yaml` before execution.

**Cluster-side blast radius if bypassed**: Deleting `kube-system` ends the cluster. Deleting `cilium` removes Pod-to-Pod networking. Deleting `istio-system` collapses mesh policy enforcement. Deleting a workload namespace deletes data.

The cluster-side RBAC binding for this guard explicitly omits `apiGroups: [""], resources: ["namespaces"]` for any verb. The API server returns `forbidden` on any namespace operation regardless of what the LLM emits.

---

## kube-system DaemonSet / Deployment writes

**Refused operations**: any `kubectl apply / patch / delete` on `DaemonSets` or `Deployments` in `kube-system`, including but not limited to `cilium`, `kube-proxy`, `coredns` (Deployment), `node-local-dns`, `metrics-server`, cloud-controller-manager.

**Why**: These workloads are the cluster control plane. A wrong replicas: 0 patch on `coredns` ends DNS for every Pod. A wrong tolerations change on `cilium` causes the Pod to evict and Pod-to-Pod connectivity to stop on the affected nodes. There is no fast rollback for a stopped CNI agent.

**Instead**: For DaemonSet-level changes, follow the upstream operator's documented upgrade path (Helm chart values, the operator's own CRD). For CoreDNS, use the `ConfigMap/coredns` path (permitted, see `permitted-mutations.md`) which exercises only the Corefile, never the Deployment.

**Cluster-side blast radius if bypassed**: Cluster-wide outage. Recovery requires kubectl access from a different machine with cluster-admin and a backup of the original DaemonSet/Deployment manifest.

The cluster-side RBAC binding omits write verbs on `apps/daemonsets` and `apps/deployments` in `kube-system` (and any namespace where the cluster's control plane runs).

---

## CustomResourceDefinition operations

**Refused operations**: `kubectl create / apply / delete` on any `apiextensions.k8s.io/v1.CustomResourceDefinition`, including Gateway API CRDs, Cilium CRDs, Istio CRDs, cert-manager CRDs.

**Why**: Deleting a CRD cascades-deletes every custom resource of that kind cluster-wide. Some CRDs (Cilium `CiliumIdentity`, Istio `WorkloadEntry`) carry runtime state that cannot be recreated from manifests. Installing a CRD at the wrong version creates a schema mismatch with running controllers, leading to admission failures on every subsequent apply of that kind.

**Instead**: CRD installs are performed by the upstream Helm chart or operator manifest at install time. Upgrades follow the operator's documented version-skew policy.

**Cluster-side blast radius if bypassed**: Deleting a CRD ends the corresponding feature; cascading deletion of CRs may delete production policy or routing config. The cluster-side RBAC binding omits `apiextensions.k8s.io` group entirely.

---

## Broad Secret operations

**Refused operations**: any `kubectl get / list` on `Secrets` outside the explicit allowlist of namespaces (`vanguard-system` for the agent's own SA, `kube-system` only for the specific ClusterMesh peer Secret name documented in `permitted-mutations.md`).

**Why**: Secrets carry credentials. A broad `kubectl get secrets --all-namespaces` exposes every cached ServiceAccount token, every ImagePullSecret, every TLS key in the cluster — to the agent's session, to its log, to whatever the harness does with response context.

**Instead**: Read only the specific Secret needed by name. The pre-flight self-check confirms `kubectl auth can-i get secrets --all-namespaces` returns `no` for the principal.

**Cluster-side blast radius if bypassed**: Credential leak; any cached token in any namespace becomes available to whoever has the agent's transcript.

---

## Cluster-admin equivalence

**Refused operations**: any operation when `kubectl auth can-i '*' '*' --all-namespaces` returns `yes` for the operator's principal or for the agent's bound ServiceAccount.

**Why**: `cluster-admin` and `system:masters` group membership bypass all RBAC checks. Every other rule in this document is enforced by RBAC; if RBAC is bypassed, the prompt is the only remaining defense, and prompt rules are advisory.

**Instead**: Operators must use a low-privileged kubeconfig with `impersonate` rights on the agent's ServiceAccount. See `docs/least-privilege-rbac.md` for the canonical pattern.

**Cluster-side blast radius if bypassed**: Every other refusal in this document is bypassable.

---

## Refusal response format

```
REFUSED — <rule-section-header-from-this-document>

Reason: <one-sentence explanation grounded in this document>
What you can do instead: <pointer to permitted-mutations.md or to architecture-review-agent for cutover plan>
RBAC enforcement: <whether the cluster-side binding also denies this verb (yes / no / depends on operator's principal)>
```

No retry. No "well actually". No partial execution. The refusal is the response.
