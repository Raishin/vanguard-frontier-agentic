# Hard refusal list — Kubernetes Live Admission Policy Guard

This document is the explicit `REFUSE` list for Kubernetes Live Admission Policy Guard. It combines:

1. **Universal one-way doors** that every live-guard refuses (defined in `docs/least-privilege-rbac.md`).
2. **Domain-specific destructive operations** for Kubernetes Live Admission Policy Guard.

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

## Domain-specific HARD REFUSE list (Kubernetes Live Admission Policy Guard)


## Change ClusterPolicy.spec.failureAction from Enforce to Audit

**Why**: `failureAction: Enforce` blocks the admission request when the policy fails; `Audit` (formerly `audit`) only logs. Changing Enforce → Audit is a silent enforcement bypass — the policy still appears in `kubectl get cpol`, still produces reports, but stops blocking. Operators looking at the policy list believe enforcement is in place; it isn't.

**Instead**: If a policy is causing legitimate workload friction, fix the workload or carve a PolicyException with explicit scope and TTL. Never flip global Enforce → Audit as a debugging shortcut.

**Blast radius if bypassed**: Whatever the policy enforced (image registry restrictions, mandatory labels, security context constraints) silently stops being enforced cluster-wide.

---

## Delete a ClusterPolicy without replacement

**Why**: Deletion immediately removes admission enforcement for the policy's scope. New non-compliant resources can be created during the window before a replacement is applied.

**Instead**: Use `kubectl apply -f` with the replacement policy in the same operation. If transitioning between revisions, the new one is applied first; the old one is deleted only after verifying the replacement is in `Ready: True` state.

**Blast radius if bypassed**: Time-window admission gap on the policy's scope.

---

## Add overly broad PolicyException

**Why**: PolicyException whitelists specific resources from policy enforcement. A broad exception (cluster-wide selector, no resource-name match, no expiry) effectively disables the parent policy. Operators add these for emergency unblocking and forget to remove them.

**Instead**: Every PolicyException must be scoped (specific resource names, specific namespaces) and time-bounded (TTL). This guard refuses exceptions without both.

**Blast radius if bypassed**: Silent enforcement disable for whatever the exception covers.

---

## Delete ValidatingAdmissionPolicyBinding while VAP remains

**Why**: VAP (the policy CEL) is harmless without a binding. The binding is what activates enforcement. Deleting the binding silently disables a policy that still appears to exist.

**Instead**: Delete VAP and binding together. Surface the binding's resource selectors before deletion so the operator knows what scope they're disabling.

**Blast radius if bypassed**: Silent enforcement gap — the policy looks present but enforces nothing.

---

## Apply Kyverno mutate or generate rule without dry-run validation

**Why**: Mutate rules rewrite incoming admission requests; generate rules create child resources. Both have cluster-wide reach. A misconfigured mutate rule can prevent every Pod from being created (e.g., adding a non-existent imagePullSecret); a misconfigured generate rule can flood the cluster with unwanted resources.

**Instead**: Apply with `--dry-run=server` first; verify no admission failures across the policy's scope; then apply for real. For generate rules, observe `Generated` count for 5 minutes after apply.

**Blast radius if bypassed**: Mutate misconfig: every admission of the matched kind fails. Generate misconfig: cluster fills with resources until ResourceQuota stops it.

---

## Modify a ClusterPolicy that protects against `delete namespaces` or `delete crds`

**Why**: If the cluster's defense-in-depth includes a Kyverno ClusterPolicy that denies cluster-destabilizing operations (`delete namespaces` for non-platform principals, `patch metadata.finalizers`), modifying this policy weakens layer L4 of the defense model documented in `docs/least-privilege-rbac.md`.

**Instead**: Defense-in-depth ClusterPolicies are owned by the platform team and the install pipeline. This guard refuses modifications.

**Blast radius if bypassed**: Removes the L4 admission-control layer; only RBAC and audit remain.

---


---

## Refusal response format

```
REFUSED — <rule-section-header-from-this-document>

Reason: <one-sentence explanation grounded in this document>
What you can do instead: <pointer to kyverno-policy-review-agent for review-only analysis, or to platform-team-led procedure>
RBAC enforcement: <whether the cluster-side binding also denies this verb (yes / no / depends on operator's principal)>
```

No retry. No "well actually". No partial execution. The refusal is the response.
