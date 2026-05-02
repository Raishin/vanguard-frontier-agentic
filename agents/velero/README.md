# 💾 Velero Agents

<p align="center">
  <span style="font-size:3.5em">💾</span>
</p>

Velero agent catalog for this marketplace.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live cluster mutation |
|---|---|---|---|
| Guarded live operators | Approval-gated Velero restore, schedule, and backup lifecycle operations on live clusters | workspace-write | approval-gated and target-confirmed only |

## 🔒 Live-guard operators (dispatched by kubernetes-maestro)

Live-guard agents for Velero are housed in `agents/kubernetes/` because they operate at the Kubernetes API layer:

| Agent | Primary use |
|---|---|
| `kubernetes-live-velero-restore-guard-agent` | Guard live `velero restore create`, backup schedule deletion, and backup lifecycle operations |

## 🛡️ Operating note

- Review agents stay read-only — they never write to the cluster
- The restore guard requires **explicit platform-team sign-off** with cluster context and backup name before every `velero restore create`
- `existingResourcePolicy: update` overwrites live Secrets, ConfigMaps, RBAC objects, and ServiceAccounts — treat as a destructive operation
- Cluster-wide restore (`includedNamespaces: []`) requires platform-team sign-off with ticket reference
- Always run `velero restore create --dry-run` before live execution (except active P0 incidents with explicit override)
- RBAC objects restored from backup may grant elevated permissions if the backup was taken from a different cluster or environment
- All live-guard agents produce a structured verdict response — see [`docs/evidence-output-spec.md`](../../docs/evidence-output-spec.md)

## 📦 Install

```bash
# Install Velero restore guard agent
npx vfa-export-agents --platform claude-code --agents kubernetes-live-velero-restore-guard-agent --repo .

# Install all Kubernetes disaster recovery agents
npx vfa-export-agents --platform claude-code --role kubernetes-disaster-recovery-engineer --repo .
```
