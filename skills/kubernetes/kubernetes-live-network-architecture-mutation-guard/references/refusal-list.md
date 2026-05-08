# Hard refusal list — one-way doors

This document is the explicit `REFUSE` list. The agent must respond with `REFUSED — <rule>` and no execution attempt for any of the following. The cluster-side RBAC binding shipped with this skill also denies these verbs at the API server level — defense in depth.

> **Scope-of-defense clarification.** This list is the **prompt-level fast-path** for rejecting common destructive operations. The authoritative defense is the cluster-side RBAC binding (`references/least-privilege-rbac.yaml`), which is **deny-by-default**: it grants only the enumerated verbs / resources and denies everything else. New attack vectors (Kubernetes adds APIs every release) may not appear in this list immediately, but the binding rejects them automatically because they are not on the allow list. If you find a destructive operation not in this list, that does **not** mean the agent will execute it — but please open an issue so the prompt-level rejection is added.

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

## Node operations

**Refused operations**: `kubectl delete node <name>`, `kubectl drain <node>`, `kubectl cordon <node>`, `kubectl uncordon <node>`, any `patch` on `nodes/spec.unschedulable`, any `patch` on `nodes/spec.taints` that affects scheduling.

**Why**: Node deletion drops the node's Pod CIDR allocation; pods on the node lose IPs and any PV with `volumeAttachment` to that node may leak its underlying volume. Draining all nodes (or even half) is a cluster-wide outage. Cordoning a node redirects Pod placement, which silently shifts traffic patterns and may overload the remaining nodes' egress paths (NAT GW, conntrack, NodeLocal DNSCache).

**Instead**: Node lifecycle is the cluster-administrator's responsibility, exercised through the cluster's node-pool management (cloud-provider controlled or kubeadm-managed). Use the architecture-review agent to surface node-imbalance findings; let humans drain.

**Cluster-side blast radius if bypassed**: Per upstream `kubernetes.io/docs/setup/production-environment/tools/kubeadm/create-cluster-kubeadm`, `kubectl delete node` removes the Node object after drain — but if executed without prior drain, every Pod on the node becomes Pending forever (no IP allocation source). `kubectl drain` with `--ignore-daemonsets --force --delete-emptydir-data` mass-evicts; if scripted across all nodes, every workload becomes unscheduled simultaneously.

The cluster-side RBAC binding for this guard omits write verbs (`patch`, `update`, `delete`) on `apiGroups: [""], resources: ["nodes"]`. The eviction subresource (`pods/eviction`) is also omitted.

---

## Admission webhook configurations

**Refused operations**: any `create` / `patch` / `update` / `delete` on `admissionregistration.k8s.io/v1` `MutatingWebhookConfiguration` or `ValidatingWebhookConfiguration` resources.

**Why**: Admission webhooks are the cluster's enforcement boundary. A malicious or misconfigured `MutatingWebhookConfiguration` can:
- Drop or rewrite arbitrary Service / Deployment specs cluster-wide
- Inject malicious sidecars into every new Pod
- Cause infinite admission-loop and brick the cluster (each create triggers a mutation that triggers another mutation)

A deletion of an existing webhook config silently removes whatever defense it provided — Kyverno policies stop enforcing, cert-manager stops injecting, sidecar mesh injection stops, sealed-secrets stops decoding. Per the upstream `DELETE /apis/admissionregistration.k8s.io/v1/mutatingwebhookconfigurations` API, the operation cascades-deletes per-namespace bindings.

**Instead**: Admission webhook lifecycle is the platform-team's responsibility, governed by the operator's Helm chart (Kyverno, cert-manager, etc.) or a dedicated GitOps pipeline. Modifying webhooks is never a network-architecture concern.

**Cluster-side blast radius if bypassed**: An attacker-controlled `MutatingWebhookConfiguration` with `failurePolicy: Ignore` and a `clientConfig.url` pointing at an attacker endpoint silently observes (and optionally mutates) every API request to selected resources. With `failurePolicy: Fail`, it can wedge the cluster.

The cluster-side RBAC binding omits `apiGroups: ["admissionregistration.k8s.io"]` entirely.

---

## APIService aggregation

