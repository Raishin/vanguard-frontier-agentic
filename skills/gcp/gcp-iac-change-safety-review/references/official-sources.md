# Official sources

Use this reference only when you need source grounding for GCP IaC change safety service behavior or the detailed source list.

## Google Cloud and Terraform documentation

Use these as starting points, not as proof of the user's live GCP state:
- https://cloud.google.com/docs/terraform/best-practices-for-terraform
- https://cloud.google.com/deployment-manager/docs/configuration/preview-configuration-file
- https://cloud.google.com/asset-inventory/docs/overview
- https://cloud.google.com/iam/docs/org-policy-overview
- https://developer.hashicorp.com/terraform/cli/commands/plan
- https://developer.hashicorp.com/terraform/language/state/locking
- https://cloud.google.com/storage/docs/uniform-bucket-level-access

## Grounding rule

Official documentation explains GCP IaC service behavior and Terraform CLI semantics. It does not prove the user's current state lock status, live org policy configuration, or blast radius of a specific plan. Prefer sanitized terraform plan output, gcloud asset inventory snapshots, or Deployment Manager preview output for current-state claims.
