# Evidence Path and Tooling

## Evidence path

1. Prefer live cluster evidence when a Kubernetes MCP server, `kubectl`, and `istioctl` are available against the mesh's primary cluster.
2. Fall back to the official Istio documentation (istio.io) for ambient/sidecar architecture, policy semantics, and CRD schema when live inspection is unavailable.
3. Ask only for sanitized YAML for the affected resources (`PeerAuthentication`, `AuthorizationPolicy`, `RequestAuthentication`, `Gateway`, `VirtualService`, `DestinationRule`, `Sidecar`, namespace labels) when current-state proof matters.
4. Label conclusions as `live evidence`, `documentation-based`, `sanitized user evidence`, or `inference`.

## Useful live-evidence commands

```shell
# All Istio security and traffic policies across the cluster
kubectl get peerauthentication,authorizationpolicy,requestauthentication,gateway,virtualservice,destinationrule,sidecar -A -o yaml

# Confirm mesh mode (ambient vs sidecar) — namespace labels
kubectl get namespaces --show-labels | grep -E 'istio.io/dataplane-mode|istio-injection'

# Ambient: list waypoint deployments and bindings
kubectl get gateways.gateway.networking.k8s.io -A -l istio.io/waypoint-for
kubectl get pods -A -l gateway.networking.k8s.io/gateway-name

# Inspect ztunnel state on each node
kubectl -n istio-system get daemonset ztunnel
istioctl x ztunnel-config workload      # what ztunnel sees as in-mesh workloads
istioctl x ztunnel-config policies      # what L4 policies ztunnel is enforcing
istioctl x ztunnel-config services      # service-to-workload mapping

# Validate proposed changes before applying
istioctl analyze -n <namespace>
istioctl analyze --recursive .

# For sidecar mode: which workloads have sidecars injected
kubectl get pods -A -o jsonpath='{range .items[?(@.metadata.annotations.sidecar\.istio\.io/inject!="false")]}{.metadata.namespace}/{.metadata.name}{"\n"}{end}'

# Inspect effective policy at a specific workload
istioctl proxy-config listener <pod>.<namespace>
istioctl proxy-config cluster <pod>.<namespace>
istioctl authz check <pod>.<namespace>

# Mesh control-plane state
kubectl -n istio-system get deploy istiod -o yaml
istioctl version
istioctl proxy-status
```

## Mesh state to confirm before review

- **Mesh mode per namespace** — sidecar (`istio-injection=enabled`), ambient (`istio.io/dataplane-mode=ambient`), or none. A single mesh can mix modes; conclusions differ.
- **Waypoint deployment for ambient namespaces** — `kubectl get gateways.gateway.networking.k8s.io -n <namespace> -l istio.io/waypoint-for`. Without a waypoint, L7 AuthorizationPolicy rules in that namespace are not enforced.
- **Istio version** (`istioctl version`) — ambient went GA in 1.24; older versions have different semantics.
- **Whether `PeerAuthentication` exists in the mesh root namespace** (default `istio-system` or whatever `meshConfig.rootNamespace` points to). Mesh-wide policies live there.
- **Whether multi-cluster (multi-primary or primary-remote)** is in use — `AuthorizationPolicy` evaluation crosses cluster boundaries when mesh networking is configured.

## Sanitization rules

- Never request kubeconfig contents, mesh root CA private keys, JWKs private keys, or workload service-account tokens.
- Replace identifiable cluster URLs, JWT issuer URLs (when sensitive), and namespace names with placeholders unless the user provides them.
- Do not print Istio root CA certificates beyond their public certificate body.
