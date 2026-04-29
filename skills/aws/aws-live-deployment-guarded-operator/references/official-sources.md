# Official sources

Primary AWS documentation used to shape this guarded live-AWS role:

- https://docs.aws.amazon.com/cli/v1/reference/sts/get-caller-identity.html
- https://docs.aws.amazon.com/codepipeline/latest/userguide/approvals.html
- https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-change-calendar.html
- https://docs.aws.amazon.com/wellarchitected/latest/operational-excellence-pillar/welcome.html

## Source-grounding rule

Use official AWS documentation as the source of truth for service behavior. Use Context7 only as a supplementary guideline for agent-guardrail design when it helps clarify stricter instruction patterns; do not let secondary examples override AWS service docs.
