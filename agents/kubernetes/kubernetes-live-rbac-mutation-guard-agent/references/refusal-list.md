# Hard refusal list — Kubernetes Live RBAC Mutation Guard

This document is the explicit `REFUSE` list for Kubernetes Live RBAC Mutation Guard. It combines:

1. **Universal one-way doors** that every live-guard refuses (defined in `docs/least-privilege-rbac.md`).
2. **Domain-specific destructive operations** for Kubernetes Live RBAC Mutation Guard.

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

## Domain-specific HARD REFUSE list (Kubernetes Live RBAC Mutation Guard)


## Bind a non-infrastructure ServiceAccount to cluster-admin

**Why**: ClusterRoleBinding to `cluster-admin` grants every verb on every resource. The bound SA can now perform every other destructive operation on this list.

**Instead**: Define a narrowly-scoped ClusterRole with the minimum verbs the workload needs. Per upstream RBAC good practices: `Avoid wildcard permissions, especially to all resources`.

**Blast radius if bypassed**: Cluster-admin equivalence for the bound SA.

---

## Grant escalate verb on roles

**Why**: Per upstream `kubernetes.io/docs/concepts/security/rbac-good-practices`: *granting users the `escalate` right allows them to bypass Kubernetes' built-in protections against privilege escalation*. A user with `escalate` on `clusterroles` can update any ClusterRole to include verbs they don't currently hold.

**Instead**: The platform team uses a documented escalation procedure (e.g. break-glass account) instead of granting `escalate`. This guard refuses creation of any Role/ClusterRole with `escalate` verb without explicit CISO sign-off.

**Blast radius if bypassed**: Self-bootstrap to cluster-admin in two API calls.

---

## Grant bind verb on roles

**Why**: Per upstream RBAC good practices: *granting users the `bind` right allows them to bypass Kubernetes' built-in protections against privilege escalation*. A user with `bind` on `clusterroles` can create bindings to roles with rights they don't already possess.

**Instead**: Same as `escalate` — break-glass procedure, not RBAC grant. Refuse without CISO sign-off.

**Blast radius if bypassed**: Self-bootstrap to any existing privileged role.

---

## Grant impersonate verb on users / groups / serviceaccounts

**Why**: The `impersonate` verb on `users` lets the principal request operations as any user, including `system:admin`. On `groups` it includes `system:masters` (the RBAC-bypass group). On `serviceaccounts` it lets the principal act as any SA cluster-wide.

**Instead**: Impersonation rights are reserved for low-privileged operator accounts that pre-flight the mutation guards (per `docs/least-privilege-rbac.md`) — explicitly NOT for routine workloads. Never grant impersonate without operator-account justification.

**Blast radius if bypassed**: Impersonation as `system:masters` group bypasses RBAC entirely (per upstream).

---

## Wildcard verb (`*`) or wildcard resource (`*`) in any rule

**Why**: Per upstream RBAC good practices verbatim: *Avoid wildcard permissions, especially to all resources, as this grants access to current and future object types.* New API kinds added in future Kubernetes releases are auto-granted.

**Instead**: Enumerate verbs and resources explicitly. The list of verbs on each resource is finite and well-documented.

**Blast radius if bypassed**: Future-proof privilege escalation; new APIs grant new powers automatically.

---

## Add a subject in `system:masters` group

**Why**: Per upstream: *Do not add users to the `system:masters` group, as this bypasses all RBAC checks and grants unrestricted superuser access.* Subjects in this group are NEVER subject to RBAC authorization — every verb is allowed regardless of binding.

**Instead**: There is no legitimate use case for adding a non-bootstrap user to `system:masters`. This guard HARD REFUSES any binding with this group.

**Blast radius if bypassed**: Permanent, irrevocable cluster-admin equivalence — the only fix is rotating the cluster CA.

---

## Bind to the `default` ServiceAccount in any namespace

**Why**: The `default` SA is shared by every Pod in the namespace that doesn't specify its own SA. A binding to it grants the role to every such Pod — typically every Pod in the namespace at first.

**Instead**: Create a dedicated ServiceAccount for the workload; bind only that SA. Refuse bindings to `default`.

**Blast radius if bypassed**: Shared blast radius; future Pods in the namespace inherit the binding silently.

---

## Delete a ClusterRoleBinding without confirming dependent workloads

**Why**: RBAC has no built-in revocation grace period or dependency tracking. Deleting a binding mid-workload-flight causes API calls to start returning `forbidden` immediately. Workloads that retry indefinitely consume resources; workloads that crash on auth failure cycle.

**Instead**: Identify dependent workloads via `kubectl get rolebindings.rbac.authorization.k8s.io -A -o json | jq '.items[] | select(.roleRef.name=="<role>")'`; coordinate workload migration before deletion.

**Blast radius if bypassed**: Authorization denial cascade; cached SA tokens may keep working briefly until the API server's authorization cache expires.

---


---

## Refusal response format

```
REFUSED — <rule-section-header-from-this-document>

Reason: <one-sentence explanation grounded in this document>
What you can do instead: <pointer to kubernetes-rbac-review-agent for review-only analysis, or to platform-team-led procedure>
RBAC enforcement: <whether the cluster-side binding also denies this verb (yes / no / depends on operator's principal)>
```

No retry. No "well actually". No partial execution. The refusal is the response.
