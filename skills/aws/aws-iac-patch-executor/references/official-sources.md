# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/best-practices.html
- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks-changesets-samples.html
- https://docs.aws.amazon.com/cdk/v2/guide/home.html
- https://docs.aws.amazon.com/prescriptive-guidance/latest/choose-iac-tool/cloudformation.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- CloudFormation best practices include validating templates, creating change sets before updating stacks, using stack policies, logging CloudFormation calls with CloudTrail, using drift detection, configuring rollback triggers, and applying least privilege.
- Change-set examples show that template edits, parameter changes, added/removed resources, and replacement fields can alter blast radius in different ways.

Sampled live evidence:
- Read-only regional availability sampling reported AWS CloudFormation as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `CloudFormation+ValidateTemplate`, `CloudFormation+CreateChangeSet`, and `CloudFormation+DetectStackDrift` were reported `isAvailableIn` in those regions.

Review implications:
- Patch execution must stay repo-scoped unless explicitly approved for live mutation; every patch needs minimal diff, validation output, expected plan/change-set effect, and rollback instructions.
- Do not claim production safety from local syntax validation alone; replacement/delete risk and live drift require stronger evidence.
