# 🕸️ Istio Skills

<p align="center">
  <!-- 🖼️ Add an Istio logo to assets/logos/cnative/istio/ and update this path -->
  <span style="font-size:3.5em">🕸️</span>
</p>

This folder contains Istio-focused skills curated for this marketplace.

## Local marketplace portfolio

This folder contains **1** local Istio skill:

- `istio-ambient-mesh-review`

## Portfolio posture

Istio skills for evidence-backed service mesh review covering both **sidecar mode** and **ambient mode** (ztunnel + optional waypoint proxies). Ambient mode introduces a layered architecture where L4 zero-trust is enforced at ztunnel and L7 features require an explicit waypoint deployment.

These skills are intentionally conservative:

- prefer `kubectl get peerauthentication,authorizationpolicy,gateway,virtualservice,destinationrule,sidecar -A -o yaml` for live mesh state grounding before any review
- treat **L7 `AuthorizationPolicy` rules in ambient mode without a waypoint** as a critical finding — the L7 fields are silently ignored when ztunnel handles the traffic alone
- challenge `PeerAuthentication` with `mode: PERMISSIVE` or `DISABLE` in production — mTLS must be `STRICT`
- challenge mesh-wide `PeerAuthentication` changes — the blast radius is the whole mesh
- use official Istio documentation (istio.io) for ambient architecture, ztunnel internals, waypoint placement, HBONE protocol, and `AuthorizationPolicy` semantic differences between sidecar and ambient modes

Run `npm run validate` after changing cataloged Istio skills.
