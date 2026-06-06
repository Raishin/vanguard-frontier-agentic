# Rollback Playbook: Azure Live AKS Rollout Guard

## Immediate rollback — undo to previous ReplicaSet

```bash
# Pause the rollout first to stop further progress
kubectl rollout pause deployment/<deployment-name> -n <namespace-name>

# Check rollout history to identify the target revision
kubectl rollout history deployment/<deployment-name> -n <namespace-name>

# Undo to the immediately prior revision
kubectl rollout undo deployment/<deployment-name> -n <namespace-name>

# Or undo to a specific revision
kubectl rollout undo deployment/<deployment-name> -n <namespace-name> --to-revision=<N>
```

## Verify rollback success

```bash
kubectl rollout status deployment/<deployment-name> -n <namespace-name>
kubectl get pods -n <namespace-name> -o wide
kubectl describe deployment <deployment-name> -n <namespace-name> | grep -A 5 "Conditions:"
```

## Rollback limitations

- `kubectl rollout undo` reverts the pod template spec only (image, env, volumes).
- It does NOT revert ConfigMaps, Secrets, PVCs, or Service endpoint changes.
- If a schema migration ran as an init container, the rollback will reuse the new schema.
- HPA target replicas and PDB settings are not reverted by `rollout undo`.

## Escalation path

1. If rollback leaves pods in `CrashLoopBackOff`: check logs with `kubectl logs <pod-name> -n <namespace-name> --previous`
2. If node is under memory pressure: drain the node with `kubectl drain <node-name> --ignore-daemonsets`
3. If the cluster is unresponsive: escalate to AKS support via Azure portal → cluster → Support + troubleshooting
