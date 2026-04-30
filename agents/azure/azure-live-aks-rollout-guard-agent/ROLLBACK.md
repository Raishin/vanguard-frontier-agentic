# AKS Rollout — Rollback Playbook

## Option 1: Immediate undo (reverts to previous ReplicaSet)

```bash
kubectl rollout undo deployment/<DEPLOY_NAME> -n <NAMESPACE>
kubectl rollout status deployment/<DEPLOY_NAME> -n <NAMESPACE>
```

## Option 2: Undo to a specific revision

```bash
# List revision history
kubectl rollout history deployment/<DEPLOY_NAME> -n <NAMESPACE>

# Undo to specific revision
kubectl rollout undo deployment/<DEPLOY_NAME> \
  --to-revision=<REVISION_NUMBER> \
  -n <NAMESPACE>
```

## Option 3: Pause a stuck rollout mid-flight

```bash
kubectl rollout pause deployment/<DEPLOY_NAME> -n <NAMESPACE>
# Inspect, patch if needed, then resume or undo
kubectl rollout resume deployment/<DEPLOY_NAME> -n <NAMESPACE>
```

## Verify rollback completed

```bash
kubectl rollout status deployment/<DEPLOY_NAME> -n <NAMESPACE>
kubectl get pods -n <NAMESPACE> -l app=<APP_LABEL>
kubectl top pods -n <NAMESPACE>
```
