# Rollback Playbook: OCI Live OKE Rollout Guard

## Rollback a Kubernetes deployment to the previous revision

```bash
# Pause rollout immediately
kubectl rollout pause deployment/<DEPLOYMENT_NAME> -n <NAMESPACE>

# Check rollout history
kubectl rollout history deployment/<DEPLOYMENT_NAME> -n <NAMESPACE>

# Undo to previous revision
kubectl rollout undo deployment/<DEPLOYMENT_NAME> -n <NAMESPACE>

# Or undo to a specific revision
kubectl rollout undo deployment/<DEPLOYMENT_NAME> -n <NAMESPACE> --to-revision=<N>

# Verify
kubectl rollout status deployment/<DEPLOYMENT_NAME> -n <NAMESPACE>
```

## Cancel an in-flight DevOps pipeline deployment

```bash
oci devops deployment list \
  --deployment-pipeline-id <PIPELINE_OCID> \
  --query "data[?\"lifecycle-state\"=='IN_PROGRESS'].{id:id,displayName:\"display-name\"}"

oci devops deployment cancel --deployment-id <DEPLOYMENT_OCID> --force
```

## Rollback a node pool version upgrade

```bash
oci ce node-pool update \
  --node-pool-id <NODE_POOL_OCID> \
  --kubernetes-version <PREVIOUS_VERSION>
```

## Rollback limitations

- `kubectl rollout undo` reverts the pod template spec only — does not revert ConfigMaps, Secrets, or database schema migrations.
- DevOps pipeline deployment cancellation stops future stages but does not undo already-applied Kubernetes resources.
- Node pool version downgrade is not supported by OCI — you can only go to an equal or newer Kubernetes version.
- If the cluster upgrade (control plane version) was applied, it cannot be rolled back.
