# 🚢 Argo CD Skills

<p align="center">
  <!-- 🖼️ Add an Argo CD logo to assets/logos/cnative/argocd/ and update this path -->
  <span style="font-size:3.5em">🚢</span>
</p>

This folder contains Argo CD-focused skills curated for this marketplace.

## Local marketplace portfolio

This folder contains **1** local Argo CD skill:

- `argocd-gitops-review`

## Portfolio posture

Argo CD skills for evidence-backed GitOps delivery review across `Application`, `AppProject`, `ApplicationSet`, sync windows, RBAC, sync impersonation, and multi-cluster (Argo CD Agent) topologies.

These skills are intentionally conservative:

- prefer `kubectl get applications,appprojects,applicationsets -n argocd -o yaml` and `argocd-cm` configmap state for live grounding before any review
- treat `application.sync.impersonation.enabled: false` in production as a critical finding — the controller's cluster-admin ServiceAccount is the sync identity
- treat `AppProject` with `sourceRepos: ['*']` and `destinations: ['*']` as a wide-blast-radius finding requiring explicit justification
- challenge `automated.prune: true` + `automated.selfHeal: true` on production Applications — Git divergence becomes irreversible deletion
- challenge `ApplicationSet` generators that include unbounded clusters or label selectors — one mis-labeled cluster joins the rollout
- prefer `destinationServiceAccounts` (per-Application impersonation) over the controller's default cluster-admin
- use official Argo CD documentation (argo-cd.readthedocs.io) for sync semantics, RBAC syntax, ApplicationSet strategies, and Argo CD Agent hub-and-spoke topology

Run `npm run validate` after changing cataloged Argo CD skills.
