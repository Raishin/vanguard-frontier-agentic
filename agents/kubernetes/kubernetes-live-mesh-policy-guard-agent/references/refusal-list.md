# Hard refusal list — Kubernetes Live Mesh Policy Guard

This document is the explicit `REFUSE` list for Kubernetes Live Mesh Policy Guard. It combines:

1. **Universal one-way doors** that every live-guard refuses (defined in `docs/least-privilege-rbac.md`).
2. **Domain-specific destructive operations** for Kubernetes Live Mesh Policy Guard.

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

## Domain-specific HARD REFUSE list (Kubernetes Live Mesh Policy Guard)


## Delete or modify STRICT PeerAuthentication without mTLS migration plan

**Why**: STRICT PeerAuthentication enforces mTLS for all incoming connections to the namespace's workloads. Deleting it (or changing to PERMISSIVE) lets unencrypted traffic through. Mid-migration to ambient mesh, this can occur if waypoint enrollment is incomplete and the operator panics.

**Instead**: Migration from STRICT to PERMISSIVE goes namespace-by-namespace with traffic verification at each step. Capture the policy YAML; propose the change as a diff; require platform-team sign-off. This guard refuses on STRICT→PERMISSIVE without an explicit migration plan.

**Blast radius if bypassed**: Plaintext traffic in/out of the affected namespace; mTLS-derived AuthorizationPolicy decisions become trivially spoofable.

---

## Apply L7 AuthorizationPolicy in ambient mode without waypoint enrolled

**Why**: Ambient mesh L7 enforcement requires a waypoint Pod for the targeted namespace or service account. An AuthorizationPolicy with L7 rules (path, method, headers) applied without an enrolled waypoint silently degrades to L4 enforcement only — the L7 rules are ignored. Operators believe they have L7 controls; they don't.

**Instead**: Verify `kubectl get gateway -n <ns> -l istio.io/gateway-name=waypoint` returns a Programmed waypoint before applying any L7 AuthorizationPolicy. If the waypoint is absent or not Ready, refuse the apply.

**Blast radius if bypassed**: Silent L7 enforcement bypass. Compliance posture is fictional.

---

## Delete a DENY AuthorizationPolicy

**Why**: DENY policies block specific traffic patterns. Deleting them removes the block; the underlying ALLOW policies (or default-allow if no ALLOW exists) immediately apply.

**Instead**: DENY policies are typically deleted only when the threat they address is resolved (e.g. patching a CVE that the policy worked around). Confirm with the platform team that the underlying threat is gone before deletion.

**Blast radius if bypassed**: Whatever traffic the DENY blocked is now allowed.

---

## Change waypoint enrollment label without traffic analysis

**Why**: Adding or removing the `istio.io/use-waypoint` label on a namespace or pod causes traffic to be (re)routed through the waypoint. During the transition, in-flight connections may break; established mTLS sessions don't survive the rerouting.

**Instead**: Plan waypoint enrollment changes during a maintenance window with explicit connection-drain expectations. Verify the waypoint's Programmed status before applying the label.

**Blast radius if bypassed**: Brief connection failures cluster-wide for traffic to the affected namespace; some clients see retries, others see hard failures.

---

## Modify Gateway or VirtualService that anchors a Gateway API listener

**Why**: Mixed Gateway API + Istio Gateway environments are common during migration. Modifying an Istio Gateway resource may overlap with a Gateway API Gateway listener on the same (port, protocol, host), causing one to silently win and the other to receive no traffic.

**Instead**: Architecture review owns Gateway API migration plans. Mesh policy guard does not modify gateway resources during migration windows.

**Blast radius if bypassed**: Silent traffic blackhole on one of the overlapping listeners.

---


---

## Refusal response format

```
REFUSED — <rule-section-header-from-this-document>

Reason: <one-sentence explanation grounded in this document>
What you can do instead: <pointer to istio-ambient-mesh-review-agent for review-only analysis, or to platform-team-led procedure>
RBAC enforcement: <whether the cluster-side binding also denies this verb (yes / no / depends on operator's principal)>
```

No retry. No "well actually". No partial execution. The refusal is the response.
