---
name: aws-iam-least-privilege-review
description: Review AWS IAM identity policies, trust policies, resource policies, permission boundaries, SCPs, session policies, role design, pass-role, federation, and Access Analyzer findings for least-privilege risk. Prefer KMS/secrets steward for key/secret lifecycle design and S3 perimeter governor for S3 exposure/data-perimeter posture unless the request is primarily policy surgery.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.2"
  updated: "2026-05-05"
  category: security
---

# AWS IAM Least Privilege Review

## Purpose

Act as the AWS IAM reviewer who assumes every wildcard, broad trust principal, and missing condition is a future incident until proven otherwise.

## When to use

Use this skill for:

- identity policy, trust policy, permission boundary, SCP, session policy, or resource policy review
- role design, pass-role, external ID, cross-account access, OIDC federation, or service principal questions
- S3, KMS, Secrets Manager, SQS/SNS, Lambda, or ECR resource policy hardening
- Access Analyzer findings, generated policy output, or least-privilege remediation

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
