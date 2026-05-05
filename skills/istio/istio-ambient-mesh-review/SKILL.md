---
name: istio-ambient-mesh-review
description: Use this skill for Istio service mesh review across both sidecar mode and ambient mode (ztunnel L4 + optional waypoint L7). Covers PeerAuthentication, AuthorizationPolicy, RequestAuthentication, Gateway, VirtualService, DestinationRule, Sidecar, and waypoint placement. Trigger when the user asks whether an Istio policy is correct, whether mTLS is strict, whether L7 AuthorizationPolicy will actually be enforced in ambient mode, or whether a mesh-wide PeerAuthentication change is safe.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-05"
  category: security
---

# Istio Ambient Mesh Review

## Purpose

Review Istio configuration against zero-trust correctness and the most common ambient-mode trap: **L7 `AuthorizationPolicy` rules silently ignored when no waypoint is deployed**. Ambient mode uses ztunnel for L4 zero-trust on every node, but L7 features (HTTP method, path, JWT claim matching, request header inspection) require an explicit waypoint proxy. Without one, the L7 rules in the policy are accepted but never enforced.

## Lean operating rules

- Prefer live cluster evidence (`kubectl get peerauthentication,authorizationpolicy,requestauthentication,gateway,virtualservice,destinationrule,sidecar -A -o yaml` plus `istioctl analyze` and `istioctl x ztunnel-config`) when the active client exposes it; otherwise fall back to official Istio documentation (istio.io) and sanitized YAML.
- Separate confirmed facts from inference. If mesh mode (sidecar vs ambient), waypoint deployment, and namespace labels were not queried, say so.
- **Ambient L7 policy without a waypoint is a critical finding** — the policy looks active, the API server accepts it, but ztunnel only enforces L4. The L7 fields are silently bypassed.
- Treat `PeerAuthentication` with `mode: PERMISSIVE` or `mode: DISABLE` in production as a critical finding — mTLS is the foundation of mesh zero-trust.
- Treat any mesh-wide (root namespace) `PeerAuthentication` change as a critical-blast-radius finding — the entire mesh is affected at once.
- Challenge `AuthorizationPolicy` with `action: ALLOW` and broad `from` selectors — the default action when no policy is provisioned is ALLOW, so the only thing that creates zero-trust is a deny policy or an explicit ALLOW with bounded scope.
- Challenge `RequestAuthentication` JWKs URL changes — JWT validation depends on this.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.

## References

Load these only when needed:

- [Evidence path and tooling](references/mcp-and-evidence.md) — use when choosing live cluster evidence, confirming mesh mode and waypoint deployment, or switching to documentation mode.
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review, applying ambient/sidecar stress checks, or formatting the final answer.
- [Official sources](references/official-sources.md) — use when you need the detailed Istio documentation list, ambient mode internals, and grounded insights.

## Response minimum

Return, at minimum:

- the scoped target (mesh-wide vs namespace-scoped vs workload-scoped) and evidence level,
- the mesh mode (sidecar, ambient, mixed) and the waypoint deployment state for the workloads involved,
- the mTLS posture (`STRICT` / `PERMISSIVE` / `DISABLE`) on PeerAuthentication,
- the AuthorizationPolicy enforcement layer (L4 ztunnel-enforced vs L7 waypoint-enforced) and whether L7 rules will actually run,
- the safest next actions and rollback plan,
- the assumptions or blockers that prevent stronger conclusions.
