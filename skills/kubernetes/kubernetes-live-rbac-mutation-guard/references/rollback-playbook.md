# Rollback Playbook: Kubernetes Live RBAC Mutation Guard

RBAC changes are additive and persistent. There is no built-in undo. Rollback means either deleting the new object or restoring the previous state from the captured baseline YAML.

## Rollback: delete a newly created Role, ClusterRole, binding

```bash
# Delete a Role
kubectl delete role <ROLE_NAME> -n <NAMESPACE>

# Delete a ClusterRole
kubectl delete clusterrole <CLUSTERROLE_NAME>

# Delete a RoleBinding
kubectl delete rolebinding <BINDING_NAME> -n <NAMESPACE>

# Delete a ClusterRoleBinding
kubectl delete clusterrolebinding <BINDING_NAME>
```

## Rollback: restore a modified object to its previous state

If the object was modified (not newly created), restore from the pre-mutation YAML backup:

```bash
kubectl apply -f rbac-backup-clusterrole-<TIMESTAMP>.yaml
```

Remove `resourceVersion` and `uid` from the backup YAML if you get conflict errors — strip only those fields, leave all others intact.

## Verify rollback took effect

```bash
# Confirm permissions are revoked for the affected ServiceAccount
kubectl auth can-i <verb> <resource> \
  --as=system:serviceaccount:<NAMESPACE>:<SERVICE_ACCOUNT> \
  -n <NAMESPACE>
# Should return "no"

# Confirm the binding no longer lists the principal
kubectl get clusterrolebindings -o wide | grep <BINDING_NAME>
```

## Assess dependent workload impact after deletion

Before deleting a binding, confirm which pods rely on it:

```bash
# Find pods using the affected ServiceAccount
kubectl get pods --all-namespaces \
  -o custom-columns='NAMESPACE:.metadata.namespace,NAME:.metadata.name,SA:.spec.serviceAccountName' \
  | grep <SERVICE_ACCOUNT_NAME>
```

If running pods use the deleted binding, they will lose API access on next token refresh or pod restart. Plan a maintenance window or notify the owning team before deletion.

## What cannot be rolled back

- API calls already made by the principal during the window the binding was active cannot be undone.
- Secrets read, ConfigMaps viewed, or resources created/deleted during the window must be investigated separately via Kubernetes audit logs.
- To review audit logs: check cluster audit log backend (CloudWatch, Stackdriver, Azure Monitor, or OCI Logging depending on distribution).
