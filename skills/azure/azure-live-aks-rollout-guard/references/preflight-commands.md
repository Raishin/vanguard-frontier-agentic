# Preflight Commands: Azure Live AKS Rollout Guard

Use shell variables for examples instead of raw identifiers. Populate them from an approved change record or already configured shell context; never paste tenant, subscription, resource, or secret values into chat.

## Evidence-variable convention

Variables such as $AZURE_RESOURCE_GROUP_NAME, $APP_SERVICE_APP_NAME, or $KEY_VAULT_NAME are local operator placeholders. Do not commit real values, and redact them from shared evidence unless the change record explicitly allows disclosure.

Run these commands before any AKS rollout mutation. Paste sanitized output as evidence.

## 1. Confirm identity and cluster target

```bash
az account show --query "{subscriptionName:name, user:user.name}"
az aks show -g $AZURE_RESOURCE_GROUP_NAME -n $AKS_CLUSTER_NAME \
  --query "{provisioningState:provisioningState, kubernetesVersion:kubernetesVersion, fqdn:fqdn}"
```

## 2. Fetch user-level kubeconfig

```bash
az aks get-credentials -g $AZURE_RESOURCE_GROUP_NAME -n $AKS_CLUSTER_NAME --overwrite-existing
kubectl config current-context
```

## 3. Audit PodDisruptionBudgets in target namespace

```bash
kubectl get pdb -n $KUBERNETES_NAMESPACE -o wide
# minAvailable or maxUnavailable must leave at least one pod available during rollout
```

## 4. Check current deployment rollout status

```bash
kubectl rollout status deployment/$KUBERNETES_DEPLOYMENT_NAME -n $KUBERNETES_NAMESPACE
kubectl get deployment $KUBERNETES_DEPLOYMENT_NAME -n $KUBERNETES_NAMESPACE -o jsonpath='{.spec.strategy}'
```

## 5. Verify node readiness and resource headroom

```bash
kubectl get nodes -o wide
kubectl top nodes
kubectl get pods -n $KUBERNETES_NAMESPACE -o wide
```

## 6. Confirm maxSurge / maxUnavailable strategy

```bash
kubectl get deployment $KUBERNETES_DEPLOYMENT_NAME -n $KUBERNETES_NAMESPACE \
  -o jsonpath='{.spec.strategy.rollingUpdate}'
# maxUnavailable=0 is safest for production; maxSurge=1 is a conservative default
```

## 7. Check HorizontalPodAutoscaler (if present)

```bash
kubectl get hpa -n $KUBERNETES_NAMESPACE
# HPA minReplicas must exceed PDB minAvailable or the rollout will deadlock
```
