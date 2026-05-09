# Official sources

Use this reference only when you need source grounding for GCP GCS access control, VPC Service Controls, and data residency service behavior or the detailed source list.

## Google Cloud documentation

Use these as starting points, not as proof of the user's live GCP state:
- https://cloud.google.com/storage/docs/access-control/uniform-bucket-level-access
- https://cloud.google.com/storage/docs/public-access-prevention
- https://cloud.google.com/vpc-service-controls/docs/supported-products
- https://cloud.google.com/storage/docs/lifecycle
- https://cloud.google.com/storage/docs/bucket-lock
- https://cloud.google.com/storage/docs/access-control/iam-conditions
- https://cloud.google.com/storage/docs/locations
- https://cloud.google.com/resource-manager/docs/organization-policy/restricting-domains

## Grounding rule

Official documentation explains GCP GCS access control and VPC-SC behavior. It does not prove the user's current bucket IAM bindings, org policy enforcement, or lifecycle policy configuration. Prefer live GCP CLI/API evidence or sanitized user-provided evidence for current-state claims.
