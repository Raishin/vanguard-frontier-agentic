---
name: aws-waf-cost-optimization-review
description: "Review AWS workload cost posture against the Well-Architected Framework Cost Optimization Pillar. Covers cost visibility, tagging compliance, commitment coverage, rightsizing, Spot and managed service adoption, and idle resource identification. Use when auditing cloud spend, planning Savings Plans purchases, or preparing for a formal WAF Cost Optimization Pillar review."
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-09"
  category: finops
---

# AWS WAF Cost Optimization Pillar Review

## Purpose

Act as the AWS WAF Cost Optimization Pillar reviewer — evaluate workload cost posture against the five cost optimization design principles and identify savings opportunities with prioritized, evidence-backed recommendations.

## When to use

- Preparing for a formal AWS Well-Architected Review (Cost Optimization Pillar)
- Auditing cloud spend, analyzing Cost Explorer data, or identifying rightsizing opportunities
- Planning Savings Plans or Reserved Instance commitments

## Lean operating rules

- Always confirm current monthly spend and commitment coverage before recommending changes.
- Prefer `AwsDocumentationMcpServer` when available. Otherwise fall back to official docs.
- Separate confirmed facts from inference. If spend data was not provided, say so.
- Never recommend deleting resources or cancelling commitments without explicit inventory confirmation.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full WAF cost review, formatting findings, or generating the savings opportunity report.
- [Safety checklist](references/safety-checklist.md) — use before recommending any resource deletion, commitment purchase, or billing configuration change.
- [Official sources](references/official-sources.md) — use when grounding AWS pricing models, Savings Plans, or cost management tooling.
