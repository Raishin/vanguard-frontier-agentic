# Evidence Path and Tooling

## Evidence path

1. Prefer live cluster evidence when a Kubernetes MCP server, `kubectl`, the CNI's own CLI (`cilium`, `calicoctl`), Hubble, and (when in scope) node-shell access are available.
2. Fall back to upstream documentation for canonical behavior: kubernetes.io for Service/EndpointSlice/Gateway concepts, gateway-api.sigs.k8s.io for resource semantics and stability channels, docs.cilium.io for dataplane and ClusterMesh, coredns.io for plugin behavior.
3. Ask only for sanitized YAML (Service, EndpointSlice, Gateway, HTTPRoute, IngressClass, GatewayClass, CoreDNS ConfigMap, kube-proxy ConfigMap) when current-state proof matters and live access is absent.
4. Label every conclusion as `live evidence`, `documentation-based`, `sanitized user evidence`, or `inference`.

## Cluster state to confirm before the review

- **Kubernetes version** — `kubectl version`. Networking semantics (EndpointSlice features, kube-proxy modes, Gateway API GA channel) gate on the version.
- **CNI plugin and version** — `kubectl -n kube-system get pods -l k8s-app=cilium` (or `calico-node`, `aws-node`, etc.) and the agent's `version` subcommand. The dataplane is the rest of the conversation.
- **kube-proxy mode (or replacement)** — `kubectl -n kube-system get cm kube-proxy -o yaml | grep -i mode`. On Cilium KPR, the DS may be absent.
- **IPAM mode** — Cilium: `cilium config view | grep -iE 'ipam|cluster-pool|pod-cidr'`. Calico: `calicoctl get ippool`. Cloud-native CNIs: vendor docs.
- **Pod and Service CIDRs** — `kubectl cluster-info dump | grep -E '(cluster-cidr|service-cluster-ip-range)'` or kube-controller-manager flags.
- **Node MTU and overlay MTU** — `ip link show` on a node; `cilium config view | grep -i mtu`.
- **Dual-stack posture** — `kubectl get svc -A -o jsonpath='{range .items[*]}{.metadata.name}{"\t"}{.spec.ipFamilies}{"\n"}{end}'`.
- **CoreDNS replica count, Corefile, autoscaler** — `kubectl -n kube-system get deploy coredns`, `cm coredns`.
- **NodeLocal DNSCache presence** — `kubectl -n kube-system get ds node-local-dns`.
- **Gateway API CRDs and channel** — `kubectl get crd | grep gateway.networking.k8s.io` plus the controller version.
- **Multi-cluster posture** — Cilium ClusterMesh: `cilium clustermesh status`. Submariner: `subctl show all`. Istio multi-cluster: control-plane endpoints.
- **Hubble enabled** — required for cross-cluster flow observability.

## Sanitization rules

- Never request kubeconfig contents, ClusterMesh peer Secrets, Cilium agent tokens, or service account JWTs.
- Replace cluster IDs, peer cluster URLs, public LB IPs, FQDNs, and namespace names with placeholders unless the user provides them in sanitized form.
- Do not print Cilium or Istio control-plane Secrets, service account tokens, or registration manifests.
- Do not exfiltrate `coredns` upstream resolver IPs from cloud metadata when those are sensitive.

## Lightweight active probes (read-only) the review may suggest

These probes touch only test pods or a debug DaemonSet. They mutate nothing operational.

```shell
# DNS resolution path on a pod, including search-list expansion
kubectl run dnsprobe --rm -it --image=registry.k8s.io/e2e-test-images/agnhost:2.43 -- \
  dig +short +trace <name>

# Path MTU discovery from pod to peer
kubectl run mtuprobe --rm -it --image=alpine -- \
  ping -M do -c 3 -s <payload-bytes> <peer-ip>

# Service VIP reachability and load distribution
kubectl run vipprobe --rm -it --image=curlimages/curl -- \
  sh -c 'for i in $(seq 50); do curl -s http://<svc>/ | grep -i pod-name; done | sort | uniq -c'

# Hubble flow on a single pod over 60s
hubble observe --from-pod <ns>/<pod> --since 60s
```

A review that recommends running any of these must explicitly mark them as ephemeral test pods, set `--rm`, and never recommend `--privileged` on a probe.
