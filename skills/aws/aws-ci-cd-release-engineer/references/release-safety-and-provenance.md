# Release Safety and Provenance Guide

Use this reference for AWS release pipeline reviews covering CodePipeline, CodeBuild, CodeDeploy, GitHub Actions, GitLab, artifact provenance, approvals, deployment gates, progressive delivery, rollback, and change correlation.

## What people get wrong

The lazy story is:

> If the pipeline is green, the release is safe.

Wrong. Green means a configured path passed. It does not prove artifact identity, environment isolation, approval quality, rollback readiness, or runtime health after deploy.

Common bad assumptions:

- Build success equals deploy readiness.
- Manual approval is meaningful without evidence attached.
- Artifact bucket/KMS policy is plumbing, not supply-chain control.
- Re-running a failed pipeline is safe.
- CodeDeploy rollback exists because CodeDeploy is used.
- GitHub/GitLab workflow permissions are separate from AWS risk.

## Release-specific failure modes

- Source revision, artifact, image digest, and deployment target are not tied together.
- Pipeline role or OIDC trust allows unintended branch, repo, environment, or account deployment.
- Tests run against mocks while production deploy changes IAM/network/data paths.
- Approval gates lack diff, risk, change ticket, owner, rollback, and blast-radius evidence.
- CodeDeploy canary/blue-green alarms or hooks are absent or not customer-relevant.
- Parallel/queued execution mode creates out-of-order source revisions or stale definitions.

## Minimum safe workflow

1. Identify source, build, artifact, approval, deploy, verification, and rollback stages.
2. Trace provenance from commit to artifact/image digest to deployment target.
3. Review secrets, OIDC/IAM roles, artifact bucket/KMS, and cross-account trust.
4. Evaluate gates: tests, security scans, policy checks, manual approval evidence, and change windows.
5. Verify deployment strategy: canary, blue/green, linear, all-at-once, rollback alarms, and post-deploy checks.
6. Correlate releases with incidents using deployment timestamps and runtime telemetry.
7. Recommend changes as review guidance; live reruns/deploys require explicit approval.

## Verification targets

- CodePipeline stage/action definitions, execution mode, source revision, artifact store, KMS key, and service role
- CodeBuild buildspec, environment image, privileged mode, secrets, cache, reports, and artifact outputs
- CodeDeploy deployment group, AppSpec, hooks, deployment config, alarms, and rollback settings
- GitHub Actions/GitLab OIDC trust, branch/environment protections, required checks, and secret scoping
- artifact/image signing, digest pinning, SBOM/provenance, promotion rules, and immutable release IDs
- post-deploy metrics, synthetic checks, alarms, incident correlation, and rollback runbook

## When to push back

Push back if the user asks to:

- remove tests, gates, or approvals to speed release
- deploy from mutable tags without digest/provenance controls
- rerun production pipelines without understanding failed stage and source revision
- broaden deploy roles or OIDC trust as a shortcut
- call manual approval sufficient without attached risk evidence
- skip rollback alarms or post-deploy validation
