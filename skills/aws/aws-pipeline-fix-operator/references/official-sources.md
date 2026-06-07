# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/codepipeline/latest/userguide/troubleshooting.html
- https://docs.aws.amazon.com/codebuild/latest/userguide/troubleshooting.html
- https://docs.aws.amazon.com/codedeploy/latest/userguide/troubleshooting-deployments.html
- https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-codepipeline-pipeline.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- CodeDeploy troubleshooting separates deployment failures by lifecycle event, credentials, PKCS7 validation, file conflicts, DownloadBundle errors, health checks, and platform/script behavior.
- CodePipeline can be represented as CloudFormation `AWS::CodePipeline::Pipeline`; repo-side fixes must keep pipeline resource semantics and IAM roles intact.

Sampled live evidence:
- Read-only regional availability sampling reported AWS CodePipeline, AWS CodeBuild, and AWS CodeDeploy as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `CodePipeline+GetPipelineState`, `CodeBuild+BatchGetBuilds`, and `CodeDeploy+GetDeployment` were reported `isAvailableIn` in those regions.

Review implications:
- Patch only the failing repo configuration unless explicitly authorized for live action. Require failing stage/action evidence, logs, minimal diff, validation command, and rollback.
- Do not guess root cause from a failed pipeline status; correlate source revision, build logs, deployment events, roles, artifacts, and environment config.
