# Workflow and Output Contract

## Workflow

1. Identify the target: namespace-scoped Role/RoleBinding or cluster-scoped ClusterRole/ClusterRoleBinding.
2. Identify the principal: ServiceAccount, user, or Group (including `system:` groups).
3. Prefer namespace-scoped Roles before ClusterRoles when the workload only operates in one namespace.
4. Challenge dangerous defaults:
   - `cluster-admin` ClusterRoleBinding for any non-infrastructure workload,
   - Wildcard verbs (`*`) or wildcard resources (`*`) in any Role or ClusterRole,
   - Wildcard API groups (`*`) that grant cross-group access,
   - Binding to the `default` ServiceAccount (shared blast radius),
   - `automountServiceAccountToken: true` (default) on pods that do not need API server access,
   - ClusterRoleBindings where a RoleBinding to a namespaced ClusterRole would suffice,
   - Aggregated ClusterRoles with labels that may attract unexpected rules from third-party operators.
5. Check whether RBAC controls reach sensitive resources: `secrets`, `pods/exec`, `pods/portforward`, `nodes`, `namespaces`, `clusterroles`, `clusterrolebindings`.
6. Stress-test operational hygiene:
   - prefer dedicated ServiceAccounts per workload over shared accounts,
   - prefer explicit `resources` lists over wildcards,
   - prefer explicit `verbs` lists (`get`, `list`, `watch`) over `*`,
   - challenge escalation paths: can the bound account create/update Roles or RoleBindings?

## Output

Return:

- current access summary,
- risk findings (with severity: high / medium / low),
- least-privilege alternative,
- validation commands or manifest corrections,
- assumptions and missing facts.

## Security notes

Do not suggest `cluster-admin` bindings or wildcard grants unless the user has explicitly justified the blast radius and there is no namespace-scoped alternative.
