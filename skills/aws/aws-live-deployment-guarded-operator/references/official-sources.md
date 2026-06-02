# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/codedeploy/latest/userguide/welcome.html
- https://docs.aws.amazon.com/config/latest/developerguide/codedeploy-deployment-group-auto-rollback-enabled.html
- https://docs.aws.amazon.com/codepipeline/latest/userguide/approvals.html
- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/drift-aware-change-sets.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- CodeDeploy automates deployments for EC2, Lambda, and ECS and supports deployment types such as rolling and blue/green traffic shifting depending on compute platform.
- AWS Config has a managed rule checking whether CodeDeploy deployment groups have auto rollback enabled.

Sampled live evidence:
- Read-only regional availability sampling reported AWS CodeDeploy and AWS CloudFormation as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `CodeDeploy+GetDeployment`, `CodePipeline+GetPipelineState`, and `CloudFormation+CreateChangeSet` were reported `isAvailableIn` in those regions.

Review implications:
- Live deployment operation must be approval-gated and evidence-driven: current deployment state, rollback trigger, health alarms, blast radius, change set/diff, and stop/rollback command path.
- Do not perform or recommend live mutation from stale docs or repo-only evidence.
