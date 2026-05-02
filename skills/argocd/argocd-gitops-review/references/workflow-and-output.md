# Workflow and Output Contract

## Workflow

### Step 1 — Identify the target and the surrounding AppProject

1. Confirm the kind: `Application`, `AppProject`, `ApplicationSet`, or a global ConfigMap (`argocd-cm`, `argocd-rbac-cm`).
2. For an `Application`, locate the `spec.project` reference and review the `AppProject` first — the AppProject defines the boundary the Application operates within.
3. For an `AppProject`, list every `Application` referencing it (`kubectl -n argocd get applications -o jsonpath='{range .items[?(@.spec.project=="<project>")]}{.metadata.name}{"\n"}{end}'`).
4. For an `ApplicationSet`, identify the generator type (`list`, `cluster`, `git`, `matrix`, `merge`, `pullRequest`, `scmProvider`) and the `spec.template`.

### Step 2 — Audit the AppProject blast radius

The AppProject defines four boundary surfaces. Each is a potential blast-radius finding:

1. **`sourceRepos`** — the Git or Helm repos this project may pull from. `['*']` means any repo. Recommended: explicit list.
2. **`destinations`** — the (cluster, namespace) tuples this project may deploy to. `[{server: '*', namespace: '*'}]` means anywhere. Recommended: explicit cluster URLs and namespace allowlist (or `namespace: 'team-*'` for multi-tenant patterns).
3. **`clusterResourceWhitelist`** — cluster-scoped resources this project may manage. Empty or `['*/*']` means any cluster-scoped resource (including ClusterRoleBindings, Namespaces). Recommended: empty for application projects; explicit list for platform projects.
4. **`namespaceResourceBlacklist`** — namespace-scoped resources this project may NOT manage. Recommended: include `[{group: 'rbac.authorization.k8s.io', kind: '*'}]` for application projects to prevent applications from binding their own RBAC.

