# Deployment Hotfix Safety Guide

Use this reference for repo-side deployment hotfixes involving manifests, release parameters, environment wiring, deployment groups, pipeline inputs, rollback blockers, or emergency config correction.

## What people get wrong

The lazy story is:

> Hotfix means speed matters more than process.

Wrong. Hotfixes need less surface area, not less discipline. The fastest unsafe diff is how incidents become outages.

Common bad assumptions:

- Emergency context justifies broad config changes.
- A successful validator means production is safe.
- Rollback is obvious because Git has history.
- Deployment parameters are not security-sensitive.
- Temporarily disabling checks is acceptable if we re-enable later.
- The deployment tool will catch blast-radius issues.

## Hotfix risk classes

Classify the patch before editing:

- **Parameter fix:** wrong environment, ARN, image tag, feature flag, or region.
- **Manifest/schema fix:** invalid YAML/JSON/IaC shape blocking deploy.
- **Rollback unblocker:** restore previous deployment path or remove a bad forward-only change.
- **Guardrail fix:** re-enable alarm, approval, rollback, or validation control.
- **Risky workaround:** disables checks, widens permissions, changes traffic, or hides evidence.

Only the first four are normal for this skill. The fifth needs explicit risk acceptance.

## Minimum safe workflow

1. State the failed deployment symptom and evidence source.
2. Identify the exact file/field causing the blocker.
3. Make the smallest diff that corrects only that blocker.
4. Preserve or improve rollback controls; never remove them silently.
5. Run syntax/schema/project validators.
6. Explain runtime effect and non-effect: what will change only after a separate deployment.
7. Provide rollback diff or revert command.

## Verification targets

- deployment manifest diff
- release parameter source and target environment
- deployment group rollback settings
- pipeline approval/gate settings
- image/artifact/revision identity
- IaC template validation or buildspec/workflow lint
- CloudFormation/CodeDeploy/CodePipeline docs when service behavior matters

## When to push back

Push back if the user asks to:

- bypass approvals to “save time”
- disable rollback alarms or deployment gates
- widen IAM to unblock a deploy without root cause
- change multiple environments in one hotfix
- patch secrets or credentials into repo files
- treat repo edit as proof that production is fixed

