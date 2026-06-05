# Preflight Commands: Azure Live AKS Rollout Guard

Run these commands before any AKS rollout mutation. Paste sanitized output as evidence.

## 1. Confirm identity and cluster target

```bash
az account show --query "{subscriptionName:name, user:user.name}"
az aks show -g <resource-group-name> -n <cluster-name> \
  --query "{provisioningState:provisioningState, kubernetesVersion:kubernetesVersion, fqdn:fqdn}"
```

## 2. Fetch user-level kubeconfig

```bash
az aks get-credentials -g <resource-group-name> -n <cluster-name> --overwrite-existing
kubectl config current-context
```

## 3. Audit PodDisruptionBudgets in target namespace

```bash
kubectl get pdb -n <namespace-name> -o wide
# minAvailable or maxUnavailable must leave at least one pod available during rollout
```

## 4. Check current deployment rollout status

```bash
kubectl rollout status deployment/<deployment-name> -n <namespace-name>
kubectl get deployment <deployment-name> -n <namespace-name> -o jsonpath='{.spec.strategy}'
```

## 5. Verify node readiness and resource headroom

```bash
kubectl get nodes -o wide
kubectl top nodes
kubectl get pods -n <namespace-name> -o wide
```

## 6. Confirm maxSurge / maxUnavailable strategy

```bash
kubectl get deployment <deployment-name> -n <namespace-name> \
  -o jsonpath='{.spec.strategy.rollingUpdate}'
# maxUnavailable=0 is safest for production; maxSurge=1 is a conservative default
```

## 7. Check HorizontalPodAutoscaler (if present)

```bash
kubectl get hpa -n <namespace-name>
# HPA minReplicas must exceed PDB minAvailable or the rollout will deadlock
```