**Refused operations**: any `create` / `patch` / `update` / `delete` on `apiregistration.k8s.io/v1` `APIService` resources.

**Why**: Registered `APIService`s aggregate API groups into the kube-apiserver. Modifying or deleting an APIService can:
- Hijack API calls for an entire group (e.g. `metrics.k8s.io`, `external.metrics.k8s.io`, `custom.metrics.k8s.io`) by pointing them at an attacker-controlled service
- Break HPA / VPA cluster-wide by removing the metrics service registration
- Break aggregated extension APIs like Knative, Service-Catalog, the Cilium ClusterMesh apiserver

**Instead**: APIService lifecycle is owned by the controller's Helm chart at install time. Never modified ad-hoc.

**Cluster-side blast radius if bypassed**: Hijacking `metrics.k8s.io` lets an attacker feed false metrics to HPA, triggering arbitrary scale events. Hijacking a custom-metrics APIService used by an autoscaler means the autoscaler routes through the attacker.

The cluster-side RBAC binding omits `apiGroups: ["apiregistration.k8s.io"]` entirely.

---

## Finalizer manipulation

**Refused operations**: any `patch` / `update` that removes finalizers from a resource — particularly `Namespaces`, `PersistentVolumes`, `CustomResourceDefinitions`, or any controller-managed CR with finalizers.

The pattern looks innocuous: `kubectl patch ns kube-system --type=merge -p '{"metadata":{"finalizers":[]}}'`. The agent must recognize this as a finalizer-stripping operation, not a routine metadata edit.

**Why**: Finalizers are the controller pattern that prevents premature deletion. A `Namespace` finalizer (`kubernetes`) blocks the delete until all child resources are reaped; stripping it bypasses cleanup. A `PersistentVolume` finalizer (`kubernetes.io/pv-protection`) prevents PV deletion while a PVC still references it; stripping it leaves the PVC orphaned. CRD finalizers (`customresourcecleanup.apiextensions.k8s.io`) block CRD deletion while custom resources of that kind still exist; stripping bypasses the cascade-delete safety.

**Instead**: Wait for the controller to drain the resource. If the controller is stuck, restart the controller or fix its underlying issue. Finalizer manipulation is a last-resort emergency operation, never a routine patch.

**Cluster-side blast radius if bypassed**: A namespace deleted with stripped finalizers leaves orphan PVs, lingering CRs, and abandoned controller state. PVs deleted with stripped finalizers leak underlying cloud volumes. CRDs deleted with stripped finalizers cascade-delete every CR of that kind without controller cleanup.

The cluster-side RBAC binding's `patch` verb on Services and the resourceName-locked CoreDNS ConfigMap is field-narrow enough that finalizer paths are not a primary attack surface here, but the agent must still refuse on principle. For a defense-in-depth admission policy, a Kyverno rule denying `patch` on `metadata.finalizers` for any non-platform principal is recommended.

---

## Pod and node subresources (privilege escalation paths)

**Refused operations**:
- `pods/exec` — `kubectl exec` into any Pod, especially in `kube-system`, `cilium`, `istio-system`, `linkerd`
- `pods/portforward` — `kubectl port-forward` to any Pod
- `pods/proxy` — `kubectl proxy --pod` or direct API proxy
- `pods/binding` — manual binding of a Pod to a Node, bypassing the scheduler
- `nodes/proxy` — direct kubelet API access via the apiserver proxy

**Why**:
- `pods/exec` into a `kube-system` pod gives shell access in the cluster's control-plane namespace, with the pod's mounted ServiceAccount token (often broadly privileged for control-plane components).
- `pods/portforward` bypasses NetworkPolicy entirely — the operator's local machine becomes a Pod-network-addressable peer.
- `pods/proxy` and `nodes/proxy` route requests through the API server to the kubelet, which has its own authorization model that may differ from the API server's RBAC.
- `pods/binding` lets the caller place a Pod on any Node regardless of node selector, taints, or affinity rules — bypass scheduler entirely.

**Instead**: For debugging, use `kubectl debug` with an ephemeral container and `--profile=netadmin` (or `--profile=restricted` for unprivileged debugging). For Service introspection, use a sanctioned debug pod the operator has stood up. Never `pods/exec` from an automated agent.

