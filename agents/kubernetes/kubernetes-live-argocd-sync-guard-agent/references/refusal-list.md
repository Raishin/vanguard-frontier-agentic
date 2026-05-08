# Hard refusal list — Kubernetes Live Argo CD Sync Guard

This document is the explicit `REFUSE` list for Kubernetes Live Argo CD Sync Guard. It combines:

1. **Universal one-way doors** that every live-guard refuses (defined in `docs/least-privilege-rbac.md`).
2. **Domain-specific destructive operations** for Kubernetes Live Argo CD Sync Guard.

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

## Domain-specific HARD REFUSE list (Kubernetes Live Argo CD Sync Guard)


## Trigger argocd app sync to production without sync-window check

**Why**: Argo CD sync windows define safe deploy windows. Forcing sync outside the window (`argocd app sync --force`) bypasses the window and may deploy to production during a known-bad change window (e.g., end-of-quarter freeze, Friday-night freeze).

**Instead**: Read `argocd appproject get <project> -o yaml` for `syncWindows`; confirm current time is within an `allow` window or outside any `deny` window. Refuse if outside.

**Blast radius if bypassed**: Production deploy during a freeze window; rollback may itself be subject to the freeze, leaving the cluster stuck in a half-deployed state.

---

## Delete a sync window mid-deployment

**Why**: Sync windows are evaluated by the Argo CD application controller continuously. Deleting one mid-deploy can change which Applications are eligible to sync; subsequent autoSync attempts may fire unexpectedly.

**Instead**: Sync window changes are AppProject scope. Modify them via the AppProject manifest, not by deletion. This guard refuses deletion of sync windows.

**Blast radius if bypassed**: Unexpected production sync triggered by autoSync after the window is removed.

---

## Expand AppProject sourceRepos / destinations / clusterResourceWhitelist

**Why**: AppProject limits which Git repos can source manifests, which destinations can receive them, and which cluster-scoped resources can be created. Expanding these is a privilege expansion: an Application in the project can now sync from a new repo or create a new resource kind. Without review, an attacker-controlled repo can be added.

**Instead**: AppProject changes go through GitOps (the cluster's own repo) with a code-review approval. This guard refuses ad-hoc kubectl patches that expand an AppProject's surface.

**Blast radius if bypassed**: Privilege expansion for every Application in the project; supply-chain attack vector if a malicious repo is added.

---

## Enable auto-sync on a production Application

**Why**: `syncPolicy.automated.prune: true` makes Argo CD reconcile Git changes automatically AND prune deleted resources. A Git revert that removes a critical Service silently deletes the Service from the cluster. AutoSync on production needs careful syncOptions and `selfHeal` configuration.

**Instead**: Production AutoSync is enabled at AppProject level only with explicit `syncWindows` and CISO sign-off. This guard refuses to enable AutoSync on a prod Application via patch.

**Blast radius if bypassed**: Git mistakes propagate to production immediately; pruning deletes resources that the cluster operator may have created out-of-band.

---

## Delete an Application with PrunePropagationPolicy: foreground

**Why**: Deleting an Argo CD Application with default propagation cascades to delete every deployed resource — Deployments, Services, ConfigMaps, Secrets, even PVCs. PVCs may finalize their underlying PVs (data loss).

**Instead**: Delete the Application with `--cascade=orphan` if the goal is to stop reconciling without deleting deployed resources. Confirm with operator before any cascade-delete on production.

**Blast radius if bypassed**: Whatever the Application deployed is deleted; data loss possible if PVCs are deployed and finalize.

---

## ApplicationSet with a cluster generator targeting unmanaged clusters

**Why**: ApplicationSet's cluster generator iterates over `argocd cluster list`. Adding a new generator without restricting via labels can flood every connected cluster with the templated Application — including dev clusters, sandbox clusters, or partner clusters not authorized for this workload.

**Instead**: Cluster generators MUST include a `selector.matchLabels` constraint. This guard refuses ApplicationSets without a label selector on cluster generators.

**Blast radius if bypassed**: Workload deployed to clusters not authorized to host it; potential data leak if the workload includes secrets or accesses external services.

---


---

## Refusal response format

```
REFUSED — <rule-section-header-from-this-document>

Reason: <one-sentence explanation grounded in this document>
What you can do instead: <pointer to argocd-gitops-review-agent for review-only analysis, or to platform-team-led procedure>
RBAC enforcement: <whether the cluster-side binding also denies this verb (yes / no / depends on operator's principal)>
```

No retry. No "well actually". No partial execution. The refusal is the response.
