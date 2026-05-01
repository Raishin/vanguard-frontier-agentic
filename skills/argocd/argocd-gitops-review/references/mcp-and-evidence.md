# Evidence Path and Tooling

## Evidence path

1. Prefer live cluster evidence when a Kubernetes MCP server, `kubectl`, or the `argocd` CLI is available against the Argo CD control-plane cluster.
2. Fall back to official Argo CD documentation (argo-cd.readthedocs.io) and the upstream argo-cd GitHub repository when live inspection is unavailable.
3. Ask only for sanitized `Application` / `AppProject` / `ApplicationSet` YAML, the redacted `argocd-cm` and `argocd-rbac-cm` ConfigMaps, and `argocd app history` output when current-state proof matters.
4. Label conclusions as `live evidence`, `documentation-based`, `sanitized user evidence`, or `inference`.

## Useful live-evidence commands

```shell
# All Applications, AppProjects, and ApplicationSets in the argocd namespace
kubectl -n argocd get applications,appprojects,applicationsets -o yaml

# Detailed Application status (sync, health, lastSyncRevision)
kubectl -n argocd get application <app-name> -o yaml
argocd app get <app-name>
argocd app history <app-name>

# Argo CD configuration (the global config knobs)
kubectl -n argocd get configmap argocd-cm -o yaml
kubectl -n argocd get configmap argocd-rbac-cm -o yaml
kubectl -n argocd get configmap argocd-cmd-params-cm -o yaml

# RBAC effective policy
argocd account list
argocd account get-user-info <user>

# Cluster registrations (every destination cluster has its own Secret)
kubectl -n argocd get secrets -l argocd.argoproj.io/secret-type=cluster -o yaml

# Sync windows on an AppProject
kubectl -n argocd get appproject <project> -o jsonpath='{.spec.syncWindows}'

# Argo CD Agent (hub-and-spoke deployments)
kubectl -n argocd get agents -o yaml
```

## Argo CD install state to confirm before review

- Argo CD version (`kubectl -n argocd get deploy argocd-server -o jsonpath='{.spec.template.spec.containers[0].image}'`) — sync impersonation, RBAC granular actions, and ApplicationSet RollingSync arrived in different versions.
- `application.sync.impersonation.enabled` in `argocd-cm` — `false` (default) means every sync runs as the controller's ServiceAccount on every destination.
- `application.sync.requireOverridePrivilegeForRevisionSync` in `argocd-cm` — `true` requires explicit override permission for ad-hoc revision syncs.
- `webhook.maxPayloadSizeMB` in `argocd-cm` — large Helm value files may exceed the default.
- Whether Argo CD Agent (argocd-agent) is in use for hub-and-spoke multi-cluster — different security model.
- Whether Argo CD Autopilot manages Argo CD itself via GitOps — change review must include the Autopilot repo.

## Sanitization rules

- Never request kubeconfig contents, cluster Secret contents, repository SSH keys, or webhook signing secrets in chat.
- Replace identifiable cluster URLs and namespaces with placeholders unless the user provides them and confirms it is safe to use them.
- Do not print Git repository tokens, OCI registry tokens, or Helm OCI credentials.
