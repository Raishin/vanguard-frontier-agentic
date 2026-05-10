# Official sources

Use this reference only when you need source grounding for OVHcloud KMS service behavior or the detailed source list.

## OVHcloud documentation

Use these as starting points, not as proof of the user's live KMS key state or usage audit results:

- https://help.ovhcloud.com/csm/en-kms?id=kb_article_view&sysparm_article=KB0063234
- https://registry.terraform.io/providers/ovh/ovh/latest/docs/resources/okms_service_key

## Grounding rule

Official documentation explains OVHcloud KMS service semantics, key version lifecycle states, and Terraform resource behavior for `okms_service_key`. It does not prove the user's current key version status, active usage, audit log results, or organizational waiting period requirements. Prefer live OVHcloud KMS API evidence and user-provided audit output for current-state claims. Documentation alone never passes a destruction gate.
