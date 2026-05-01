# Evidence Path and Tooling

## Evidence path

1. Prefer live cluster evidence (`kubectl`) plus the cloud-provider's CLI (`aws`, `az`, `gcloud`) or MCP server when available.
2. Fall back to official documentation: Kubernetes ServiceAccount admin, AWS IRSA, Azure Workload Identity, GCP Workload Identity Federation.
3. Ask only for sanitized ServiceAccount, Pod, and trust policy YAML/JSON, plus the cluster's OIDC issuer URL.
4. Label conclusions as `live evidence`, `documentation-based`, `sanitized user evidence`, or `inference`.

## Useful live-evidence commands

```shell
# ServiceAccount with workload identity annotations
kubectl get serviceaccount -A -o yaml | grep -A2 -E 'eks\.amazonaws\.com/role-arn|azure\.workload\.identity/client-id|iam\.gke\.io/gcp-service-account'

# Pod's projected ServiceAccount token volume
kubectl get pod <pod> -n <ns> -o yaml | grep -A20 'projected:'

# Verify pod is consuming the projected token
kubectl exec -it <pod> -n <ns> -- ls -la /var/run/secrets/tokens/
kubectl exec -it <pod> -n <ns> -- cat /var/run/secrets/tokens/<audience-token>

# Cluster OIDC issuer (each cluster has one — IAM trusts it)
# AWS EKS:
aws eks describe-cluster --name <cluster> --query "cluster.identity.oidc.issuer" --output text
# Azure AKS:
az aks show --resource-group <rg> --name <cluster> --query "oidcIssuerProfile.issuerUrl" --output tsv
# GKE:
gcloud container clusters describe <cluster> --location <location> --format='value(workloadIdentityConfig.workloadPool)'

# Confirm there's no static credential alongside
kubectl exec -it <pod> -n <ns> -- env | grep -E 'AWS_ACCESS_KEY_ID|AZURE_CLIENT_SECRET|GOOGLE_APPLICATION_CREDENTIALS'
kubectl exec -it <pod> -n <ns> -- ls /var/run/secrets/

# AWS — view IAM role trust policy
aws iam get-role --role-name <role-name> --query 'Role.AssumeRolePolicyDocument'

# Azure — view federated identity credentials on the user-assigned managed identity
az identity federated-credential list --identity-name <mi> --resource-group <rg>

# GCP — view IAM policy on the service account
gcloud iam service-accounts get-iam-policy <gsa>@<project>.iam.gserviceaccount.com
```

## Cluster state to confirm before review

- **OIDC issuer enabled** on the cluster (provider-specific switch).
- **OIDC issuer URL** — IAM trust policies key off this URL.
- **Webhook installed** for the workload identity model (AWS Pod Identity Webhook, Azure Workload Identity admission webhook, GKE built-in).
- **Default audience** for the cluster (cloud-specific): `sts.amazonaws.com` on AWS, `api://AzureADTokenExchange` on Azure, `<workload-identity-pool-name>` on GCP.
- **Service account → IAM mapping mechanism**: annotation, label, federated identity credential, or IAM policy binding.

## Sanitization rules

- Never request kubeconfig contents, IAM access keys, Azure client secrets, GCP service account JSON keys.
- Replace identifiable cluster URLs, account IDs, tenant IDs, and project IDs with placeholders unless the user provides them.
- Do not print projected ServiceAccount token JWTs; reference the file path and audience claim only.
