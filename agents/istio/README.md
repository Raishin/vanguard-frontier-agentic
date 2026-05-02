# 🕸️ Istio Agents

<p align="center">
  <span style="font-size:3.5em">🕸️</span>
</p>

Istio agent catalog for this marketplace.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live cluster mutation |
|---|---|---|---|
| Review agents | Audit ambient mesh L4/L7, AuthorizationPolicy, PeerAuthentication, mTLS, Waypoint | read-only | not allowed by default |
| Guarded live operators | Apply AuthorizationPolicy, PeerAuthentication mutations on live clusters | workspace-write | approval-gated and target-confirmed only |

## 📋 Mesh review agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `istio-ambient-mesh-review-agent` | Review ztunnel L4 vs waypoint L7 enforcement boundaries, silent-bypass trap, `PERMISSIVE` mode, mTLS posture | read-only | — |

## 🔒 Live-guard operators (dispatched by kubernetes-maestro)

Live-guard agents for Istio are housed in `agents/kubernetes/` because they operate at the Kubernetes API layer:

| Agent | Primary use |
|---|---|
| `kubernetes-live-mesh-policy-guard-agent` | Guard live `kubectl apply/delete` on Istio AuthorizationPolicy, PeerAuthentication, Sidecar, Telemetry resources |

## 🛡️ Operating note

- Review agents stay read-only — they never write to the cluster
- The silent-bypass trap: a workload **without** a Waypoint proxy cannot enforce L7 `AuthorizationPolicy` rules even if a policy exists — traffic passes through ztunnel at L4 only
- `PERMISSIVE` peer authentication allows plaintext — treat it as a temporary migration mode, not a production default
- Any `AuthorizationPolicy` with `action: DENY` on a wide selector can cause unintended traffic black-holes
- All live-guard agents produce a structured verdict response — see [`docs/evidence-output-spec.md`](../../docs/evidence-output-spec.md)

## 📦 Install

```bash
# Install Istio review agent
npx vfa-export-agents --platform claude-code --agents istio-ambient-mesh-review-agent --repo .

# Install all Kubernetes network agents (includes live-guard)
npx vfa-export-agents --platform claude-code --role kubernetes-network-engineer --repo .
```
