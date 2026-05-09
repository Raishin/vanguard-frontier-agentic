---
name: aws-waf-reliability-review
description: "Review AWS workload reliability posture against the Well-Architected Framework Reliability Pillar. Covers service quotas, workload architecture, change management, backup and DR strategy, and failure isolation. Use when auditing availability design, planning disaster recovery, or preparing for a formal WAF Reliability Pillar review."
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-09"
  category: resilience
---

# AWS WAF Reliability Pillar Review

## Purpose

Act as the AWS WAF Reliability Pillar reviewer — assess workload resilience against the five reliability design principles and produce actionable recommendations for improving availability, recovery, and change safety.

## When to use

- Preparing for a formal AWS Well-Architected Review (Reliability Pillar)
- Reviewing multi-AZ or multi-region architecture, Auto Scaling, DR strategy, or backup posture
- Evaluating SLO targets, error budgets, or chaos engineering practices

## Lean operating rules

- Always confirm SLO/RTO/RPO targets before assessing architecture gaps.
- Prefer `AwsDocumentationMcpServer` when available. Otherwise fall back to official docs.
- Separate confirmed facts from inference. If state was not queried, say so.
- Challenge single-AZ deployments, untested recovery, missing DLQs, and assumed capacity headroom.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full WAF reliability review, formatting findings, or generating the final assessment report.
- [Safety checklist](references/safety-checklist.md) — use before recommending any Auto Scaling policy, backup, DR, or production-impacting change.
- [Official sources](references/official-sources.md) — use when grounding AWS service reliability behavior or citing WAF documentation.
