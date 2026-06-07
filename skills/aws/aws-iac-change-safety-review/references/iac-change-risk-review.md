# IaC Change Risk Review Guide

Use this reference when reviewing CloudFormation, CDK, SAM, Terraform, Serverless Framework, change sets, drift, stack policies, StackSets, and production deployment preflights.

## What people get wrong

The lazy story is:

> The plan/change set is the source of truth, so read it and decide.

Wrong. Plans and change sets are necessary but not sufficient. They can miss live drift, parameter context, provider behavior, replacement side effects, data retention, and organizational guardrails.

Common bad assumptions:

- `No changes` means no operational risk.
- Replacement is acceptable if CloudFormation can perform it.
- Terraform destroy/create is safe when names stay similar.
- Drift is separate from the proposed change.
- Stack policies and rollback triggers are optional.
- IAM diffs can be reviewed by action count alone.

## IaC-review failure modes

- Resource replacement deletes state, changes DNS, rotates identity, or breaks consumers.
- IAM, KMS, security group, route, bucket, or trust-policy changes widen blast radius.
- Parameter/default changes alter multiple environments unexpectedly.
- StackSets or modules propagate a risky pattern across accounts/Regions.
- Drift means the apply will not behave like the repo diff.
- Rollback would fail because stateful resources, migrations, or retained resources are not handled.

## Minimum safe workflow

1. Identify IaC tool, target environment, account/Region scope, and deployment mechanism.
2. Review repo diff plus generated template/plan/change set, not only source files.
3. Classify each material change as add, modify, replace, delete, permission widen, exposure, stateful, or rollback-sensitive.
4. Check drift, parameters, stack policies, rollback triggers, and cross-stack/module dependencies where applicable.
5. Require validation evidence: lint, synth, plan/change set, policy checks, and test outputs.
6. Return go/no-go with blockers, risky resources, mitigation, owner, and rollback evidence.
7. Keep deployment/apply/destroy outside this review unless a separate guarded approval path is invoked.

## Verification targets

- CloudFormation/SAM template diff, change set replacement/delete details, drift detection, stack policy, rollback trigger, and events
- CDK synth/diff output, bootstrap environment, asset changes, and security approval prompts
- Terraform fmt/validate/plan, provider lockfile, state backend, targeted apply risk, and destroy/replacement markers
- IAM/KMS/security group/network/storage/database diffs and policy validation output
- StackSets, modules, nested stacks, parameters, exports/imports, and environment overlays
- rollback plan: previous template, state backup, retained resources, migration reversal, and data restore point

## When to push back

Push back if the user asks to:

- approve apply from source diff only
- ignore replacement/delete risk because validation passed
- skip drift detection for production stacks
- widen IAM/network controls as a quick fix
- use targeted apply/destroy without dependency analysis
- combine unrelated cleanup with a risky production change
