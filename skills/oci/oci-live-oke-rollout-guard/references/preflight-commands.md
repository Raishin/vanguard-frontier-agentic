# Preflight Commands: OCI Live OKE Rollout Guard

Run these before any OKE rollout mutation. Paste sanitized output as evidence.

## 1. Confirm identity and compartment target

```bash
oci iam region-subscription list
oci ce cluster list \
  --compartment-id <COMPARTMENT_OCID> \
  --query "data[].{name:name,id:id,lifecycleState:\"lifecycle-state\",kubernetesVersion:\"kubernetes-version\"}"
```

## 2. Get cluster kubeconfig

```bash
oci ce cluster create-kubeconfig \
  --cluster-id <CLUSTER_OCID> \
  --file ~/.kube/config \
  --region <REGION> \
  --token-version 2.0.0
kubectl config current-context
```

## 3. Check node pool health

```bash
oci ce node-pool list \
  --cluster-id <CLUSTER_OCID> \
  --compartment-id <COMPARTMENT_OCID> \
  --query "data[].{name:name,id:id,lifecycleState:\"lifecycle-state\",quantityPerSubnet:\"quantity-per-subnet\"}"
kubectl get nodes -o wide
```

## 4. Check PodDisruptionBudgets

```bash
kubectl get pdb -n <NAMESPACE> -o wide
```

## 5. Check current deployment rollout status

```bash
kubectl rollout status deployment/<DEPLOYMENT_NAME> -n <NAMESPACE>
kubectl get deployment <DEPLOYMENT_NAME> -n <NAMESPACE> \
  -o jsonpath='{.spec.strategy.rollingUpdate}'
```

## 6. Verify DevOps pipeline approval stage is configured

```bash
oci devops deployment-pipeline list \
  --project-id <PROJECT_OCID> \
  --query "data[].{displayName:\"display-name\",id:id,lifecycleState:\"lifecycle-state\"}"
```