**Cluster-side blast radius if bypassed**: `pods/exec` into the cilium-agent pod is functionally cluster-admin equivalence — the pod has CAP_NET_ADMIN, host network, and the cilium-agent ServiceAccount token. `pods/portforward` can bridge attacker traffic into the Pod network. `pods/binding` defeats every scheduling guarantee the cluster makes.

The cluster-side RBAC binding omits all of `pods/exec`, `pods/portforward`, `pods/proxy`, `pods/binding`, `nodes/proxy`, and `pods/eviction`.

---

## CSR approval and TokenRequest minting

**Refused operations**:
- `update` on `certificates.k8s.io/v1` `CertificateSigningRequest` `approval` subresource (CSR approval)
- `create` on `certificates.k8s.io/v1` `CertificateSigningRequest` resources with a `subject` containing `O=system:masters` or any `system:` group
- `create` on `serviceaccounts/token` (TokenRequest API) for any ServiceAccount outside the agent's own.

**Why**:
- A CSR approved with subject `O=system:masters, CN=...` produces a client certificate that authenticates as a member of `system:masters` — RBAC bypassed entirely. This is the Kubernetes equivalent of issuing a root token.
- `TokenRequest` mints a fresh time-bounded token for any ServiceAccount the caller has `create` permission on the token subresource for. If the bound principal can call `tokenrequests` on `system:serviceaccount:kube-system:cilium`, it becomes the cilium-agent for the token's lifetime.

**Instead**: CSR approval is a cluster-admin operation. ServiceAccount tokens are minted at pod creation by the kubelet via `TokenRequestProjection`, never ad-hoc.

**Cluster-side blast radius if bypassed**: Approving a `system:masters` CSR is a permanent cluster-takeover — the cert is valid until expiry and not revocable without rotating the cluster CA. Minting a token for a privileged SA gives time-bounded but full access to whatever that SA can do.

The cluster-side RBAC binding omits `apiGroups: ["certificates.k8s.io"]` entirely and omits `serviceaccounts/token` create.

---

## Manual Endpoints / EndpointSlices writes

**Refused operations**: any `create` / `patch` / `update` / `delete` on `v1.Endpoints` or `discovery.k8s.io/v1.EndpointSlice` resources outside of the controller-managed lifecycle.

**Why**: `EndpointSlice` is owned by the EndpointSlice controller, which derives slices from `Service` selectors and Pod readiness. Manual writes race with the controller — the controller will eventually reconcile and overwrite the manual change. But during the window between manual write and reconciliation (tens of seconds to minutes), Service traffic routes to whatever IPs the manual write specified. An attacker-controlled IP in an EndpointSlice for a sensitive Service gets traffic for that window.

`v1.Endpoints` (the older, controller-managed API) has the same race-condition concern.

**Instead**: To redirect Service traffic, change the Service's selector or the backend Pods' labels. Use ExternalName Services for cross-cluster targets. Never manual EndpointSlice writes.

**Cluster-side blast radius if bypassed**: Transient man-in-the-middle of any selected Service. Even after reconciliation, log analysis may show the malicious window if not searched for explicitly.

The cluster-side RBAC binding grants only `get`/`list`/`watch` on `endpoints` and `endpointslices` — never write.

---

## kube-system ConfigMap writes outside the resourceName-locked allowlist

