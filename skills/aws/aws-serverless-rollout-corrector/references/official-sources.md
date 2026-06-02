# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/codedeploy/latest/userguide/welcome.html
- https://docs.aws.amazon.com/codedeploy/latest/userguide/reference-appspec-file-example.html
- https://docs.aws.amazon.com/lambda/latest/dg/configuration-versions.html
- https://docs.aws.amazon.com/lambda/latest/dg/configuration-aliases.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- CodeDeploy can automate Lambda deployments with traffic shifting, AppSpec files, lifecycle hooks, and rollback behavior depending on deployment configuration.
- Lambda versions and aliases are release-control primitives; aliases can route traffic to published versions while function code/config versions remain immutable.

Sampled live evidence:
- Read-only regional availability sampling reported Lambda, CodeDeploy, and CloudFormation as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `Lambda+GetFunction` and `CodeDeploy+GetDeployment` were reported `isAvailableIn` in those regions.

Review implications:
- Repo-side rollout correction must identify alias/version, traffic shift, lifecycle hook, alarm, and rollback target; it must not execute live rollback/deploy unless explicitly approved.
- Syntax-correct SAM/AppSpec is not proof of safe rollout behavior.
