# Official sources

Primary AWS documentation used to shape this guarded live-AWS role:

- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/best-practices.html
- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks-changesets.html
- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-rollback-triggers.html
- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/protect-stack-resources.html
- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/detect-drift-stack.html

## Source-grounding rule

Use official AWS documentation as the source of truth for service behavior. Use Context7 only as a supplementary guideline for agent-guardrail design when it helps clarify stricter instruction patterns; do not let secondary examples override AWS service docs.
