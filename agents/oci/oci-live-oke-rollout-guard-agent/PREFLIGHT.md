# OKE Rollout — Preflight Commands

## 1. Confirm OKE cluster state

```bash
oci ce cluster get \
  --cluster-id <CLUSTER_OCID> \
  --query 'data.{name:name, state:"lifecycle-state", version:"kubernetes-version", endpoint:endpoints}'
```

## 2. Check DevOps pipeline status

```bash
oci devops deploy-pipeline get \
  --pipeline-id <PIPELINE_OCID> \
  --query 'data.{name:name, state:"lifecycle-state"}'

# List deployment stages with types
oci devops deploy-stage list \
  --pipeline-id <PIPELINE_OCID> \
  --query 'data.items[].{name:"display-name", type:"deploy-stage-type", id:id}'
```

## 3. Fetch kubeconfig and confirm context

```bash
oci ce cluster create-kubeconfig \
  --cluster-id <CLUSTER_OCID> \
  --file $HOME/.kube/oci-prod-config \
  --region <REGION> \
  --token-version 2.0.0
export KUBECONFIG=$HOME/.kube/oci-prod-config
kubectl config current-context
```

## 4. Audit rollout strategy and PDB

```bash
kubectl rollout status deployment/<DEPLOY_NAME> -n <NAMESPACE> --timeout=30s || true
kubectl get pdb -n <NAMESPACE> -o wide
kubectl describe deployment <DEPLOY_NAME> -n <NAMESPACE> | grep -A 5 "RollingUpdateStrategy"
```

## 5. Blue-green: confirm stable service selector before swap

```bash
kubectl get svc <SERVICE_NAME> -n <NAMESPACE> \
  -o jsonpath='{.spec.selector}' | python3 -m json.tool
```
