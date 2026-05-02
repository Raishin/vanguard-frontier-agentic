# 🔄 Argo CD Agents

<p align="center">
  <span style="font-size:3.5em">🔄</span>
</p>

Argo CD agent catalog for this marketplace.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live cluster mutation |
|---|---|---|---|
| Review agents | Audit Application, AppProject, ApplicationSet, sync-window, RBAC | read-only | not allowed by default |
| Guarded live operators | Apply sync, AppProject mutations, manage sync windows via argocd CLI or kubectl | workspace-write | approval-gated and target-confirmed only |

## 📋 GitOps review agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `argocd-gitops-review-agent` | Review Application blast-radius, AppProject boundaries, sync impersonation, RollingSync, sync-window scope | read-only | — |

## 🔒 Live-guard operators (dispatched by kubernetes-maestro)

Live-guard agents for Argo CD are housed in `agents/kubernetes/` because they operate at the Kubernetes API and Argo CD server layer:

| Agent | Primary use |
|---|---|
| `kubernetes-live-argocd-sync-guard-agent` | Guard live `argocd sync`, `argocd app set`, AppProject mutations, sync-window changes |

## 🛡️ Operating note

- Review agents stay read-only — they never trigger a sync or modify an Application
- AppProject boundaries define blast radius — a project with `clusterResourceWhitelist: [{group: '*', kind: '*'}]` is effectively cluster-admin for its Applications
- Sync impersonation (`impersonation.enabled`) is a privilege escalation path — review the service account bound to the Application before approving
- `RollingSync` with `maxUnavailable` must be reviewed against PDB/HPA settings
- All live-guard agents produce a structured verdict response — see [`docs/evidence-output-spec.md`](../../docs/evidence-output-spec.md)

## 📦 Install

```bash
# Install Argo CD review agent
npx vfa-export-agents --platform claude-code --agents argocd-gitops-review-agent --repo .

# Install all Kubernetes application platform agents (includes live-guard)
npx vfa-export-agents --platform claude-code --role kubernetes-application-platform-engineer --repo .
```
