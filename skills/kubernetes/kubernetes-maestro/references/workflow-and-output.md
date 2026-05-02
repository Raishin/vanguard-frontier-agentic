# Routing table and domain taxonomy

Use this reference when classifying a task or selecting the right specialist(s).

## Routing table

| Signal keywords | Agent ID | Domain | Live-guard? |
|---|---|---|---|
| RBAC, Role, ClusterRole, RoleBinding, ClusterRoleBinding, ServiceAccount, can-i, least privilege | kubernetes-rbac-review-agent | RBAC review | No |
| apply RBAC, kubectl apply role, grant permission, bind ClusterRole, create RoleBinding, escalate verb | kubernetes-live-rbac-mutation-guard-agent | Live RBAC mutation | YES — gate required |
| PSA, PodSecurityAdmission, pod-security label, enforce/audit/warn, restricted profile, baseline profile, privileged profile, PSP migration | kubernetes-psa-review-agent | Pod security admission | No |
| Kyverno, ClusterPolicy, kyverno policy, PolicyException, mutate rule, generate rule, image verify, background scan | kyverno-policy-review-agent | Kyverno policy review | No |
| apply Kyverno policy, kubectl apply cpol, change failureAction, delete ClusterPolicy, add PolicyException | kubernetes-live-admission-policy-guard-agent | Live admission policy mutation | YES — gate required |
| IRSA, workload identity, serviceAccountToken, OIDC trust, pod identity, azure workload identity, GKE WI, annotate serviceaccount, projected token | kubernetes-workload-identity-review-agent | Workload identity review | No |
| Istio, ambient mesh, waypoint, ztunnel, AuthorizationPolicy, PeerAuthentication, mTLS, RequestAuthentication, VirtualService, DestinationRule | istio-ambient-mesh-review-agent | Istio mesh review | No |
| apply AuthorizationPolicy, apply PeerAuthentication, change mTLS, delete DENY policy, enable PERMISSIVE | kubernetes-live-mesh-policy-guard-agent | Live mesh policy mutation | YES — gate required |
| Cilium, CiliumNetworkPolicy, CiliumClusterwideNetworkPolicy, NetworkPolicy, ClusterMesh, egress gateway, Hubble, L7 policy | cilium-network-policy-review-agent | Cilium network policy review | No |
| apply CiliumNetworkPolicy, kubectl apply cnp, delete default-deny, change toCIDRSet, egress gateway | kubernetes-live-network-policy-guard-agent | Live network policy mutation | YES — gate required |
| Argo CD, ArgoCD, Application, AppProject, ApplicationSet, sync window, argocd sync, gitops, app of apps | argocd-gitops-review-agent | Argo CD GitOps review | No |
| argocd app sync, sync production, delete sync-window, expand AppProject, enable auto-sync | kubernetes-live-argocd-sync-guard-agent | Live Argo CD sync guard | YES — gate required |
| OpenTelemetry, OTEL, otelcol, collector, pipeline, receiver, processor, exporter, Instrumentation CR, TargetAllocator | opentelemetry-collector-config-review-agent | OpenTelemetry review | No |

## Domain taxonomy

| Domain | Keywords and signals |
|---|---|
| `rbac` | Role, ClusterRole, RoleBinding, ClusterRoleBinding, ServiceAccount, can-i, RBAC, least privilege, permission, verb, subject |
| `admission-security` | PSA, PodSecurityAdmission, pod-security label, enforce, audit, warn, restricted, baseline, privileged, PSP migration, Kyverno, ClusterPolicy, PolicyException, mutate, generate, image verify |
| `workload-identity` | IRSA, workload identity, serviceAccountToken, OIDC, pod identity, azure workload identity, GKE WI, projected token, bound service account |
| `mesh` | Istio, ambient mesh, waypoint, ztunnel, AuthorizationPolicy, PeerAuthentication, mTLS, RequestAuthentication, VirtualService, DestinationRule, Envoy |
| `network-policy` | Cilium, CiliumNetworkPolicy, NetworkPolicy, ClusterMesh, Hubble, egress gateway, L7 policy, CNI |
| `gitops` | Argo CD, ArgoCD, Application, AppProject, ApplicationSet, sync window, app of apps, GitOps, deployment sync |
| `observability` | OpenTelemetry, OTEL, otelcol, collector, pipeline, receiver, processor, exporter, Instrumentation CR, TargetAllocator, tracing, metrics, logs |
| `live-guard` | apply RBAC live, apply admission policy live, change mTLS live, apply network policy live, argocd sync production, requires human gate, production mutation |

