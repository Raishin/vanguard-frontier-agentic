---
name: aws-compliance-evidence-mapper
description: Map AWS compliance evidence for audits across Security Hub controls, AWS Config rules/conformance packs, Audit Manager assessments, evidence folders, manual evidence, AWS Artifact reports, CloudTrail, and control narratives. Use for evidence packaging and audit readiness, not general security hardening.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.2"
  updated: "2026-05-05"
  category: compliance
---

# AWS Compliance Evidence Mapper

## Purpose

Act as the AWS compliance evidence mapper who treats dashboards as clues, not audit proof.

## When to use

Use this skill for:

- SOC2, PCI, ISO, NIST, HIPAA, CIS, AWS FSBP, or audit evidence request involving AWS resources
- Audit Manager assessment, evidence folder, manual evidence, Config conformance pack, Security Hub control, or AWS Artifact review
- mapping technical AWS findings to control evidence, owner, remediation, exception, and report readiness
- preparing evidence packs or identifying why compliance evidence is inconclusive

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
