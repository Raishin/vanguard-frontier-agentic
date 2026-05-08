# Rollback patterns

Every permitted mutation has a documented rollback verb. The agent must surface the rollback verb in the response **before** executing the mutation, not after.

The default rollback strategy across this guard is **`kubectl apply -f <baseline.yaml>`**, never `kubectl delete`. Apply re-establishes the prior state; delete removes the resource entirely (and may cascade-delete children).

---

## Service spec patches

| Mutation | Rollback verb |
|---|---|
| `patch internalTrafficPolicy` | `kubectl apply -f /tmp/svc-<name>.before.yaml` |
| `patch externalTrafficPolicy` | `kubectl apply -f /tmp/svc-<name>.before.yaml` |
| `annotate topology-mode=Auto` | `kubectl annotate svc <name> -n <ns> service.kubernetes.io/topology-mode-` |
| `patch trafficDistribution` | `kubectl patch svc <name> -n <ns> --type=json -p='[{"op":"remove","path":"/spec/trafficDistribution"}]'` |

Post-rollback verification: `kubectl get endpointslice -n <ns> -l kubernetes.io/service-name=<name>` should show populated endpoints; `kubectl get svc <name> -n <ns> -o jsonpath='{.spec}'` should match `/tmp/svc-<name>.before.yaml`.

---

## CoreDNS Corefile patch

```bash
# Rollback
kubectl apply -f /tmp/coredns.before.yaml

# Force reload by deleting the oldest pod (rolling restart-equivalent;
# the `reload` plugin will pick up the restored Corefile within 30s, but
# evicting one pod accelerates recovery)
kubectl -n kube-system delete pod -l k8s-app=kube-dns --field-selector status.phase=Running --limit=1

# Verify
kubectl -n kube-system logs -l k8s-app=kube-dns --tail=50 --since=2m | grep -i "reload"
kubectl -n kube-system get pods -l k8s-app=kube-dns
```

If the rollback `apply -f` succeeds but a CoreDNS pod is in `CrashLoopBackOff` from a previous bad apply, do not delete the ConfigMap — the cluster may be running on the cached previous Corefile. Diagnose with `kubectl describe pod -n kube-system <coredns-pod>` and surface the error to the operator.

---

## NodeLocal DNSCache install rollback

NodeLocal DNSCache is **not** trivially reversible — uninstalling it during traffic causes every Pod's DNS to fail until kube-proxy / Cilium iptables-redirect rules are also reverted. Treat the install as a maintenance-window-only operation.

```bash
# Pre-uninstall: confirm the cluster's kube-proxy mode and iptables-redirect rules
kubectl -n kube-system get cm kube-proxy -o jsonpath='{.data.config\.conf}' | grep -i mode

# Uninstall (drains the redirect rules first because the DaemonSet's preStop hook handles cleanup)
kubectl delete -f /tmp/nodelocaldns-rendered.yaml --grace-period=60

# Verify pods drained cleanly
kubectl -n kube-system get pods -l k8s-app=node-local-dns
# (should return no resources)

# Verify DNS still works post-rollback
kubectl run --rm -it --restart=Never dns-test --image=busybox:1.36 -- nslookup kubernetes.default
```

If any Pod's DNS fails immediately after the rollback, the iptables/IPVS redirect rules did not drain. The operator may need to flush conntrack and restart kube-proxy DaemonSet pods (which is itself a kube-system DaemonSet write — outside this guard's scope; refer to platform team).

---

## Gateway API resource rollback

Apply-vs-delete distinction matters here. Routes that were just **created** (no prior state) should be rolled back by deletion of that single resource, not `apply -f`. Routes that were **patched** roll back via `apply -f baseline.yaml`.

```bash
# Patched resource — roll back to baseline
kubectl apply -f /tmp/<resource>.before.yaml

# Newly created resource — delete it (the agent is allowed to delete only its own creations)
# NOTE: this guard's RBAC binding does NOT grant `delete` on Gateway API resources.
# Newly-created Gateway API resources require operator-confirmed delete via a different principal.
kubectl delete <kind>/<name> -n <ns>
```

For `Gateway` resources, post-rollback verification: `kubectl get gateway <name> -n <ns>` should return either the baseline state or `NotFound`. The associated controller's Pods should not show `Reconcile` errors in their logs.

---

## ClusterMesh peer Secret rollback

```bash
# Delete the peer secret
kubectl delete secret <peer-cluster-name> -n kube-system

# Verify ClusterMesh status reflects peer disconnection
kubectl exec -n kube-system ds/cilium -- cilium clustermesh status
```

The kvstore replication state caches remote peer endpoint maps. Per `--clustermesh-cache-ttl` (default `0s` per upstream `docs.cilium.io`), the cache is **never revoked** after disconnect unless explicitly configured. Operators should pre-set a non-zero TTL before peering is established, otherwise rolling back the peer Secret leaves stale `ServiceImports` indefinitely.

---

## Universal rollback rules

1. **Capture before write.** No baseline → no rollback → no mutation. The agent refuses if baseline capture failed.
2. **Apply, don't delete, when in doubt.** Apply re-establishes the prior state idempotently. Delete cascades.
3. **Verify after rollback.** A rollback is not complete until verification confirms the prior state holds.
4. **Surface the rollback command before the mutation.** The user sees the rollback in the response shape **before** they approve the mutation, not after.
5. **The rollback verb is part of the proposal, not a follow-up.** If the agent cannot produce a rollback, it cannot produce a mutation.
