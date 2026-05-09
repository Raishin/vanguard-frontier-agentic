# Official sources

Use this reference only when you need source grounding for GCP change impact and resource dependency service behavior or the detailed source list.

## Google Cloud documentation

Use these as starting points, not as proof of the user's live GCP state:
- https://cloud.google.com/asset-inventory/docs/overview
- https://cloud.google.com/vpc/docs/shared-vpc
- https://cloud.google.com/iam/docs/understanding-service-accounts
- https://cloud.google.com/resource-manager/docs/organization-policy/overview
- https://cloud.google.com/vpc/docs/vpc-peering
- https://cloud.google.com/asset-inventory/docs/searching-iam-policies
- https://cloud.google.com/resource-manager/docs/creating-managing-folders

## Grounding rule

Official documentation explains GCP resource dependency and org policy behavior. It does not prove the user's current Shared VPC topology, SA impersonation chains, or org hierarchy. Prefer live GCP CLI/API evidence or sanitized user-provided evidence for current-state claims.
