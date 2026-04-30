# Safety checklist

- Do not treat analyzer output as permission to skip Terraform review entirely.
- Do not ignore `replace`, `delete`, or unrelated non-repeated-block changes just because some OCI nested blocks look noisy.
- Do not use this skill for live apply approval without a separate Terraform guarded-live review.
- Do not assume every OCI repeated block is modeled; support is intentionally conservative.
- Do not treat OCI routing-policy rule churn as harmless without checking whether ordered routing logic changed.
- Do not treat path-route-set or rule-set churn as harmless without checking whether path matching, backend selection, or header-manipulation behavior changed.
- If plan JSON is incomplete, stale, hand-edited, or missing `resource_changes`, say confidence is lower.
- If sensitive or unknown-after-apply values limit comparison accuracy, say that explicitly.
