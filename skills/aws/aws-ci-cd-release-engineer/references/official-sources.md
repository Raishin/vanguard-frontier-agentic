# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/devopsagent/latest/userguide/about-aws-devops-agent.html
- https://docs.aws.amazon.com/devopsagent/latest/userguide/working-with-devops-agent-proactive-incident-prevention.html
- https://docs.aws.amazon.com/codedeploy/latest/userguide/deployments-rollback-and-redeploy.html
- https://docs.aws.amazon.com/codepipeline/latest/userguide/welcome.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, or operational state. Prefer AWS managed MCP read-only evidence through the user's configured read-only AWS profile, read-only AWS CLI evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- AWS Well-Architected operational guidance recommends build/deployment management systems and automated change management to reduce manual deployment error.
- CodeDeploy rollback documentation is relevant to deployment safety, but rollback behavior still depends on each deployment group/application configuration.

Sampled live evidence:
- Read-only regional availability sampling reported `isAvailableIn` for AWS CodePipeline, AWS CodeBuild, and AWS CodeDeploy in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `CodePipeline+GetPipelineState` and `CodeDeploy+GetDeployment` were reported `isAvailableIn` in those regions.

Review implications:
- Pipeline existence is not release safety. Require gates, test evidence, artifact integrity/provenance, least-privilege deployment roles, telemetry, rollback criteria, and post-deploy validation.
- Docs do not prove the user's pipeline configuration, branch protections, approvals, or production deployment state.
