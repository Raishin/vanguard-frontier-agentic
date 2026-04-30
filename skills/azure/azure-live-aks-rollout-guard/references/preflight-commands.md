# Preflight Commands: Azure Live AKS Rollout Guard

Run these commands before any AKS rollout mutation. Paste sanitized output as evidence.

## 1. Confirm identity and cluster target

```bash
az account show --query "{subscription:id, name:name, user:user.name}"
az aks show -g <RESOURCE_GROUP> -n <CLUSTER_NAME> \
  --query "{provisioningState:provisioningState, kubernetesVersion:kubernetesVersion, fqdn:fqdn}"
```

## 2. Fetch user-level kubeconfig

```bash
az aks get-credentials -g <RESOURCE_GROUP> -n <CLUSTER_NAME> --overwrite-existing
kubectl config current-context
```

## 3. Audit PodDisruptionBudgets in target namespace

```bash
kubectl get pdb -n <NAMESPACE> -o wide
# minAvailable or maxUnavailable must leave at least one pod available during rollout
```

## 4. Check current deployment rollout status

```bash
kubectl rollout status deployment/<DEPLOYMENT_NAME> -n <NAMESPACE>
kubectl get deployment <DEPLOYMENT_NAME> -n <NAMESPACE> -o jsonpath='{.spec.strategy}'
```

## 5. Verify node readiness and resource headroom

```bash
kubectl get nodes -o wide
kubectl top nodes
kubectl get pods -n <NAMESPACE> -o wide
```

## 6. Confirm maxSurge / maxUnavailable strategy

```bash
kubectl get deployment <DEPLOYMENT_NAME> -n <NAMESPACE> \
  -o jsonpath='{.spec.strategy.rollingUpdate}'
# maxUnavailable=0 is safest for production; maxSurge=1 is a conservative default
```

## 7. Check HorizontalPodAutoscaler (if present)

```bash
kubectl get hpa -n <NAMESPACE>
# HPA minReplicas must exceed PDB minAvailable or the rollout will deadlock
```
