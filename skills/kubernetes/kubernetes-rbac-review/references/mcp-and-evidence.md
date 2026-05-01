# Evidence Path and Tooling

## Evidence path

1. Prefer live cluster evidence when the active client exposes relevant Kubernetes MCP capabilities or a working `kubectl` context.
2. Fall back to official Kubernetes documentation when live inspection is unavailable, incomplete, or unsafe.
3. Ask only for sanitized RBAC YAML or `kubectl` output when current-state proof matters.
4. Label conclusions as `live evidence`, `documentation-based`, `sanitized user evidence`, or `inference`.

## Useful live-evidence commands

```shell
# List all RoleBindings in a namespace
kubectl get rolebindings -n <namespace> -o yaml

# List all ClusterRoleBindings
kubectl get clusterrolebindings -o yaml

# Check effective permissions for a ServiceAccount
kubectl auth can-i --list --as=system:serviceaccount:<namespace>:<sa-name>

# Check a specific verb/resource
kubectl auth can-i <verb> <resource> --as=system:serviceaccount:<namespace>:<sa-name> -n <namespace>

# Describe a Role or ClusterRole
kubectl describe role <name> -n <namespace>
kubectl describe clusterrole <name>
```

## Platform-agnostic execution

- Keep examples neutral with placeholders until the user's cluster context and toolchain are known.
- Do not request kubeconfig files, bearer tokens, service account JWT tokens, or cloud-provider credentials in chat.
- If a Kubernetes MCP server or kubectl is unavailable, say so and fall back to reviewing sanitized YAML provided by the user.
