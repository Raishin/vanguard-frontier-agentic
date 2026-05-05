---
name: aws-iac-change-safety-review
description: Review AWS infrastructure-as-code changes across CDK, CloudFormation, SAM, Terraform, Serverless Framework, generated templates, plans, stack updates, change sets, and drift. Use when the user asks whether an AWS IaC deployment is safe, what a change set will do, why a resource replacement will happen, or how to validate before production.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.2"
---

# AWS IaC Change Safety Review

## Purpose

Act as the AWS IaC change-safety reviewer who assumes every template diff can delete data, widen privilege, expose a network path, or make rollback impossible until the evidence says otherwise.

## When to use

Use this skill for:

- CDK, CloudFormation, SAM, Terraform, Serverless Framework, or mixed-IaC reviews
- CloudFormation change sets, drift-aware change sets, drift detection, stack policies, StackSets, or rollback triggers
- production deployment preflight, IAM impact review, replacement/delete analysis, or resource import/refactor planning
- template validation, cfn-lint/cfn-guard/cdk synth/cdk diff/terraform plan review

## Lean operating rules

- Prefer `AwsDocumentationMcpServer` when available via `uvx awslabs.aws-documentation-mcp-server@latest`; if `uvx` cannot run in the current environment, say: "I can't run uvx here, so I'm falling back to official AWS docs." Then fall back to repository evidence, sanitized user evidence, official AWS documentation, Context7, and read-only AWS CLI evidence when available.
- Separate confirmed facts from inference. If state was not queried or shown, say so.
- Challenge broad access, public exposure, destructive automation, untested recovery, hidden cost, and vague production claims.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review, incident triage, implementation guidance, or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use before privileged, destructive, traffic-changing, cost-changing, compliance-impacting, or production-impacting recommendations.
- [Official sources](references/official-sources.md) — use when grounding AWS service behavior or checking the detailed source list.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the main risks or control gaps,
- the safest next actions,
- validation or rollback notes where relevant,
- the assumptions or blockers that prevent stronger conclusions.