**Refused operations**: any `create` / `patch` / `update` / `delete` on ConfigMaps in `kube-system` (or any namespace where the cluster's control plane runs) **other than** the explicitly resourceName-locked `ConfigMap/coredns` permitted in `permitted-mutations.md`.

This includes:
- `ConfigMap/cilium-config` — Cilium agent runtime configuration; changing it may switch endpoint routing modes or disable encryption
- `ConfigMap/kube-proxy` — kube-proxy mode and behavior
- `ConfigMap/kubelet-config` — kubelet flags (applied on next kubelet restart)
- `ConfigMap/cluster-info` (in `kube-public`) — cluster bootstrapping reference
- Any `ConfigMap` with the label `addonmanager.kubernetes.io/mode: Reconcile` — addon-manager-owned

**Why**: These ConfigMaps drive the behavior of the cluster's control-plane components. Modifying them is functionally equivalent to a control-plane reconfiguration. The CoreDNS ConfigMap is the one exception, and only because we have a tight reload-and-verify protocol for it — `permitted-mutations.md` documents that protocol explicitly.

**Instead**: Control-plane reconfiguration goes through the cluster's bootstrapping tooling (kubeadm, the cloud-provider's managed control plane, or the operator's own CRD-driven config). Never ad-hoc.

**Cluster-side blast radius if bypassed**: Patching `cilium-config` to disable encryption silently removes pod-to-pod mTLS. Patching `kube-proxy` ConfigMap and triggering a DaemonSet restart cycles every node's kube-proxy with the new config — this is the kube-proxy mode swap one-way door (above) by another path.

The cluster-side RBAC binding restricts `patch` on `configmaps` to the literal `resourceNames: ["coredns"]`. All other `kube-system` ConfigMaps are denied at the API server level.

---

## PriorityClass and IngressClass cluster-scoped resources

**Refused operations**:
- any `delete` / `update` / `patch` on `scheduling.k8s.io/v1.PriorityClass` resources, especially `system-cluster-critical` and `system-node-critical`
- any `delete` / `update` / `patch` on `networking.k8s.io/v1.IngressClass` resources

**Why**:
- Per upstream `kubernetes.io/docs/reference/kubernetes-api/workload-resources/priority-class-v1`, deleting a PriorityClass is supported but the `system-cluster-critical` and `system-node-critical` classes are critical to control-plane eviction order. Removing them, or lowering their priority, can cause control-plane Pods to be evicted first under node pressure.
- Per upstream `kubernetes.io/docs/reference/kubernetes-api`, `IngressClass` is the binding between an Ingress controller and the Ingress resources it claims. Deleting it leaves every Ingress in that class without a controller — traffic stops.

**Instead**: PriorityClass and IngressClass are install-time concerns of the operator's chart. Never modified ad-hoc.

**Cluster-side blast radius if bypassed**: `system-cluster-critical` deletion: kube-apiserver, kube-scheduler, etcd, control-plane addons can be evicted under pressure, escalating a node-level resource problem to a cluster-level outage. `IngressClass` deletion: every external HTTP route in the cluster fails until the IngressClass is restored and the controller's reconcile loop re-binds.

The cluster-side RBAC binding omits `apiGroups: ["scheduling.k8s.io"]` and `apiGroups: ["networking.k8s.io"], resources: ["ingressclasses"]` entirely (`ingresses` itself is read-only on this guard since Gateway API supersedes it).

---

## Lease objects in `kube-node-lease`

**Refused operations**: any `create` / `patch` / `update` / `delete` on `coordination.k8s.io/v1.Lease` objects in the `kube-node-lease` namespace.

**Why**: Each Node has a Lease object in `kube-node-lease` whose `spec.renewTime` is updated by the kubelet to signal liveness. The node-controller marks a Node as `NotReady` after a configurable grace period (default 40 seconds) without a renewed Lease. Manipulating these:
- Deleting a Node's Lease may not immediately mark it `NotReady` (the Node controller has its own grace) but can interfere with the kubelet's heartbeat path.
- Patching `spec.renewTime` to far-future makes a dead node appear alive, which makes Pod scheduling place workloads on a node that will never run them.
- Patching to far-past forces premature `NotReady`, triggering Pod eviction (if `node.kubernetes.io/not-ready` toleration is short).

**Instead**: Node liveness is the kubelet's responsibility. Never manual Lease edits.

**Cluster-side blast radius if bypassed**: False-Ready nodes accept Pod placement that never executes; false-NotReady nodes trigger workload churn.

The cluster-side RBAC binding omits `apiGroups: ["coordination.k8s.io"]` entirely.

---

## Refusal response format

```
REFUSED — <rule-section-header-from-this-document>

Reason: <one-sentence explanation grounded in this document>
What you can do instead: <pointer to permitted-mutations.md or to architecture-review-agent for cutover plan>
RBAC enforcement: <whether the cluster-side binding also denies this verb (yes / no / depends on operator's principal)>
```

No retry. No "well actually". No partial execution. The refusal is the response.
