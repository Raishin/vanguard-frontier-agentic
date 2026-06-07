# IaC Patch Safety Guide

Use this reference when editing CloudFormation, SAM, CDK, Terraform, or AWS-focused IaC files to correct defects while staying repo-scoped.

## What people get wrong

The lazy story is:

> IaC patching is safe because it is only a file change.

Wrong. A small IaC diff can delete data, replace resources, widen IAM, expose networks, or break rollback once applied.

Common bad assumptions:

- Syntax validation catches dangerous changes.
- Resource replacement is acceptable unless production says otherwise.
- Deleting a property is safer than setting it explicitly.
- IAM wildcard is temporary and harmless.
- Security group changes are easy to reason about by inspection.
- Terraform plan/change set is optional for small patches.

## High-risk diff classes

Flag these before editing or recommending apply:

- resource deletion or replacement
- IAM action/resource/principal broadening
- security group/NACL/route/public exposure changes
- KMS key policy, secret, certificate, or data-store retention changes
- database, queue, bucket, stream, or backup lifecycle changes
- cross-account trust or SCP/permission boundary changes
- drift-sensitive changes where live state may differ from repo

## Minimum safe workflow

1. Identify IaC framework and target environment.
2. Inspect existing style and patch only the requested defect.
3. Classify blast radius and high-risk diff class.
4. Run static validation: template lint, synth, format, schema, or validate.
5. If live execution is requested later, require plan/change-set/drift evidence first.
6. Provide rollback: revert diff, prior parameter value, previous template, or state-safe rollback path.
7. Keep live apply/deploy/destroy out of scope unless separately approved.

## Verification targets

- `cfn-lint`, `aws cloudformation validate-template`, `sam validate`
- `cdk synth`, `cdk diff`
- `terraform fmt`, `terraform validate`, `terraform plan` when applicable
- CloudFormation replacement/delete indicators from change sets
- drift detection output for existing stacks
- policy validation for IAM/security-sensitive changes
- project-specific tests and schema checks

## When to push back

Push back if the user asks to:

- apply/deploy immediately after a repo patch
- skip plan/change-set because the diff is small
- use wildcard IAM to unblock validation
- delete stateful resources without backup/retention proof
- change production networking without rollback path
- “clean up” unrelated IaC while fixing one issue

