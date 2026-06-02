# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/codepipeline/latest/userguide/approvals.html
- https://docs.aws.amazon.com/codepipeline/latest/userguide/actions-invoke-lambda-function.html
- https://docs.aws.amazon.com/codepipeline/latest/userguide/tutorials-four-stage-pipeline.html
- https://docs.aws.amazon.com/codedeploy/latest/userguide/deployments-rollback-and-redeploy.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- CodePipeline supports manual approval actions and pipeline stages; approvals are a release gate, not evidence that the release is safe.
- Lambda invoke actions in CodePipeline use execution roles, continuation tokens, and JSON parameters; custom approval automation can fail independently of the deployment target.

Sampled live evidence:
- Read-only regional availability sampling reported AWS CodePipeline and AWS CodeDeploy as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `CodePipeline+GetPipelineState`, `CodePipeline+ListActionExecutions`, and `CodeDeploy+GetDeployment` were reported `isAvailableIn` in those regions.

Review implications:
- Do not approve a pipeline without current pipeline execution state, artifact/revision identity, test/security gate results, deployment target, rollback path, owner approval, and incident/change-calendar context.
- Approval is mutation-enabling; require explicit user authorization before approving or rejecting live actions.
