# 🔄 FluxCD Agents

<p align="center">
  <span style="font-size:3.5em">🔄</span>
</p>

FluxCD agent catalog for this marketplace.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live cluster mutation |
|---|---|---|---|
| Review agents | Audit FluxCD Kustomization, HelmRelease, GitRepository source trust, and SOPS encryption posture | read-only | not allowed |

## 📋 Kustomization and HelmRelease review agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `fluxcd-kustomization-helmrelease-review-agent` | Review FluxCD Kustomization ServiceAccount scoping, prune safety, HelmRelease version pinning and remediation, GitRepository source trust, SOPS encryption posture, and multi-tenant isolation | read-only | — |

## 🛡️ Operating note

- `Kustomization` with `prune: true` and no `deletionPolicy` annotation deletes resources when they are removed from Git — verify intent on stateful resources (PVCs, Secrets, CRDs)
- `HelmRelease` with `version: "*"` or without semver pinning auto-upgrades on every source interval, breaking production without a PR gate
- SOPS decryption failure causes Kustomization reconciliation to fail silently in some configurations — verify `decryption.provider` and `decryption.secretRef` are present
- `GitRepository` with `secretRef` using deploy keys have no automatic rotation — verify key age and rotation policy
- Multi-tenant mode requires each tenant `Kustomization` to use a scoped `ServiceAccount`; shared default SA grants cross-tenant access

*Live ArgoCD-equivalent mutations → covered by `kubernetes-live-argocd-sync-guard-agent` semantics*

## 📦 Install

```bash
# Install FluxCD review agent
npx vfa-export-agents --platform claude-code --agents fluxcd-kustomization-helmrelease-review-agent --repo .

# Install all Kubernetes developer platform agents
npx vfa-export-agents --platform claude-code --role kubernetes-developer-platform-engineer --repo .
```
