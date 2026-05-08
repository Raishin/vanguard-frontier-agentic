# Hard refusal list — Kubernetes Live Velero Restore Guard

This document is the explicit `REFUSE` list for Kubernetes Live Velero Restore Guard. It combines:

1. **Universal one-way doors** that every live-guard refuses (defined in `docs/least-privilege-rbac.md`).
2. **Domain-specific destructive operations** for Kubernetes Live Velero Restore Guard.

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

## Domain-specific HARD REFUSE list (Kubernetes Live Velero Restore Guard)


## Restore overwrites a running production namespace

**Why**: By default, Velero restores skip resources that already exist. With `existingResourcePolicy: update`, the restore overwrites running resources with the backup's state — undoing all changes since the backup. Configurations applied since the backup are lost.

**Instead**: Restore to a NEW namespace using `--namespace-mappings <src>:<src>-restore`, then promote after verification. Never restore-overwrite to a running namespace without explicit acknowledgment.

**Blast radius if bypassed**: Loss of every change made since the backup — config, secrets, persistent state for resources covered by the backup.

---

## Restore with includeClusterResources: true to production

**Why**: Cluster-scoped resources (CRDs, ClusterRoles, StorageClasses, PriorityClasses) restored from backup may conflict with the cluster's current state — e.g. restoring an old CRD version that the controller no longer supports, restoring a default StorageClass that has changed parameters.

**Instead**: Cluster-resource restores are platform-team operations, not workload restores. This guard refuses cluster-resource restores without explicit platform-team sign-off.

**Blast radius if bypassed**: Controllers crash on schema mismatch; admission policies break; storage provisioning may fail.

---

## Delete a production backup

**Why**: Backups are the rollback baseline. Deleting one removes a rollback option for whatever the backup covered. If the backup was the only one covering a particular point in time, the deletion is irreversible.

**Instead**: Backup retention is governed by `BackupStorageLocation.spec.objectStorage` lifecycle and Schedule TTL. Manual deletion is rare; require explicit retention-policy reasoning and platform-team sign-off.

**Blast radius if bypassed**: Loss of rollback option for the backup's covered scope.

---

## Restore to wrong cluster (mismatched cluster ID)

**Why**: Velero backups carry cluster-specific resource references (Service ClusterIPs, PV bindings to specific node hostnames in some configurations). Restoring backup-from-cluster-A to cluster-B without `--namespace-mappings` and resource-scoping leads to unpredictable state.

**Instead**: Cross-cluster restore is a deliberate DR operation with explicit cluster-ID confirmation and resource-mapping plan. This guard verifies the target cluster's identity matches the backup's `metadata.labels[velero.io/cluster-name]` before proceeding.

**Blast radius if bypassed**: PV bindings may attach to the wrong node; Services may collide on ClusterIP; CRDs may clash.

---

## Restore Backup without checking expiry or integrity

**Why**: Backups have a TTL (`backup.spec.ttl`); after expiry, the underlying object-storage data may be partially removed but the Backup CR may persist briefly. Restoring an expired Backup may produce a partial restore. Backup integrity (checksum verification) is not automatic.

**Instead**: Pre-restore: confirm `Backup.status.phase == Completed`, `Backup.status.expiration` is in the future, and run `velero backup describe <name>` for warnings. Refuse on any non-Completed phase.

**Blast radius if bypassed**: Partial restore — some PVs missing, some ConfigMaps missing — leaves cluster in a state worse than no-restore.

---

## Restore a Backup containing an old version of a CRD

**Why**: If the backup was taken from an earlier Kubernetes / operator version, restoring its CRD definitions over the current ones can downgrade CRD schemas. Existing custom resources of newer schema may fail validation; controllers may stop reconciling.

**Instead**: Cluster-scoped resources are platform-team territory. Workload restores should set `includeClusterResources: false`.

**Blast radius if bypassed**: CRD schema downgrade; widespread admission failures on existing resources of newer schema.

---

## Velero restore that re-introduces deleted security policy

**Why**: If a NetworkPolicy or AuthorizationPolicy was intentionally deleted in production (because it was buggy or out-of-date), a restore can silently re-introduce it. The cluster is back in the bad state without warning.

**Instead**: Pre-restore: review what resources the backup contains; explicitly opt out of restoring policy resources known to have changed. Use `--exclude-resources` flag.

**Blast radius if bypassed**: Re-application of intentionally-removed policy; unpredictable application impact.

---


---

## Refusal response format

```
REFUSED — <rule-section-header-from-this-document>

Reason: <one-sentence explanation grounded in this document>
What you can do instead: <pointer to velero-backup-restore-guard-agent for review-only analysis, or to platform-team-led procedure>
RBAC enforcement: <whether the cluster-side binding also denies this verb (yes / no / depends on operator's principal)>
```

No retry. No "well actually". No partial execution. The refusal is the response.
