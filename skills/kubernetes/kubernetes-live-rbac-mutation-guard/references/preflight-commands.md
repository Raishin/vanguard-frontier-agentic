# Preflight Commands: Kubernetes Live RBAC Mutation Guard

Run all of these before applying any RBAC mutation to a live cluster.

## 1. Confirm active cluster context and caller identity

```bash
kubectl config current-context
kubectl config view --minify --output 'jsonpath={.clusters[0].name}'
kubectl auth whoami   # Kubernetes 1.28+; shows current user/SA
# Older clusters:
kubectl get serviceaccount -n kube-system default -o jsonpath='{.metadata.name}'
```

## 2. Capture current state of target object (MANDATORY rollback baseline)

```bash
# Role
kubectl get role <ROLE_NAME> -n <NAMESPACE> -o yaml > rbac-backup-role-$(date +%Y%m%d-%H%M%S).yaml

# ClusterRole
kubectl get clusterrole <CLUSTERROLE_NAME> -o yaml > rbac-backup-clusterrole-$(date +%Y%m%d-%H%M%S).yaml

# RoleBinding
kubectl get rolebinding <BINDING_NAME> -n <NAMESPACE> -o yaml > rbac-backup-rolebinding-$(date +%Y%m%d-%H%M%S).yaml

# ClusterRoleBinding
kubectl get clusterrolebinding <BINDING_NAME> -o yaml > rbac-backup-clusterrolebinding-$(date +%Y%m%d-%H%M%S).yaml
```

## 3. Check what permissions the proposed Role or ClusterRole would grant

```bash
# Simulate permissions for a ServiceAccount after the proposed binding
kubectl auth can-i --list \
  --as=system:serviceaccount:<NAMESPACE>:<SERVICE_ACCOUNT> \
  -n <NAMESPACE>

# Check a specific permission
kubectl auth can-i <verb> <resource> \
  --as=system:serviceaccount:<NAMESPACE>:<SERVICE_ACCOUNT> \
  -n <NAMESPACE>
```

## 4. Check whether a ClusterRole already exists before creating a new one

```bash
kubectl get clusterrole <NAME> -o yaml 2>/dev/null && echo "EXISTS" || echo "NOT FOUND"
```

## 5. Find all subjects currently bound to a Role or ClusterRole (blast radius before deletion)

```bash
# Who is bound to a ClusterRole cluster-wide?
kubectl get clusterrolebindings \
  -o custom-columns='NAME:.metadata.name,ROLE:.roleRef.name,SUBJECTS:.subjects[*].name' \
  | grep <CLUSTERROLE_NAME>

# Who is bound to a Role in a namespace?
kubectl get rolebindings -n <NAMESPACE> \
  -o custom-columns='NAME:.metadata.name,ROLE:.roleRef.name,SUBJECTS:.subjects[*].name' \
  | grep <ROLE_NAME>
```

## 6. Check whether the proposed role grants escalation verbs

```bash
# Review the proposed RBAC YAML for dangerous verbs
kubectl apply --dry-run=client -f proposed-role.yaml

# Grep the YAML for escalation verbs before apply
grep -E '"\*"|escalate|bind|impersonate' proposed-role.yaml
```

## 7. Verify `automountServiceAccountToken` on the target ServiceAccount

```bash
kubectl get serviceaccount <SA_NAME> -n <NAMESPACE> \
  -o jsonpath='{.automountServiceAccountToken}'
# Empty or "true" means tokens are auto-mounted. Verify pods using this SA actually need API access.
```
