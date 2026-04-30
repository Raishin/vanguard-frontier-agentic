# OKE Rollout — Rollback Playbook

## Option 1: kubectl rollback (in-cluster, immediate)

```bash
kubectl rollout undo deployment/<DEPLOY_NAME> -n <NAMESPACE>
kubectl rollout status deployment/<DEPLOY_NAME> -n <NAMESPACE>
```

## Option 2: Blue-green — switch service selector back to stable

```bash
kubectl patch service <SERVICE_NAME> -n <NAMESPACE> \
  -p '{"spec":{"selector":{"version":"<STABLE_VERSION>"}}}'

# Confirm traffic is on stable
kubectl get svc <SERVICE_NAME> -n <NAMESPACE> -o jsonpath='{.spec.selector}'
```

## Option 3: OCI DevOps — re-run previous successful deployment

```bash
# Find last successful deployment
oci devops deployment list \
  --pipeline-id <PIPELINE_OCID> \
  --query 'data.items[?contains("lifecycle-state", `SUCCEEDED`)][0].id'

oci devops deployment create-single-deploy-stage-deployment \
  --deploy-pipeline-id <PIPELINE_OCID> \
  --deploy-stage-id <STABLE_STAGE_OCID> \
  --display-name "rollback-$(date +%Y%m%dT%H%M%S)"
```

## Option 4: Node pool scale-down (if node-level instability is the root cause)

```bash
oci ce node-pool update \
  --node-pool-id <NODE_POOL_OCID> \
  --node-config-details '{"size": <PREVIOUS_SIZE>}'
```

## Verify

```bash
kubectl get pods -n <NAMESPACE> -l app=<APP_LABEL>
kubectl top pods -n <NAMESPACE>
```
