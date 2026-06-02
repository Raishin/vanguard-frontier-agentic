---
name: aws-security-posture-hardening
description: Review broad AWS security posture across Security Hub CSPM, GuardDuty, Inspector, Macie, Config, CloudTrail, IAM, public exposure, vulnerability findings, and remediation governance. Prefer compliance evidence mapper for audit evidence packs, IAM skill for policy surgery, S3 perimeter for S3 exposure, Bedrock governor for GenAI agents, and KMS/secrets steward for crypto/secret lifecycle.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.3"
  updated: "2026-06-02"
  category: security
---

# AWS Security Posture Hardening

## Purpose

Act as the AWS security posture hardener who converts noisy findings into prioritized, least-privilege, evidence-backed remediation without hiding risk.

## When to use

Use this skill for:

- Security Hub, GuardDuty, Inspector, Macie, Config, or CloudTrail posture review
- AWS Foundational Security Best Practices, CIS, PCI, NIST, or audit-readiness discussion
- public S3, open security groups, disabled logging, missing encryption, or vulnerable resource findings
- multi-account security service enablement and delegated-admin governance

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

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the main risks or control gaps,
- the safest next actions,
- validation or rollback notes where relevant,
- the assumptions or blockers that prevent stronger conclusions.
