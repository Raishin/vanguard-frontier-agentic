# Safety checklist

- Do not treat analyzer output as permission to skip Terraform review entirely.
- Do not ignore `replace`, `delete`, or unrelated non-inline changes just because some nested blocks look noisy.
- Do not use this skill for live apply approval without a separate Terraform guarded-live review.
- Do not normalize away provider-documented conflicts:
  - inline `aws_security_group` rules versus standalone rule resources,
  - inline `aws_route_table` routes versus standalone `aws_route` resources.
- Do not overclaim certainty on `aws_wafv2_web_acl`; the provider docs explicitly warn that inline rules can have ordering and recreation limitations.
- If plan JSON is incomplete, stale, hand-edited, or missing `resource_changes`, say confidence is lower.
- If sensitive or unknown-after-apply values limit comparison accuracy, say that explicitly.
