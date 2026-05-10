# Official sources

Use this reference when grounding Scaleway IAM service behavior or confirming permission set semantics.

## Scaleway IAM documentation

Use these as starting points — not as proof of the user's live Scaleway IAM state:

- https://www.scaleway.com/en/docs/iam/concepts/ — IAM concepts: organizations, projects, applications, permission sets, API keys
- https://www.scaleway.com/en/developers/api/iam/ — IAM API reference: policy, application, group, and API key resource operations
- https://registry.terraform.io/providers/scaleway/scaleway/latest/docs/resources/iam_policy — Terraform `scaleway_iam_policy` resource: rule structure, principal bindings, permission set assignment

## Grounding rule

Official documentation explains Scaleway IAM service behavior and permission set semantics. It does not prove the user's current policy bindings, active API key expiry dates, key rotation history, or live organization/project configuration. Prefer repo evidence or sanitized user-provided evidence for current-state claims.
