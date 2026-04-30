# Rollback Playbook: Azure Live AKS Rollout Guard

## Immediate rollback — undo to previous ReplicaSet

```bash
# Pause the rollout first to stop further progress
kubectl rollout pause deployment/<DEPLOYMENT_NAME> -n <NAMESPACE>

# Check rollout history to identify the target revision
kubectl rollout history deployment/<DEPLOYMENT_NAME> -n <NAMESPACE>

# Undo to the immediately prior revision
kubectl rollout undo deployment/<DEPLOYMENT_NAME> -n <NAMESPACE>

# Or undo to a specific revision
kubectl rollout undo deployment/<DEPLOYMENT_NAME> -n <NAMESPACE> --to-revision=<N>
```

## Verify rollback success

```bash
kubectl rollout status deployment/<DEPLOYMENT_NAME> -n <NAMESPACE>
kubectl get pods -n <NAMESPACE> -o wide
kubectl describe deployment <DEPLOYMENT_NAME> -n <NAMESPACE> | grep -A 5 "Conditions:"
```

## Rollback limitations

- `kubectl rollout undo` reverts the pod template spec only (image, env, volumes).
- It does NOT revert ConfigMaps, Secrets, PVCs, or Service endpoint changes.
- If a schema migration ran as an init container, the rollback will reuse the new schema.
- HPA target replicas and PDB settings are not reverted by `rollout undo`.

## Escalation path

1. If rollback leaves pods in `CrashLoopBackOff`: check logs with `kubectl logs <POD> -n <NAMESPACE> --previous`
2. If node is under memory pressure: drain the node with `kubectl drain <NODE> --ignore-daemonsets`
3. If the cluster is unresponsive: escalate to AKS support via Azure portal → cluster → Support + troubleshooting
