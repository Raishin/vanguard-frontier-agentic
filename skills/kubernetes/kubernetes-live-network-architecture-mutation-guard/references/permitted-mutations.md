# Permitted mutations

This is the explicit allowlist. Any mutation outside this list MUST be either rejected by the prompt rules OR refused by the cluster RBAC binding.

## Service spec patches (low blast radius, reversible)

### `internalTrafficPolicy` / `externalTrafficPolicy`

Reversible field on `v1.Service`. Default is `Cluster`; `Local` constrains routing to node-local endpoints.

```bash
# Capture baseline
kubectl get svc <name> -n <ns> -o yaml > /tmp/svc-<name>.before.yaml

# Apply the patch
kubectl patch svc <name> -n <ns> --type=merge \
  -p '{"spec":{"externalTrafficPolicy":"Cluster"}}'

# Verify EndpointSlices still serve the Service
kubectl get endpointslice -n <ns> -l kubernetes.io/service-name=<name>

# Rollback
kubectl apply -f /tmp/svc-<name>.before.yaml
```

Refusal triggers inside this mutation type:
- Setting `externalTrafficPolicy: Local` on a Service whose backend Pods are not present on every Node that the LoadBalancer health-check targets — blackhole risk.
- Setting `internalTrafficPolicy: Local` on a Service consumed cluster-wide where backends are not co-located with consumers on every Node.

### `service.kubernetes.io/topology-mode: Auto` annotation

Annotation, additive. Only applies when `Service` is multi-zone with sufficient endpoints per zone.

```bash
kubectl annotate svc <name> -n <ns> \
  service.kubernetes.io/topology-mode=Auto --overwrite
```

Rollback: `kubectl annotate svc <name> -n <ns> service.kubernetes.io/topology-mode-`

### `spec.trafficDistribution` field (KEP-4444)

Newer API; supersedes the annotation when both are set. Field-level patch on `v1.Service`.

```bash
kubectl patch svc <name> -n <ns> --type=merge \
  -p '{"spec":{"trafficDistribution":"PreferClose"}}'
```

Rollback: `kubectl patch svc <name> -n <ns> --type=json -p='[{"op":"remove","path":"/spec/trafficDistribution"}]'`

## CoreDNS Corefile patch (medium blast radius, reversible with backup)

The CoreDNS `reload` plugin polls the Corefile every 30 seconds. If the new Corefile is invalid, CoreDNS rejects the reload and continues serving the previous config — but if the pod restarts for any other reason, it will fail to start. Always have a baseline.

```bash
# Capture baseline (CRITICAL — never skip)
kubectl get cm coredns -n kube-system -o yaml > /tmp/coredns.before.yaml

# Apply
kubectl apply -f /tmp/coredns.new.yaml

# Verify reload (look for "Reloading complete" in logs within 60 seconds)
kubectl -n kube-system logs -l k8s-app=kube-dns --tail=50 --since=2m | grep -i "reload"

# Verify no CrashLoopBackOff
kubectl -n kube-system get pods -l k8s-app=kube-dns

# If any pod is unhealthy — rollback
kubectl apply -f /tmp/coredns.before.yaml
```

Refusal triggers:
- New Corefile lacks the `reload` plugin (would prevent future zero-downtime updates).
- New Corefile lacks the `loop` plugin (CoreDNS forward-loop detection).
- New Corefile lacks the `health` plugin (liveness probe target removed).
- New Corefile lacks the `kubernetes` plugin or its `pods insecure` mode set to anything other than the cluster's documented standard.
- `forward .` target is changed to a private IP without confirming reachability from CoreDNS pod network.

## NodeLocal DNSCache install / upgrade (medium blast radius, requires explicit human gate)

Manifest from upstream `kubernetes/dns` repository (`nodelocaldns.yaml`). The install adds a `DaemonSet` and patches kube-proxy iptables / IPVS rules to redirect DNS traffic to the local cache. **This is a node-wide control-plane change**; refuse without an explicit human go-ahead even though the manifest itself is documented.

