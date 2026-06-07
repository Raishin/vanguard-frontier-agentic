# Pipeline Failure Analysis Guide

Use this reference when fixing CodePipeline, CodeBuild, CodeDeploy, GitHub Actions, GitLab, buildspecs, artifact paths, environment variables, or deployment wiring in repository files.

## What people get wrong

The lazy story is:

> The pipeline failed, so patch the line that looks broken.

Wrong. Pipeline failures are often evidence-routing problems: wrong revision, wrong artifact, wrong role, wrong environment, or a deployment failure surfacing as a build failure.

Common bad assumptions:

- The failed stage is the root cause.
- Re-running is harmless.
- Build logs contain no secrets.
- Artifact path changes are low risk.
- A green build means deploy safety.
- Fixing CI config authorizes a live pipeline run.

## Failure-mode map

- **Source stage:** wrong branch, webhook, connection, commit, submodule, or artifact format.
- **Build stage:** buildspec path, runtime image, env var, IAM role, dependency cache, test command, artifact upload.
- **Deploy stage:** CodeDeploy lifecycle hook, ECS task set, Lambda alias, CloudFormation change set, missing permission.
- **Approval/gate:** missing manual approval, stale approval, wrong condition, Lambda approval action failure.
- **Cross-account:** artifact bucket/KMS/key policy, role trust, external ID, region mismatch.

## Minimum safe workflow

1. Identify provider and failing stage/action.
2. Confirm source revision and artifact that failed.
3. Inspect logs without exposing secrets.
4. Patch the smallest repo-side cause.
5. Preserve gates, approvals, artifact integrity, and rollback settings.
6. Run local lint/test/build validators relevant to the changed file.
7. State whether a live pipeline re-run is required and require approval for it.

## Verification targets

- pipeline definition or workflow YAML
- buildspec and artifact paths
- CodeBuild project env/runtime/image settings where represented in repo
- CodeDeploy AppSpec and deployment group references
- CloudFormation/CDK/Terraform pipeline resource definitions
- IAM role references and KMS/artifact bucket wiring
- failing log excerpt sanitized by the user or read-only tool output

## When to push back

Push back if the user asks to:

- remove tests/gates to make the pipeline green
- print or paste secret-bearing logs
- re-run production deploys without approval
- change artifact identity without release-owner signoff
- widen deploy role permissions as a blind fix
- ignore a failed post-deploy validation