## Specialist reference

### RBAC

| Agent | Domain | Use when… |
|---|---|---|
| `kubernetes-rbac-review-agent` | RBAC review | Reviewing Roles, ClusterRoles, bindings, ServiceAccount permissions, or running kubectl auth can-i audit for least privilege |
| `kubernetes-live-rbac-mutation-guard-agent` | Live RBAC mutation | Applying new RBAC objects, granting permissions, binding ClusterRoles, or escalating verbs in a live cluster — gate required |

### Admission security

| Agent | Domain | Use when… |
|---|---|---|
| `kubernetes-psa-review-agent` | Pod security admission | Reviewing PSA labels on namespaces, enforcing/auditing/warning against restricted or baseline profiles, or planning PSP migration |
| `kyverno-policy-review-agent` | Kyverno policy review | Reviewing or authoring Kyverno ClusterPolicies, mutate/generate/verify rules, PolicyExceptions, or running background scan analysis |
| `kubernetes-live-admission-policy-guard-agent` | Live admission policy mutation | Applying or deleting Kyverno ClusterPolicies, changing failureAction, or adding PolicyExceptions in a live cluster — gate required |

### Workload identity

| Agent | Domain | Use when… |
|---|---|---|
| `kubernetes-workload-identity-review-agent` | Workload identity review | Reviewing IRSA annotations, OIDC trust relationships, projected serviceAccountToken usage, Azure Workload Identity, or GKE Workload Identity setup |

### Mesh

| Agent | Domain | Use when… |
|---|---|---|
| `istio-ambient-mesh-review-agent` | Istio mesh review | Reviewing Istio ambient mesh waypoint config, AuthorizationPolicy, PeerAuthentication, mTLS mode, VirtualService/DestinationRule, or RequestAuthentication |
| `kubernetes-live-mesh-policy-guard-agent` | Live mesh policy mutation | Applying or deleting AuthorizationPolicy or PeerAuthentication, changing mTLS mode, or enabling PERMISSIVE mode in a live cluster — gate required |

### Network policy

| Agent | Domain | Use when… |
|---|---|---|
| `cilium-network-policy-review-agent` | Cilium network policy review | Reviewing CiliumNetworkPolicy, CiliumClusterwideNetworkPolicy, ClusterMesh config, Hubble observability, or L7 policy rules |
| `kubernetes-live-network-policy-guard-agent` | Live network policy mutation | Applying or deleting CiliumNetworkPolicy, removing default-deny rules, changing toCIDRSet, or modifying egress gateway config in a live cluster — gate required |

### GitOps

| Agent | Domain | Use when… |
|---|---|---|
| `argocd-gitops-review-agent` | Argo CD GitOps review | Reviewing ArgoCD Application/AppProject/ApplicationSet config, sync windows, app-of-apps patterns, or GitOps reconciliation strategy |
| `kubernetes-live-argocd-sync-guard-agent` | Live Argo CD sync guard | Triggering an argocd app sync to production, deleting sync windows, expanding AppProject scope, or enabling auto-sync on a production app — gate required |

### Observability

| Agent | Domain | Use when… |
|---|---|---|
| `opentelemetry-collector-config-review-agent` | OpenTelemetry review | Reviewing OpenTelemetry Collector pipelines, receiver/processor/exporter configs, Instrumentation CRs, or TargetAllocator setup for Kubernetes workloads |

## Multi-domain dispatch examples

### Example 1: RBAC + Workload Identity audit

**User request:** "We're migrating to IRSA on EKS. Review our ServiceAccount annotations and make sure the RBAC permissions on those accounts are least-privilege."

**Routing:**
```
Route: kubernetes-rbac-review-agent, kubernetes-workload-identity-review-agent
Reason: Task spans RBAC least-privilege review and IRSA/workload-identity trust configuration — both domains are clearly present.
Mode: parallel (2)
```

