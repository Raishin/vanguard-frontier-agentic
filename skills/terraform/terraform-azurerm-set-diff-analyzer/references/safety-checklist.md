# Safety checklist

- Do not treat analyzer output as permission to skip Terraform review entirely.
- Do not ignore `replace`, `delete`, or non-set attribute changes just because some nested blocks are order-only noise.
- Do not use this skill for live apply approval without a separate Terraform guarded-live review.
- Do not assume every AzureRM nested block is modeled; unsupported attributes can still contain real changes.
- Do not treat Application Gateway rewrite-rule-set or URL-path-map churn as harmless without checking listener, routing-rule, and path-routing semantics.
- Do not treat NSG security-rule churn as harmless without checking priority, direction, access, protocol, and address or port scope.
- If plan JSON is incomplete, stale, hand-edited, or missing `resource_changes`, say confidence is lower.
- If sensitive or unknown-after-apply values limit comparison accuracy, say that explicitly.
