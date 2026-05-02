# ☸️ Kubernetes Agents

<p align="center">
  <span style="font-size:3.5em">☸️</span>
</p>

Kubernetes agent catalog for this marketplace.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live cluster mutation |
|---|---|---|---|
| Review agents | Audit RBAC, admission, PSA, workload identity, mesh, networking | read-only | not allowed by default |
| Guarded live operators | Work in repos or shells connected to live clusters via kubectl / argocd CLI | workspace-write | approval-gated and target-confirmed only |

---

## 🧭 Maestro router

| Agent | Primary use | Default live posture |
|---|---|---|
| `kubernetes-maestro-agent` | Classify task → select narrowest specialist(s) → dispatch in parallel; never auto-dispatch live-guard agents | read-only |

Install the maestro if you want a single entry point that routes to the right specialist automatically.

---

## 🔐 RBAC agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `kubernetes-rbac-review-agent` | Review Roles, ClusterRoles, RoleBindings, ClusterRoleBindings | read-only | — |
| `kubernetes-live-rbac-mutation-guard-agent` | Guard live kubectl apply/create/delete on RBAC objects | current-state capture + escalation check + approval required | `escalate`, `bind`, or `impersonate` verbs present; wildcard verb/resource grants; cluster-admin binding without platform-team sign-off |

---

## 🛡️ Pod security agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `kubernetes-psa-review-agent` | Review Pod Security Admission namespace labels — enforce/audit/warn mode, version pinning, PSP migration posture | read-only | — |
| `kubernetes-pod-spec-review-agent` | Review individual Pod/Deployment/StatefulSet specs — securityContext, capabilities, privileged, readOnlyRootFilesystem, host network/PID/IPC, image tag pinning | read-only | — |

---

## 🔑 Secrets and PKI agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `external-secrets-operator-review-agent` | Review ESO SecretStore, ClusterSecretStore, ExternalSecret, PushSecret for scope creep, auth anti-patterns, refresh interval, dataFrom blast radius | read-only | — |

---

## 💰 Cost attribution agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `kubecost-chargeback-allocation-review-agent` | Review Kubecost label taxonomy, shared cost model, idle allocation policy, namespace budget alerts, API auth | read-only | — |

---

## 🆔 Workload identity agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `kubernetes-workload-identity-review-agent` | Review IRSA, Azure Workload Identity, GKE Workload Identity Federation, projected token config, `automountServiceAccountToken`, OIDC trust policy scope | read-only | — |

---

## 🛡️ Admission policy agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `kubernetes-live-admission-policy-guard-agent` | Guard live kubectl apply/delete on Kyverno ClusterPolicy, Policy, PolicyException, ValidatingAdmissionPolicy, MutatingAdmissionPolicy | current-state capture + blast-radius assessment + explicit platform-team sign-off required | `failureAction: Enforce` on untested policy; PolicyException without expiry or scope evidence; wildcard subject |

---

## 🔄 GitOps / sync agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `kubernetes-live-argocd-sync-guard-agent` | Guard live argocd sync, argocd app set, AppProject mutations, sync-window changes | current-state capture + rollback plan + explicit platform-team sign-off required | Sync impersonation without identity review; AppProject with cluster-admin clusterResourceWhitelist; sync-window deletion without downstream impact assessment |

---

## 🕸️ Mesh policy agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `kubernetes-live-mesh-policy-guard-agent` | Guard live kubectl apply/delete on Istio AuthorizationPolicy, PeerAuthentication, Sidecar, Telemetry resources | current-state capture + traffic impact assessment + explicit platform-team sign-off required | Policy with `action: DENY` on wide selector without traffic analysis; removing `STRICT` PeerAuthentication without mTLS migration plan |

---

## 🐝 Network policy agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `kubernetes-live-network-policy-guard-agent` | Guard live kubectl apply/delete on CiliumNetworkPolicy, CiliumClusterwideNetworkPolicy, standard NetworkPolicy | current-state capture + connectivity impact assessment + explicit platform-team sign-off required | Policy permitting egress to 169.254.169.254 (metadata service) without explicit justification; clusterwide policy deletion without replacement |

---

## 💾 Backup and restore agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `kubernetes-live-velero-restore-guard-agent` | Guard live velero restore create, backup schedule deletion, and backup lifecycle operations | current-state capture + pre-restore checklist + explicit platform-team sign-off required | Cluster-wide restore without ticket reference; restore from `PartiallyFailed` backup without explicit acknowledgment; `existingResourcePolicy: update` without approver review of overwrite scope |

---

## 🛡️ Operating notes

- Review agents stay read-only — they never write to the cluster
- Live-guard agents require **explicit platform-team sign-off** with cluster context and current state before every mutation
- All live-guard agents capture `kubectl get ... -o yaml` before any write — this is the rollback artifact
- RBAC has no built-in rollback — cached service account tokens remain valid after binding deletion until they expire (up to 1 hour)
- Admission policy changes with `failureAction: Enforce` can block workload admission cluster-wide — treat them as breaking changes
- All live-guard agents produce a structured verdict response — see [`docs/evidence-output-spec.md`](../../docs/evidence-output-spec.md)

---

## 📦 Install

```bash
# 🧭 Install the maestro router (routes to all specialists)
npx vfa-export-agents --platform claude-code --agents kubernetes-maestro-agent --repo .

# 🔐 RBAC specialist
npx vfa-export-agents --platform claude-code --agents kubernetes-rbac-review-agent --repo .

# 🆔 Workload identity specialist
npx vfa-export-agents --platform claude-code --agents kubernetes-workload-identity-review-agent --repo .

# 📦 Install by role (recommended — installs the right curated set)
npx vfa-export-agents --platform claude-code --role kubernetes-admission-security-engineer --repo .
npx vfa-export-agents --platform claude-code --role kubernetes-network-engineer --repo .
npx vfa-export-agents --platform claude-code --role kubernetes-application-platform-engineer --repo .
npx vfa-export-agents --platform claude-code --role kubernetes-runtime-security-engineer --repo .
npx vfa-export-agents --platform claude-code --role kubernetes-pki-engineer --repo .
npx vfa-export-agents --platform claude-code --role kubernetes-observability-engineer --repo .
npx vfa-export-agents --platform claude-code --role kubernetes-supply-chain-security-engineer --repo .
npx vfa-export-agents --platform claude-code --role kubernetes-developer-platform-engineer --repo .
npx vfa-export-agents --platform claude-code --role kubernetes-disaster-recovery-engineer --repo .
```