Both specialists run in parallel: `kubernetes-rbac-review-agent` audits Role/ClusterRole bindings on the relevant ServiceAccounts; `kubernetes-workload-identity-review-agent` reviews IRSA annotations and OIDC trust configuration.

---

### Example 2: Admission security + Network policy hardening

**User request:** "We want to enforce the restricted PSA profile on the payments namespace and also add a default-deny Cilium policy for that namespace."

**Routing:**
```
Route: kubernetes-psa-review-agent, cilium-network-policy-review-agent
Reason: Task covers PSA namespace label enforcement and Cilium default-deny network policy — two separate admission and network domains.
Mode: parallel (2)
```

`kubernetes-psa-review-agent` reviews the enforce label and identifies any pods that would fail the restricted profile; `cilium-network-policy-review-agent` reviews the proposed default-deny CiliumNetworkPolicy for correctness.

---

### Example 3: Mesh + GitOps + Observability review

**User request:** "We're rolling out Istio ambient mesh to prod via Argo CD. I need to review the AuthorizationPolicies, the ArgoCD Application config, and make sure our OTel collector is capturing L4 telemetry."

**Routing:**
```
Route: istio-ambient-mesh-review-agent, argocd-gitops-review-agent, opentelemetry-collector-config-review-agent
Reason: Task spans Istio AuthorizationPolicy review, Argo CD Application config review, and OpenTelemetry collector pipeline review — three distinct domains.
Mode: parallel (3)
```

All three specialists run in parallel: `istio-ambient-mesh-review-agent` reviews waypoint and AuthorizationPolicy; `argocd-gitops-review-agent` reviews the Application sync strategy and sync windows; `opentelemetry-collector-config-review-agent` reviews the collector pipeline for L4 ambient mesh telemetry.

---

### Example 4: RBAC + Kyverno policy + Workload Identity for a new service onboarding

**User request:** "Onboarding a new microservice. Need to create a ServiceAccount with correct IRSA annotations, add RBAC for it, write a Kyverno policy to enforce image signing, and make sure it can't escalate privileges."

**Routing:**
```
Route: kubernetes-rbac-review-agent, kubernetes-workload-identity-review-agent, kyverno-policy-review-agent
Reason: Task spans RBAC role design, workload identity OIDC trust, and Kyverno image-verify/privilege-escalation policy — three clearly identified domains.
Mode: parallel (3)
```

`kubernetes-rbac-review-agent` designs least-privilege Role and RoleBinding; `kubernetes-workload-identity-review-agent` reviews IRSA annotation and OIDC trust; `kyverno-policy-review-agent` drafts the image-verify ClusterPolicy with a deny rule for privilege escalation.

---

### Example 5: Live-guard gate — RBAC mutation to production

**User request:** "Apply the new ClusterRoleBinding for the payments service account in the prod cluster."

**Routing:**
```
Route: kubernetes-live-rbac-mutation-guard-agent
Reason: Applying a ClusterRoleBinding to a live production cluster is a live RBAC mutation — gate required.
Mode: live-guard-gate
```

**STOP — Live-guard gate. Before this dispatch can proceed, you must provide:**

1. **Blast-radius assessment:** Which namespaces, workloads, and users are affected by this ClusterRoleBinding? What is the scope of the verbs and resources being granted?
2. **Rollback path:** What is the exact command to revoke this binding if it grants unintended access, and how long will rollback take?
3. **Explicit written confirmation:** Type "I confirm I understand the blast radius and rollback path. Proceed."

If you cannot supply a rollback path, route to `kubernetes-rbac-review-agent` first to develop a scoped binding with a documented revocation procedure.

---

## Live-guard gate protocol

Before routing to any live-guard agent, surface all three and wait for explicit written confirmation:

1. **Blast-radius assessment** — which resources, namespaces, workloads, or users are affected if this goes wrong?
2. **Rollback path** — what is the tested recovery procedure, exact commands, and estimated recovery time?
3. **Explicit confirmation** — "I confirm I understand the blast radius and rollback path. Proceed."

If the user cannot supply a rollback path, recommend the corresponding review agent to develop the rollback path first before dispatching the live-guard agent.

## Safety checklist reference

Load [references/safety-checklist.md](safety-checklist.md) before any live-guard dispatch or when blast-radius assessment is required.
