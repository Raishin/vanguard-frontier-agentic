# Hard refusal list — Kubernetes Live Network Policy Guard

This document is the explicit `REFUSE` list for Kubernetes Live Network Policy Guard. It combines:

1. **Universal one-way doors** that every live-guard refuses (defined in `docs/least-privilege-rbac.md`).
2. **Domain-specific destructive operations** for Kubernetes Live Network Policy Guard.

> **Scope-of-defense clarification.** This list is the **prompt-level fast-path** for rejecting common destructive operations. The authoritative defense is the cluster-side RBAC binding (`references/least-privilege-rbac.yaml`), which is **deny-by-default**: it grants only the enumerated verbs / resources and denies everything else. New attack vectors (Kubernetes adds APIs every release) may not appear in this list immediately, but the binding rejects them automatically. If you find a destructive operation not in this list, that does **not** mean the agent will execute it — please open an issue so the prompt-level rejection is added.

The format for each entry: **what is refused**, **why it's a one-way door**, **what to do instead**, **cluster-side blast radius if the prompt-level refusal is bypassed**.

---

## Universal one-way doors (refused by every live-guard)

These apply across all live-guard agents in this repo. The cluster-side RBAC binding for this guard explicitly omits the verbs/resources for each of these:

- **Namespace deletion** (`kubectl delete ns <any>`) — kube-system / cilium / istio-system / argocd / velero deletion is cluster-fatal.
- **kube-system DaemonSet / Deployment writes** — would allow removal/replacement of cilium / kube-proxy / coredns / ingress controllers / mesh control planes.
- **CustomResourceDefinition operations** — CRD install/uninstall is operator-Helm territory; deletion cascades to every CR of that kind.
- **Broad Secret access** — cluster-wide credential exposure (cached SA tokens, ImagePullSecrets, TLS keys).
- **Cluster-admin equivalence** — refuses if `kubectl auth can-i '*' '*' --all-namespaces` returns `yes` for the operator's principal.
- **Node operations** — `kubectl delete node`, `drain`, `cordon`, `nodes/spec.taints` patch.
- **Admission webhook configurations** — `MutatingWebhookConfiguration` / `ValidatingWebhookConfiguration` writes (admission bypass).
- **APIService aggregation** — `apiregistration.k8s.io` writes (aggregation hijack).
- **Finalizer manipulation** — `metadata.finalizers` patches that bypass namespace / PV / CRD deletion protection.
- **Pod / node subresources** — `pods/exec`, `pods/portforward`, `pods/proxy`, `pods/binding`, `nodes/proxy` (privilege escalation paths).
- **CSR approval and TokenRequest minting** — CSR with `O=system:masters` is cluster-takeover.
- **Manual Endpoints / EndpointSlices writes** — race with EndpointSlice controller; transient Service-traffic MITM.
- **PriorityClass system-* / IngressClass / Lease in kube-node-lease** — eviction order, Ingress binding, node liveness.

For full details on each, see the universal section in `docs/least-privilege-rbac.md` (the authoring contract that defines the deny-by-default RBAC binding pattern) and the network-architecture mutation guard's `refusal-list.md` (the canonical reference implementation).

---

## Domain-specific HARD REFUSE list (Kubernetes Live Network Policy Guard)


## CiliumClusterwideNetworkPolicy default-deny removal or weakening

**Why**: Cluster-wide policies enforce identity-aware default-deny across all namespaces. Removing or weakening one opens cluster-wide unrestricted ingress/egress for matched workloads. Without snapshot, recovery requires re-deriving the policy from documentation.

**Instead**: Capture the existing policy YAML, propose the change as a diff, and require platform-team sign-off on the diff. ClusterwideNetworkPolicy writes are NOT in this guard's RBAC binding by default — operator opts in only when intentional.

**Blast radius if bypassed**: Pod-to-pod traffic that was previously denied flows freely; data-plane attackers gain lateral-movement capability cluster-wide.

---

## toCIDRSet expansion to include cloud metadata service

**Why**: Adding `0.0.0.0/0` or any range that includes `169.254.169.254/32` to a CiliumNetworkPolicy `toCIDRSet` allows pod egress to the cloud metadata service. This is the SSRF/credential-theft CVE class. Without `except: ["169.254.169.254/32"]`, every pod under the policy can mint instance IAM credentials.

**Instead**: Use IRSA / Workload Identity / Pod Identity for cloud credentials. If broad egress is genuinely required, always exclude 169.254.169.254/32, fd00:ec2::254/128 (AWS IPv6 IMDS), and metadata.google.internal range explicitly.

**Blast radius if bypassed**: Every pod under the policy can obtain the node's IAM role credentials. The cloud-side attacker has whatever the node role can do — typically broad.

---

## L7 policy applied without Envoy DaemonSet running

**Why**: Cilium L7 policy (rules under `toPorts.rules.http` etc.) requires the Cilium Envoy DaemonSet (or sidecar mode) to enforce. If Envoy isn't running, the L7 rule is silently ignored — the policy compiles, applies, but enforces only the L3/L4 portion. Operators believe they have L7 enforcement; they don't.

**Instead**: Verify `kubectl -n kube-system get ds cilium-envoy` (or `cilium config view | grep -i envoy`) before applying any L7 rule. If Envoy is absent, surface as an error and refuse to apply the policy.

**Blast radius if bypassed**: Silent L7 enforcement bypass. Compliance posture (e.g. PCI segmentation claims) is fictional until Envoy is present.

---

## Default-deny removal without immediate replacement

**Why**: Deleting a default-deny CiliumNetworkPolicy in a namespace transitions the namespace to default-allow. The window between delete and re-apply (even seconds) lets attacker traffic through.

**Instead**: Use `kubectl apply -f` with the new policy that REPLACES the default-deny in a single API call. Never delete-then-apply. If transitioning between policy revisions, the new one must be applied first; the old one deleted only after verification.

**Blast radius if bypassed**: Time-window default-allow on the affected namespace.

---

## policy-default-local-cluster flag flip in ClusterMesh

**Why**: The Cilium `policy-default-local-cluster` flag (introduced for ClusterMesh isolation) determines whether NetworkPolicies apply only to local-cluster traffic or to remote ClusterMesh peers as well. Flipping it changes how every existing policy is evaluated cluster-wide.

**Instead**: Treat as a one-way door requiring full ClusterMesh re-validation. Architecture review (`kubernetes-network-architecture-review-agent`) produces the migration plan; this guard refuses to flip the flag.

**Blast radius if bypassed**: Every NetworkPolicy's effective scope changes. Some flows that worked stop working; some flows that were blocked open up.

---


---

## Refusal response format

```
REFUSED — <rule-section-header-from-this-document>

Reason: <one-sentence explanation grounded in this document>
What you can do instead: <pointer to cilium-network-policy-review-agent for review-only analysis, or to platform-team-led procedure>
RBAC enforcement: <whether the cluster-side binding also denies this verb (yes / no / depends on operator's principal)>
```

No retry. No "well actually". No partial execution. The refusal is the response.
