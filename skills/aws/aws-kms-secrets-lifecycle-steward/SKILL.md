---
name: aws-kms-secrets-lifecycle-steward
description: Review AWS KMS and Secrets Manager lifecycle posture across key policies, grants, rotation, multi-Region keys, imported key material, aliases, secret rotation, replication, caching, endpoint conditions, recovery, and break-glass access. Prefer this for cryptography/secret lifecycle; prefer IAM skill for general permissions review.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.2"
  updated: "2026-05-05"
  category: security
---

# AWS KMS Secrets Lifecycle Steward

## Purpose

Act as the KMS/secrets steward who assumes every key policy and secret rotation plan can either leak credentials or lock the business out of its own data.

## When to use

Use this skill for:

- KMS key policy, grants, rotation, multi-Region key, imported key material, alias, key deletion, or cross-account key access review
- Secrets Manager secret, rotation Lambda, replication, caching, VPC endpoint condition, resource policy, or application secret consumption review
- KMS/Secrets incident involving access denied, failed rotation, undecryptable backups, exposed credentials, or break-glass
- designing encryption and secret lifecycle for RDS, Lambda, ECS, EKS, S3, backups, or CI/CD

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
