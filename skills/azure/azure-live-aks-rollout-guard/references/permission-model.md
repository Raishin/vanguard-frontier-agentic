# Permission Model: Azure Live AKS Rollout Guard

## Azure RBAC (control plane — cluster credential access)

```json
{
  "Name": "AKS Rollout Guard",
  "IsCustom": true,
  "Description": "Read AKS cluster state and fetch user-level kubeconfig. No cluster admin rights.",
  "Actions": [
    "Microsoft.ContainerService/managedClusters/read",
    "Microsoft.ContainerService/managedClusters/listClusterUserCredential/action"
  ],
  "NotActions": [
    "Microsoft.ContainerService/managedClusters/delete",
    "Microsoft.ContainerService/managedClusters/agentPools/write"
  ],
  "AssignableScopes": [
    "/subscriptions/<SUBSCRIPTION_ID>/resourceGroups/<TARGET_RG>/providers/Microsoft.ContainerService/managedClusters/<CLUSTER_NAME>"
  ]
}
```

`listClusterUserCredential` grants a user-level kubeconfig. What the user can do inside
the cluster is governed by AKS-integrated Entra ID RBAC, not this control-plane role.

## Kubernetes RBAC (data plane — in-cluster namespace scope)

Bind the operator's Entra ID identity to a namespace-scoped Role (never ClusterRole):

```yaml
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: rollout-guard
  namespace: <TARGET_NAMESPACE>
rules:
- apiGroups: ["apps"]
  resources: ["deployments", "replicasets"]
  verbs: ["get", "list", "watch", "patch", "update"]
- apiGroups: [""]
  resources: ["pods", "pods/log"]
  verbs: ["get", "list", "watch"]
- apiGroups: ["policy"]
  resources: ["poddisruptionbudgets"]
  verbs: ["get", "list"]
```

## Do not assign

- `Azure Kubernetes Service Cluster Admin Role` — full cluster admin kubeconfig
- `cluster-admin` ClusterRoleBinding in Kubernetes
- `Microsoft.ContainerService/managedClusters/agentPools/delete`
- Subscription-level Contributor for routine rollout operations
