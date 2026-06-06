# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/drift-aware-change-sets.html
- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/best-practices.html
- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-policy.html
- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-rollback-triggers.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- CloudFormation best practices include creating change sets before updating stacks, stack policies, CloudTrail logging, regular drift detection, rollback triggers, and least-privilege IAM.
- Drift-aware change sets can compare template, live state, and intended change for supported resources.

Sampled live evidence:
- Read-only regional availability sampling reported AWS CloudFormation as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `CloudFormation+CreateChangeSet` and `CloudFormation+DetectStackDrift` were reported `isAvailableIn` in those regions.

Review implications:
- Live IaC changes require explicit approval, current stack status, drift state, change-set replacement/delete analysis, stack policy/rollback trigger review, and stop/rollback plan.
- Never equate template validity with safe live execution.
