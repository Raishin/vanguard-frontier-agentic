# Official Sources

Load these only when needed:

- [Argo CD documentation home](https://argo-cd.readthedocs.io/en/stable/) — use as the entry point for any Argo CD authoring, install, or operator-side question.
- [Declarative setup](https://argo-cd.readthedocs.io/en/stable/operator-manual/declarative-setup/) — use for `Application`, `AppProject`, cluster Secret, repository Secret, and ConfigMap structure.
- [argocd-cm reference](https://argo-cd.readthedocs.io/en/stable/operator-manual/argocd-cm-yaml/) — use for global controller knobs including `application.sync.impersonation.enabled`, `application.sync.requireOverridePrivilegeForRevisionSync`, and `webhook.maxPayloadSizeMB`.
- [Auto-sync](https://argo-cd.readthedocs.io/en/stable/user-guide/auto_sync/) — use for `automated`, `prune`, `selfHeal` semantics and operational guidance.
- [Sync Options](https://argo-cd.readthedocs.io/en/stable/user-guide/sync-options/) — use for `Replace`, `Force`, `ServerSideApply`, `PruneLast`, `CreateNamespace`, `Validate=false`, `RespectIgnoreDifferences`.
- [Sync Windows](https://argo-cd.readthedocs.io/en/stable/user-guide/sync_windows/) — use for deploy-freeze enforcement at the AppProject level.
- [ApplicationSet Generators](https://argo-cd.readthedocs.io/en/stable/operator-manual/applicationset/Generators/) — use for `list`, `cluster`, `git`, `matrix`, `merge`, `pullRequest`, `scmProvider` generator semantics.
- [ApplicationSet Progressive Syncs (RollingSync)](https://argo-cd.readthedocs.io/en/stable/operator-manual/applicationset/Progressive-Syncs/) — use for staged ApplicationSet rollouts.
- [Argo CD RBAC](https://argo-cd.readthedocs.io/en/stable/operator-manual/rbac/) — use for `policy.csv` syntax, default role, group bindings, and granular action permissions.
- [Sync impersonation proposal](https://argo-cd.readthedocs.io/en/stable/proposals/decouple-application-sync-user-using-impersonation/) — use for the AppProject `destinationServiceAccounts` field and the least-privilege sync identity model.
- [Argo CD upgrading guide](https://argo-cd.readthedocs.io/en/stable/operator-manual/upgrading/) — use when version-specific RBAC actions or API fields matter.
- [Argo CD User Management](https://argo-cd.readthedocs.io/en/stable/operator-manual/user-management/) — use for SSO via OIDC / SAML / Dex and group claims.
- [Argo CD Webhook](https://argo-cd.readthedocs.io/en/stable/operator-manual/webhook/) — use for repository webhook setup and signature verification.
- [Argo CD Agent](https://github.com/argoproj-labs/argocd-agent) — use for hub-and-spoke multi-cluster topologies replacing direct cluster registrations.
- [Argo CD Autopilot](https://github.com/argoproj-labs/argocd-autopilot) — use when Argo CD itself is managed via GitOps.

## Grounded insights worth carrying into the skill

- The Argo CD controller defaults to running as cluster-admin on every destination cluster. The `application.sync.impersonation.enabled` flag in `argocd-cm` is the switch that activates per-Application ServiceAccount impersonation via `destinationServiceAccounts` on the AppProject.
- `AppProject` boundaries are the only enforced isolation between teams sharing one Argo CD instance. Wildcards in `sourceRepos`, `destinations`, `clusterResourceWhitelist`, or empty `namespaceResourceBlacklist` collapse the boundary.
- `automated.selfHeal: true` combined with `automated.prune: true` means a Git revert (or Git outage that exposes a stale ref) deletes prod resources. There is no built-in confirmation step.
- ApplicationSet's `cluster` generator with an empty selector auto-onboards every newly registered cluster. This is the most-cited blast-radius mode in Argo CD post-incident reviews.
- ApplicationSet RollingSync intentionally forces auto-sync **disabled** on generated Applications (the controller logs warnings if any have auto-sync enabled). RollingSync drives sync via OutOfSync detection, not auto-sync.
- The `Replace=true` sync option is destructive on `StatefulSet`, `Service`, `PersistentVolumeClaim`, and any resource with finalizers. Argo CD's default three-way merge (or server-side apply on newer versions) is safer.
- Argo CD RBAC granular actions (e.g., `action/apps/Deployment/restart`, `action/argoproj.io/Rollout/abort`) shipped in v2.8+. Older policies that don't list these still work but won't grant the action — operators may discover gaps after upgrade.
- The `requireOverridePrivilegeForRevisionSync: true` flag in `argocd-cm` requires explicit `override` permission to sync to a non-tracked revision (e.g., a branch instead of HEAD of the configured target). This blocks easy ad-hoc syncs that bypass Git review.
- Argo CD Autopilot's bootstrap repo manages Argo CD itself — changes to that repo can disable RBAC, weaken AppProject scopes, or rotate the admin password. Treat the Autopilot repo as a tier-0 control surface.
- The Argo CD Agent (argocd-agent) introduces a hub-and-spoke topology where the central Argo CD installation does not hold cluster credentials for spoke clusters; agents connect outbound. Different threat model from the classic direct-cluster registration.
