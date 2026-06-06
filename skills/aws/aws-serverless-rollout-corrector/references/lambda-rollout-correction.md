# Lambda Rollout Correction Guide

Use this reference when fixing Lambda aliases, versions, CodeDeploy/SAM traffic shifting, event-source mappings, destinations, DLQs, or serverless rollout wiring in repository files.

## What people get wrong

The lazy story is:

> It is just a Lambda config fix; update the template and redeploy.

Wrong. Serverless rollout defects often hide in alias/version traffic, event-source semantics, retries, and alarms. A repo patch can make the next deployment worse if it ignores those boundaries.

Common bad assumptions:

- `$LATEST` is acceptable in production rollout wiring.
- Alias changes are harmless because code is immutable.
- Event-source mapping changes are not production-impacting.
- CodeDeploy hooks are optional ceremony.
- DLQ/destination changes do not affect audit or replay.
- A SAM/AppSpec diff proves runtime rollback readiness.

## Service-specific failure modes

- Alias points to the wrong published version or shifts traffic to unvalidated code.
- Provisioned concurrency is attached to the wrong alias/version.
- CodeDeploy deployment preference lacks alarms or lifecycle validation hooks.
- Event source mapping batch size, maximum batching window, filter criteria, or starting position changes replay/latency behavior.
- SQS/Lambda partial batch response settings are missing or inconsistent.
- Async destinations or DLQs are removed during a hotfix.
- IAM permission changes break event-source polling, log writes, or downstream calls.

## Minimum safe workflow

1. Identify the rollout mechanism: SAM, CloudFormation, CDK, Terraform, Serverless Framework, or raw config.
2. Identify the release primitive: Lambda version, alias, CodeDeploy deployment group, event source mapping, API stage, or EventBridge rule.
3. Patch the smallest repo field that corrects the defect.
4. Preserve existing rollback path: previous alias target, previous mapping config, previous deployment preference, or previous template version.
5. Run local validators for the IaC/framework in use.
6. State what the patch changes at runtime and what it does not execute.
7. Require explicit approval before deploy, rollback, publish-version, alias update, or event-source mutation.

## Verification targets

Use repo and read-only evidence where available:

- Lambda alias/version references in templates
- CodeDeploy deployment preference, alarms, and lifecycle hooks
- event-source mapping config: source ARN, batch size, filters, enabled state, destinations
- function timeout, memory, reserved/provisioned concurrency
- DLQ and async destination configuration
- IAM permissions for invoke, poll, log, and downstream calls
- validation commands such as `sam validate`, `cfn-lint`, `cdk synth`, `terraform validate`, or project tests

## When to push back

Push back if the user asks to:

- patch rollout config and immediately deploy without approval
- remove alarms/hooks to make a deployment pass
- point production aliases at `$LATEST`
- disable event-source mappings without a replay plan
- “fix” retries by dropping failures silently
- claim rollback is safe without a previous alias/version target

