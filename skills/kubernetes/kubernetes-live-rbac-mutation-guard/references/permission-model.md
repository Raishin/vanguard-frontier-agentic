# Permission Model: Kubernetes Live RBAC Mutation Guard

## Privilege escalation verbs — always high severity

Kubernetes reserves three verbs specifically to prevent privilege escalation. Any Role that grants these bypasses the escalation protection and allows the holder to exceed their own permission ceiling:

| Verb | On resource | Effect |
|---|---|---|
| `escalate` | `clusterroles`, `roles` | Grants permissions the subject does not hold |
| `bind` | `clusterroles`, `roles`, `clusterrolebindings`, `rolebindings` | Creates bindings to roles the subject is not bound to |
| `impersonate` | `users`, `groups`, `serviceaccounts` | Acts as any other identity — bypasses all authentication controls |

**Block immediately. Require CISO-level or platform-team sign-off before approving any of these.**

## High-severity resource grants

| Resource | Verb | Risk |
|---|---|---|
| `secrets` | `get`, `list` at ClusterRole | Read every secret cluster-wide |
| `pods/exec` | `create` | Interactive shell on any pod |
| `pods/attach` | `create` | Same as exec — interactive shell |
| `pods/portforward` | `create` | Tunnel arbitrary TCP to pod ports |
| `nodes/proxy` | `get`, `create` | Access kubelet API on every node (cluster-admin equivalent for node ops) |
| `clusterroles` | `create`, `update` | Create or expand roles — potential escalation |
| `clusterrolebindings` | `create`, `update` | Grant any role to any principal cluster-wide |

## Least-privilege patterns for common workload scenarios

### Read-only workload monitoring (namespace-scoped)
```yaml
rules:
- apiGroups: [""]
  resources: ["pods", "services", "endpoints"]
  verbs: ["get", "list", "watch"]
- apiGroups: ["apps"]
  resources: ["deployments", "replicasets"]
  verbs: ["get", "list", "watch"]
```

### CI/CD deploy service account (namespace-scoped, not cluster-wide)
```yaml
rules:
- apiGroups: ["apps"]
  resources: ["deployments"]
  verbs: ["get", "list", "patch", "update"]
- apiGroups: [""]
  resources: ["configmaps"]
  verbs: ["get", "list", "create", "update"]
```

### Operator with CRD management (namespace-scoped preferred; cluster only if CRDs are global)
```yaml
rules:
- apiGroups: ["mygroup.io"]
  resources: ["myresources"]
  verbs: ["get", "list", "watch", "create", "update", "patch", "delete"]
# Never add * verbs or * resources even for operators
```

## Scope decision tree

```
Does the workload access resources across multiple namespaces?
  YES → ClusterRole + RoleBinding per namespace (not ClusterRoleBinding)
  NO  → Role in its namespace + RoleBinding in its namespace

Does the workload access cluster-scoped resources (Nodes, PersistentVolumes, Namespaces)?
  YES → ClusterRole required; bind with ClusterRoleBinding only if truly cluster-wide
  NO  → Namespace-scoped Role is always preferred
```

## Minimum caller permissions for RBAC mutation operations

The agent or human performing RBAC mutations should hold only:
```
create/update/delete on roles, clusterroles, rolebindings, clusterrolebindings
```
They should NOT hold `escalate` or `bind` — the mutation guard's job is to prevent those grants, not hold them.