```bash
# Capture cluster DNS service IP for the manifest's __PILLAR__LOCAL__DNS__ token
kubectl get svc -n kube-system kube-dns -o jsonpath='{.spec.clusterIP}'

# Apply the upstream-rendered manifest (with values substituted)
kubectl apply -f /tmp/nodelocaldns-rendered.yaml

# Verify rollout — every Node must have a Ready node-local-dns pod within 5 minutes
kubectl rollout status ds/node-local-dns -n kube-system --timeout=5m

# Rollback (uninstall — high impact, plan before applying)
kubectl delete -f /tmp/nodelocaldns-rendered.yaml
```

Refusal triggers:
- Cluster does not run kube-proxy or Cilium kube-proxy replacement (NodeLocalDNS depends on a kube-proxy mode that supports the iptables / IPVS redirect).
- The `DaemonSet` request lacks `resources.limits.memory` (NodeLocal DNSCache OOM is a known node-wide DNS outage).
- The `DaemonSet` request lacks a `PodDisruptionBudget`.
- Node selector excludes any node — partial DNS coverage is worse than none.

## Gateway API resources (medium blast radius, reversible)

### `Gateway` resource

```bash
# Pre-check: GatewayClass must reference a controller that is actually running
GWC=$(yq '.spec.gatewayClassName' /tmp/gw.yaml)
CONTROLLER=$(kubectl get gatewayclass $GWC -o jsonpath='{.spec.controllerName}')
# Confirm controller pods are Ready before applying

kubectl apply -f /tmp/gw.yaml
kubectl wait --for=condition=Programmed gateway/<name> -n <ns> --timeout=2m

# Status verification
kubectl get gateway <name> -n <ns> -o jsonpath='{.status.conditions}'
```

Refusal triggers:
- `GatewayClass` does not exist or its controller is not running.
- `spec.allowedRoutes.namespaces.from: All` without an accompanying `ReferenceGrant` strategy — security boundary expansion.
- `Listener` overlap on (port, protocol, hostname) with existing Gateway in same scope.

### `HTTPRoute` / `GRPCRoute` / `TLSRoute`

GA / Standard channel since Gateway API v1.1.0 (per `gateway-api.sigs.k8s.io/api-types/grpcroute`).

```bash
kubectl apply -f /tmp/route.yaml
kubectl wait --for=condition=Accepted httproute/<name> -n <ns> --timeout=1m

# Status verification — Accepted: True AND ResolvedRefs: True per parent
kubectl get httproute <name> -n <ns> -o jsonpath='{.status.parents}'
```

Refusal triggers:
- Route attaches to a Gateway that is not in `Programmed: True`.
- `parentRefs` reference a Gateway in another namespace without a matching `ReferenceGrant`.

### `ReferenceGrant`

Cross-namespace authorization for routes. Always-additive; deletion can break in-flight routes.

```bash
kubectl apply -f /tmp/refgrant.yaml
# No status conditions; verify by querying the route's ResolvedRefs.
```

## Cilium ClusterMesh peer Secret (medium blast radius, requires explicit human gate)

Creates a `Secret` in the Cilium namespace whose data field carries the peer cluster's etcd client certificate, key, and CA. **Never log or print the data fields.**

```bash
# Capture baseline (confirm absence)
kubectl get secret cilium-clustermesh -n kube-system 2>&1 | tee /tmp/clustermesh-pre.txt

# Apply the peer secret — secret data is sensitive
kubectl apply -f /tmp/clustermesh-peer.yaml

# Verify ClusterMesh comes up
kubectl exec -n kube-system ds/cilium -- cilium clustermesh status

# Rollback
kubectl delete secret cilium-clustermesh -n kube-system
```

Refusal triggers:
- Secret destination namespace does not match the documented Cilium ClusterMesh namespace for this cluster.
- Secret name does not exactly match the peer cluster identifier.
- Secret data contains plaintext that looks like a CA or client key but is not properly base64-encoded for the `Secret` `data` field.
- ClusterMesh peer `policy-default-local-cluster` flag has not been considered (delegate to `cilium-network-policy-review-agent`).
