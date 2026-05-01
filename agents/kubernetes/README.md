# ☸️ Kubernetes Agents

<p align="center">
  <!-- 🖼️ Add a Kubernetes logo to assets/logos/cloud/kubernetes/ and update this path -->
  <span style="font-size:3.5em">☸️</span>
</p>

Kubernetes agent catalog for this marketplace. 😄

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live cluster mutation |
|---|---|---|---|
| Review agents | Audit RBAC objects, detect escalation paths, assess scope | read-only | not allowed by default |
| Guarded live operators | Work in repos or shells connected to live clusters via kubectl | workspace-write | approval-gated and target-confirmed only |

## 🔐 RBAC agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `kubernetes-rbac-review-agent` | Review Roles, ClusterRoles, RoleBindings, ClusterRoleBindings | read-only | — |
| `kubernetes-live-rbac-mutation-guard-agent` | Guard live kubectl apply/create/delete on RBAC objects | current-state capture + escalation check + approval required | `escalate`, `bind`, or `impersonate` verbs present; wildcard verb/resource grants; cluster-admin binding without platform-team sign-off |

## 🛡️ Operating note

- 😄 review agents stay read-only — they never write to the cluster
- 🚦 the live mutation guard captures `kubectl get ... -o yaml` before every write — RBAC has no built-in rollback
- 🚫 escalation verbs (`escalate`, `bind`, `impersonate`) are hard stops — no approval path bypasses them without explicit platform-team sign-off
- ⚠️ cached service account tokens remain valid after binding deletion until they expire (up to 1 hour)
- 🧾 all live-guard agents produce a structured verdict response — see [`docs/evidence-output-spec.md`](../../docs/evidence-output-spec.md)
