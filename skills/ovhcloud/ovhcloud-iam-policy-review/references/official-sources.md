# Official sources

Use this reference only when you need source grounding for OVHcloud IAM service behavior or the detailed source list.

## OVHcloud documentation

Use these as starting points, not as proof of the user's live OVHcloud IAM state:

- https://help.ovhcloud.com/csm/en-account-iam-policies?id=kb_article_view&sysparm_article=KB0055594
- https://registry.terraform.io/providers/ovh/ovh/latest/docs/resources/iam_policy
- https://api.ovh.com/console/#/me/api/credential

## Grounding rule

Official documentation explains OVHcloud IAM service behavior and URN schema. It does not prove the user's current account policies, identity group membership, OAuth2 credential scopes, or live access state. Prefer live OVHcloud API evidence or sanitized user-provided policy JSON for current-state claims.
