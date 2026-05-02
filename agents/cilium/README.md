# 🐝 Cilium Agents

<p align="center">
  <span style="font-size:3.5em">🐝</span>
</p>

Cilium agent catalog for this marketplace.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live cluster mutation |
|---|---|---|---|
| Review agents | Audit CiliumNetworkPolicy, ClusterMesh, WireGuard, node-level identity, egress | read-only | not allowed by default |
| Guarded live operators | Apply CiliumNetworkPolicy/NetworkPolicy mutations on live clusters | workspace-write | approval-gated and target-confirmed only |

## 📋 Network policy review agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `cilium-network-policy-review-agent` | Review CiliumNetworkPolicy, CiliumClusterwideNetworkPolicy, standard NetworkPolicy, ClusterMesh trust, 169.254.169.254 egress posture | read-only | — |

## 🔒 Live-guard operators (dispatched by kubernetes-maestro)

Live-guard agents for Cilium are housed in `agents/kubernetes/` because they operate at the Kubernetes API layer:

| Agent | Primary use |
|---|---|
| `kubernetes-live-network-policy-guard-agent` | Guard live `kubectl apply/delete` on CiliumNetworkPolicy, CiliumClusterwideNetworkPolicy, standard NetworkPolicy |

## 🛡️ Operating note

- Review agents stay read-only — they never write to the cluster
- The instance metadata service (`169.254.169.254`) should be blocked by egress policy for workloads that do not need node-level cloud credentials — its exposure has been weaponized in cloud breaches
- ClusterMesh extends identity across clusters — a permissive policy in cluster A becomes a trust boundary for cluster B
- `CiliumClusterwideNetworkPolicy` applies across all namespaces — changes require broader blast-radius assessment than namespace-scoped policies
- All live-guard agents produce a structured verdict response — see [`docs/evidence-output-spec.md`](../../docs/evidence-output-spec.md)

## 📦 Install

```bash
# Install Cilium review agent
npx vfa-export-agents --platform claude-code --agents cilium-network-policy-review-agent --repo .

# Install all Kubernetes network agents (includes live-guard)
npx vfa-export-agents --platform claude-code --role kubernetes-network-engineer --repo .
```
