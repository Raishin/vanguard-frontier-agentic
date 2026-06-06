# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/config/latest/developerguide/codedeploy-deployment-group-auto-rollback-enabled.html
- https://docs.aws.amazon.com/codedeploy/latest/userguide/deployments-rollback-and-redeploy.html
- https://docs.aws.amazon.com/codepipeline/latest/userguide/welcome.html
- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/troubleshooting.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- AWS Config has a managed rule for CodeDeploy deployment groups with auto rollback enabled, marking deployment groups non-compliant when rollback is disabled.
- Deployment hotfix safety depends on rollback plan, change-management context, and actual deployment service configuration; repository edits alone do not prove production rollback readiness.

Sampled live evidence:
- Read-only regional availability sampling reported AWS CodeDeploy, AWS CodePipeline, and AWS CloudFormation as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `CodeDeploy+GetDeployment`, `CodePipeline+GetPipelineState`, and `CloudFormation+DescribeStacks` were reported `isAvailableIn` in those regions.

Review implications:
- Keep hotfixes repo-scoped unless the user explicitly asks for live action and approval gates are satisfied.
- Require smallest-diff patching, validation command output, rollback instructions, and clear separation between file correction and live deployment execution.
