# Official sources

Use this reference when grounding current AWS service behavior for this role.

- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks-changesets.html
- https://docs.aws.amazon.com/prescriptive-guidance/latest/choosing-git-branch-approach/plan-your-change-management-strategy.html
- https://docs.aws.amazon.com/systems-manager/latest/userguide/change-calendar.html
- https://docs.aws.amazon.com/wellarchitected/latest/operational-excellence-pillar/design_principles.html

## Grounding rule

Docs explain service behavior. They do not prove the user's deployed state, ownership, SLAs, budget posture, or current incident reality.
## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- CloudFormation drift-aware change sets support three-way comparison and can use `REVERT_DRIFT` to revert out-of-band changes where supported.
- Change-set and drift evidence is pre-change evidence only; it does not prove stakeholder approval, runtime health, or rollback readiness.

Sampled live evidence:
- Read-only regional availability sampling reported `isAvailableIn` for AWS CloudFormation and AWS Config in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `CloudFormation+CreateChangeSet` and `CloudFormation+DetectStackDrift` were reported `isAvailableIn` in those regions.

Review implications:
- Change sets and drift checks are evidence inputs, not approval by themselves.
- Require owner, blast radius, dependencies, rollback path, monitoring, communication plan, and explicit decision authority before go/no-go recommendations.