Reference: [AppProject in declarative setup](https://argo-cd.readthedocs.io/en/stable/operator-manual/declarative-setup/).

### Step 3 — Audit sync identity (the most under-reviewed control)

Three sync identity modes exist; pick one and verify:

1. **Controller default** — Argo CD controller's ServiceAccount on the destination cluster. Default is broad (cluster-admin in many installs). **Critical finding** if production Applications use this without an audit trail of what the controller can do.
2. **Sync impersonation** (preferred for least privilege) — `application.sync.impersonation.enabled: true` in `argocd-cm` plus `destinationServiceAccounts` on the AppProject. Each Application syncs as a per-namespace ServiceAccount with scoped RBAC. See the [sync impersonation proposal](https://argo-cd.readthedocs.io/en/stable/proposals/decouple-application-sync-user-using-impersonation/).
3. **Cluster credentials** (legacy multi-cluster) — Argo CD has its own bearer token for each registered cluster. Rotate regularly.

Stress-tests:

- An Application with `spec.destination.namespace: kube-system` plus controller-default identity = sync runs as cluster-admin in kube-system.
- An AppProject with `destinationServiceAccounts` listing `defaultServiceAccount: 'default'` = effectively no impersonation; the default SA is always present.

### Step 4 — Audit the drift-handling posture

`spec.syncPolicy.automated` controls whether Argo CD reconciles drift. Three flags govern blast radius:

1. **`automated: {}` (auto-sync)** — every Git commit triggers a sync. Production-safe only with `syncWindows` and a tested CI gate.
2. **`automated.prune: true`** — resources removed from Git are deleted from the cluster. **Critical** without a rollback runbook: a misconfigured commit deletes prod resources.
3. **`automated.selfHeal: true`** — manual cluster changes are reverted on the next sync. Combined with `prune`, divergence becomes a hard reset to Git state.

Stress-tests:

- `automated.prune: true` on a `StatefulSet` Application = deletion cascades to PVCs (if `persistentVolumeClaimRetentionPolicy.whenDeleted: Delete`). Data loss path.
- `automated.selfHeal: true` on an Application managing CRDs from a third-party operator = the operator's runtime status updates may be reverted as drift.
- `automated` with no `syncWindow` covering deploy-freeze periods = a freeze window can be bypassed by a Git commit.

Reference: [Auto-Sync](https://argo-cd.readthedocs.io/en/stable/user-guide/auto_sync/) and [Sync Windows](https://argo-cd.readthedocs.io/en/stable/user-guide/sync_windows/).

### Step 5 — Audit `syncOptions` for stateful or sensitive resources

`spec.syncPolicy.syncOptions` overrides default sync behavior. Flag these as findings:

- **`Replace=true`** — Argo CD deletes the resource and recreates it instead of patching. For `StatefulSet`, `PersistentVolume`, `PersistentVolumeClaim`, `Service` (ClusterIP rotation), `ConfigMap` consumed by hot-reload — this is data loss or downtime.
- **`Force=true`** — passes `--force` to `kubectl apply`. Disables conflict detection.
- **`ServerSideApply=false`** when Argo CD's default is server-side apply on newer versions — falls back to client-side three-way merge, which can re-introduce drift loops.
- **`PruneLast=true`** missing on Applications that delete resources — pruning happens before resource creation, briefly leaving the namespace in an unhealthy state.
- **`CreateNamespace=true`** with no namespace finalizer or RBAC scope — creates namespaces outside AppProject `destinations`.

### Step 6 — Audit `ApplicationSet` generators

ApplicationSet generators expand into multiple Applications. Risk surface depends on generator type:

- **`list` generator** — explicit list of clusters/parameters. Lowest risk.
- **`cluster` generator** — generates an Application for every registered cluster matching a label selector. **Critical** when the selector is empty (`{}`) or matches all clusters — a new cluster automatically receives the workload before review.
- **`git` generator** — generates an Application for every directory or file pattern in a Git repo. Risk: a malicious or accidental commit adds a new directory and triggers a new Application.
- **`matrix` and `merge` generators** — combine other generators. Risk multiplies.
- **`pullRequest` generator** — generates Applications for open PRs. Risk: any PR can trigger an ephemeral deployment with the PR's manifests.
- **`scmProvider` generator** — generates Applications for every repo in an org. Risk: org-wide auto-onboarding.

Reference: [ApplicationSet Generators](https://argo-cd.readthedocs.io/en/stable/operator-manual/applicationset/Generators/) and [Progressive Syncs (RollingSync)](https://argo-cd.readthedocs.io/en/stable/operator-manual/applicationset/Progressive-Syncs/).

Stress-tests:

- ApplicationSet with `cluster` generator + no selector + auto-sync = every cluster auto-onboarded in seconds.
- ApplicationSet with `pullRequest` generator + no namespace isolation = PRs deploy to shared namespaces.
- ApplicationSet with `goTemplate: true` and unsanitized template inputs = template injection if PR titles are templated into manifests.

### Step 7 — Audit `argocd-rbac-cm` policy

The Argo CD RBAC ConfigMap (`argocd-rbac-cm`) defines who can do what in the Argo CD UI/CLI/API. Check:

1. The default role (`policy.default`) — `role:readonly` is safe; `role:admin` is wrong.
2. Specific actions on resources — newer Argo CD versions ship granular actions like `action/apps/Deployment/restart` or `action/argoproj.io/Rollout/abort`. Each granted action should map to a real on-call runbook.
3. RBAC subject scopes — `g, <group>, role:admin` on broad SSO groups is a finding.

Reference: [Argo CD RBAC](https://argo-cd.readthedocs.io/en/stable/operator-manual/rbac/).

### Step 8 — Multi-cluster (Argo CD Agent) topology

If Argo CD Agent is in use:

- The control plane stores `Application` specs; each spoke runs an agent that pulls assigned Applications.
- Each agent has its own credentials and authentication path — verify rotation cadence.
- Network path from spoke to hub must be authenticated and encrypted.

Reference: [argocd-agent](https://github.com/argoproj-labs/argocd-agent).

## Output

Return:

- **target**: `Application`, `AppProject`, `ApplicationSet`, or RBAC ConfigMap, with the project boundary,
- **evidence level**: `live evidence` / `documentation-based` / `sanitized user evidence` / `inference`,
- **sync identity**: controller default vs impersonated SA vs cluster credential, with judgment on least privilege,
- **blast radius**: `sourceRepos`, `destinations`, `clusterResourceWhitelist`, `namespaceResourceBlacklist` audit,
- **drift posture**: `automated.prune`, `automated.selfHeal`, sync windows, syncOptions concerns,
- **risk findings** (with severity: high / medium / low) — covering sync identity, blast radius, drift, ApplicationSet generators, RBAC,
- **safest next actions** with sample manifest changes,
- **rollback plan**: how to revert auto-sync, disable selfHeal, narrow AppProject scope without breaking running Applications,
- **assumptions and missing facts**.

## Security notes

- Never recommend `automated.prune: true` + `automated.selfHeal: true` on production Applications without a tested rollback runbook.
- Never recommend `AppProject` with `sourceRepos: ['*']` and `destinations: ['*']` for application projects. Platform projects may need this; document the justification.
- Never recommend disabling sync impersonation as a default in production after it has been enabled.
- Never request or print Argo CD admin tokens, repo SSH keys, or destination cluster bearer tokens.
