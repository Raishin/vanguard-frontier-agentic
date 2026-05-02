# Workflow and Output Contract

## Workflow

### Step 1 — Identify mesh mode for the affected namespaces

Istio supports three deployment modes that can coexist in one mesh. The review path differs based on mode.

1. **Sidecar mode** — pods have an Envoy sidecar injected. Namespace labeled `istio-injection=enabled`. All policy is enforced at the sidecar.
2. **Ambient mode** — no sidecars; ztunnel runs as a per-node DaemonSet for L4 zero-trust + optional waypoint proxies for L7. Namespace labeled `istio.io/dataplane-mode=ambient`.
3. **Mixed** — some workloads in a namespace use sidecars, others use ambient. Verify per-pod with annotations.

Reference: [Choosing between sidecar and ambient](https://istio.io/latest/docs/overview/dataplane-modes/) and [Ambient mode overview](https://istio.io/latest/docs/ambient/overview/).

### Step 2 — Confirm waypoint deployment for ambient namespaces (the L7 trap)

This is the most important ambient-specific check. Without a waypoint, L7 `AuthorizationPolicy` rules are silently ignored.

1. List waypoints: `kubectl get gateways.gateway.networking.k8s.io -n <namespace> -l istio.io/waypoint-for`.
2. Confirm the waypoint binding label on the namespace, ServiceAccount, or workload (`istio.io/use-waypoint: <waypoint-name>`).
3. Cross-reference any `AuthorizationPolicy` that uses L7 fields (`to.operation.methods`, `to.operation.paths`, `to.operation.hosts`, `when` keys for `request.headers`, `request.auth.claims`) — if no waypoint is bound to the workload, **the L7 rules are accepted by the API server but never enforced**.

The L4 fields that ztunnel enforces without a waypoint:

- `from.source.principals` (SPIFFE identities — the workload's ServiceAccount mTLS identity)
- `from.source.namespaces`
- `to.operation.ports`
- `when` keys: `source.principal`, `source.namespace`, `destination.port`, `connection.sni`

Reference: [L4 Authorization Policy in ambient](https://istio.io/latest/docs/ambient/usage/l4-policy/) and [Waypoint configuration](https://istio.io/latest/docs/ambient/usage/waypoint/).

### Step 3 — Audit `PeerAuthentication`

`PeerAuthentication` controls workload-to-workload mTLS. Three modes exist with very different security properties:

1. **`STRICT`** — all peer connections must use mTLS. Plaintext connections are rejected. Production target.
2. **`PERMISSIVE`** — accepts both mTLS and plaintext. Useful only during migration.
3. **`DISABLE`** — disables mTLS. Plaintext only.

Stress-tests:

- **Mesh-wide PeerAuthentication** lives in the mesh root namespace (default `istio-system`). A change here affects every workload in every namespace simultaneously. Treat as critical-blast-radius.
- **Namespace-scoped PeerAuthentication** with `mode: PERMISSIVE` in production is a finding — there is no migration in progress; this is technical debt.
- **Workload-scoped PeerAuthentication** with `mode: DISABLE` for a specific port (e.g., a health-check port) is sometimes legitimate but always requires justification.
- A namespace with **no `PeerAuthentication`** inherits mesh-wide. If mesh-wide is `PERMISSIVE`, the namespace is also `PERMISSIVE`.

Reference: [PeerAuthentication API](https://istio.io/latest/docs/reference/config/security/peer_authentication/).

### Step 4 — Audit `AuthorizationPolicy`

`AuthorizationPolicy` controls who can talk to whom. Default action when no policy exists is **ALLOW** — there is no implicit deny. Zero-trust requires explicit deny policies or explicit narrow ALLOW policies that combine to leave nothing reachable by default.

Three actions: `ALLOW`, `DENY`, `CUSTOM`, `AUDIT`.

1. **`DENY` policies are evaluated first**, then `ALLOW`. If multiple match, DENY wins.
2. **Empty `rules` with `action: DENY`** denies everything — total lockdown.
3. **`action: ALLOW` with no `from` block** allows from anywhere — only useful for narrowing by `to`.
4. **`action: ALLOW` with `from.source.principals: ['*']`** is also "anywhere" — no practical narrowing.

Stress-tests:

- An `AuthorizationPolicy` with `action: ALLOW` and `from.source.namespaces: ['*']` is a documentation-only deny — it allows all and denies none.
- L7 fields (`to.operation.methods`, `request.auth.claims`) in ambient mode without a waypoint are silently bypassed.
- `action: AUDIT` is a logging-only mode that does not enforce — use only for migration.
- Multi-cluster mesh: `AuthorizationPolicy` in one cluster can affect workloads called from another cluster; verify mesh networking topology.

Reference: [AuthorizationPolicy API](https://istio.io/latest/docs/reference/config/security/authorization-policy/).

### Step 5 — Audit `RequestAuthentication`

`RequestAuthentication` defines JWT validation — `jwksUri`, `issuer`, `audiences`, `forwardOriginalToken`. Key concerns:

1. **JWKs URI rotation** — if the issuer rotates signing keys, Istio caches the JWKs response. The `jwksUri` URL must remain reachable; outages here cause every JWT to fail.
2. **`forwardOriginalToken: true`** with sensitive JWTs forwards the bearer token to backend services — they must be trusted.
3. **`audiences: []` or missing** — accepts JWTs intended for any audience. Cross-service token replay risk.
4. **Multiple `RequestAuthentication` for the same workload** — Istio combines them. A misconfigured second one can weaken a strict first one.

Reference: [RequestAuthentication API](https://istio.io/latest/docs/reference/config/security/request_authentication/).

### Step 6 — Audit `Gateway`, `VirtualService`, `DestinationRule`, `Sidecar`

Traffic routing concerns:

- **`Gateway` with `tls.mode: SIMPLE` and no `credentialName`** — broken or insecure TLS termination.
- **`Gateway` with `tls.mode: PASSTHROUGH`** plus L7 routing in `VirtualService` — incompatible (passthrough cannot be inspected).
- **`VirtualService.http.route` with `weight`-based traffic split** — verify total weights sum to 100; otherwise traffic is dropped.
- **`DestinationRule.trafficPolicy.tls.mode: DISABLE`** on production destinations — disables Istio-side mTLS to the destination.
- **`Sidecar` resource with `egress.hosts: ['*/*']`** — disables egress restriction.

Reference: [Gateway API](https://istio.io/latest/docs/reference/config/networking/gateway/), [VirtualService API](https://istio.io/latest/docs/reference/config/networking/virtual-service/), [DestinationRule API](https://istio.io/latest/docs/reference/config/networking/destination-rule/), [Sidecar API](https://istio.io/latest/docs/reference/config/networking/sidecar/).

### Step 7 — Validate with `istioctl analyze`

`istioctl analyze` runs the same checks the control plane runs and surfaces structural problems. Run it on the proposed YAML before applying:

```shell
istioctl analyze -n <namespace>            # one namespace
istioctl analyze --all-namespaces          # whole mesh
istioctl analyze --recursive ./manifests/  # offline against files
```

Common findings:

- `IST0101` — referenced resource not found (e.g., `VirtualService` references a missing host).
- `IST0118` — port name not following Istio's protocol convention (e.g., `tcp` vs `tcp-mysql`).
- `IST0127` — namespace not labeled for injection.

## Output

Return:

- **target**: the resource and its scope (mesh-wide, namespace, workload),
- **evidence level**: `live evidence` / `documentation-based` / `sanitized user evidence` / `inference`,
- **mesh mode**: sidecar, ambient, or mixed for the affected workloads,
- **waypoint state**: deployed and bound, missing, or not applicable (sidecar mode),
- **L7 enforcement assessment**: whether L7 fields will actually run, with explicit "silently ignored" callouts where applicable,
- **mTLS posture**: `STRICT` / `PERMISSIVE` / `DISABLE` per workload / namespace / mesh,
- **risk findings** (with severity: high / medium / low),
- **safest next actions** with sample manifest changes and `istioctl analyze` output,
- **rollback plan**: how to revert the change without breaking mesh traffic mid-flight,
- **assumptions and missing facts**.

## Security notes

- Never recommend `PeerAuthentication` `mode: PERMISSIVE` or `DISABLE` for production without a documented mTLS migration plan with a date.
- Never recommend a mesh-wide root-namespace policy change without staged rollout (single namespace first, observe, expand).
- Never recommend disabling waypoint enforcement for an ambient namespace if any L7 `AuthorizationPolicy` exists for that namespace.
- Do not print Istio root CA private keys or JWKs private keys.
