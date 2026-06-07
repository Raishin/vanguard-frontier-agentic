---
name: aws-iac-change-safety-review
description: Review AWS infrastructure-as-code changes across CDK, CloudFormation, SAM, Terraform, Serverless Framework, generated templates, plans, stack updates, change sets, and drift. Use when the user asks whether an AWS IaC deployment is safe, what a change set will do, why a resource replacement will happen, or how to validate before production.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.4"
  updated: "2026-06-02"
  category: delivery
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

- Prefer current AWS documentation tools for service behavior. Use the per-skill facts and sampled live evidence in `references/official-sources.md`; when the user has configured read-only AWS MCP access, use exposed read-only tools for current-state evidence instead of guessing.
- Separate confirmed facts from inference. If state was not queried or shown, say so.
- Challenge broad access, public exposure, destructive automation, untested recovery, hidden cost, and vague production claims.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review, incident triage, implementation guidance, or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use before privileged, destructive, traffic-changing, cost-changing, compliance-impacting, or production-impacting recommendations.
- [Official sources](references/official-sources.md) — use when grounding AWS service behavior or checking the detailed source list.
- [IaC Change Risk Review Guide](references/iac-change-risk-review.md) — use for domain-specific failure modes, safe workflow, verification targets, and pushback criteria.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the main risks or control gaps,
- the safest next actions,
- validation or rollback notes where relevant,
- the assumptions or blockers that prevent stronger conclusions.
