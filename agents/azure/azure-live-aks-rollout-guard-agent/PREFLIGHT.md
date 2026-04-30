# AKS Rollout — Preflight Commands

## 1. Confirm cluster identity and version

```bash
az aks show \
  --resource-group <TARGET_RG> \
  --name <CLUSTER_NAME> \
  --query "{k8sVersion:kubernetesVersion, state:provisioningState, fqdn:fqdn}"
```

## 2. Fetch user-level kubeconfig

```bash
az aks get-credentials \
  --resource-group <TARGET_RG> \
  --name <CLUSTER_NAME> \
  --overwrite-existing
kubectl config current-context
```

## 3. Current rollout status (before apply)

```bash
kubectl rollout status deployment/<DEPLOY_NAME> -n <NAMESPACE> --timeout=30s || true
```

## 4. Audit PodDisruptionBudget

```bash
kubectl get pdb -n <NAMESPACE> -o wide
```

Fail-fast if any PDB has `ALLOWED DISRUPTIONS = 0` and the rollout requires restarts.

## 5. Audit rolling-update strategy

```bash
kubectl describe deployment <DEPLOY_NAME> -n <NAMESPACE> \
  | grep -A 5 "RollingUpdateStrategy"
```

## 6. Check unhealthy pods before advancing

```bash
kubectl get pods -n <NAMESPACE> -l app=<APP_LABEL> \
  --field-selector="status.phase!=Running" -o wide
```
