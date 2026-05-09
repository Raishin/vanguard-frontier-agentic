# Official sources

Use this reference only when you need source grounding for GCP IAM service behavior or the detailed source list.

## GCP documentation

Use these as starting points, not as proof of the user's live GCP state:

- https://cloud.google.com/iam/docs/using-iam-securely — IAM best practices and security guidance
- https://cloud.google.com/iam/docs/resource-manager-policy-evaluation — how IAM bindings are evaluated across the resource hierarchy
- https://cloud.google.com/iam/docs/workload-identity-federation — configuring WIF for external workloads
- https://cloud.google.com/resource-manager/docs/organization-policy/overview — org policy constraints reference
- https://cloud.google.com/iam/docs/service-account-permissions — Service Account permissions and impersonation
- https://cloud.google.com/iam/docs/conditions-overview — IAM conditions syntax and supported resources
- https://cloud.google.com/iam/docs/creating-custom-roles — custom role design guidance
- https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity — GKE Workload Identity for pod-level SA binding

## Grounding rule

Official documentation explains GCP service behavior. It does not prove the user's current project, folder, org, quota, resource configuration, IAM boundary, or operational state. Prefer sanitized user-provided evidence (Terraform exports, gcloud output, Policy Analyzer output) for current-state claims. Never infer production state from documentation alone.
