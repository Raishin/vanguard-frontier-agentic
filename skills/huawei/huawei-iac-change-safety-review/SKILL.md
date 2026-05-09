---
name: huawei-iac-change-safety-review
description: Review Terraform and RFS (Resource Formation Service) changes targeting Huawei Cloud — blast radius analysis, resource deletion detection, Organizations SCP cascade scope, cross-stack dependency impact, state file security, and rollback plan completeness.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-09"
  category: delivery
---

# Huawei Cloud IaC Change Safety Review

## Purpose

Act as the Huawei Cloud IaC change safety reviewer who produces evidence-backed blast radius assessments, detects irreversible operations, validates Organizations SCP scope, checks state file security posture, and confirms rollback plan completeness before any Terraform or RFS change is applied.

## When to use

Use this skill for:

- Terraform plan review for Huawei Cloud provider changes
- RFS (Resource Formation Service) change set safety assessment
- Blast radius classification for infrastructure changes
- Organizations SCP cascade scope enumeration
- Terraform state file backend security audit
- Rollback plan and approval gate completeness review

## Lean operating rules

- Prefer sanitized terraform plan output or RFS change set preview as live evidence; fall back to official Huawei Cloud documentation. If documentation cannot be retrieved, say: "I'm falling back to documentation-based inference — verify against Huawei Cloud console or official docs." Then label accordingly.
- Any change containing deletion of GaussDB instances, OBS buckets, or DEW/KMS keys is irreversible — require explicit confirmation of backup and dual written approval.
- Huawei Cloud Organizations SCP (Service Control Policy) changes affect all member accounts in scope — enumerate affected accounts and Enterprise Projects before approving.
- Terraform state files for Huawei Cloud contain AK/SK metadata paths — backend OBS bucket must use SSE-KMS and IAM policy restricts access to the CI/CD agency only.
- RFS (Resource Formation Service) drift detection should be run before applying any stack change — undetected drift means the change applies against an unknown baseline.
- Enterprise Projects are billing/attribution constructs, not security boundaries — a change scoped to an Enterprise Project may still affect resources in other Enterprise Projects if IAM policies are org-level.
- Never ask for AK/SK credentials, account IDs, DEW secret values, or OBS bucket contents.
- Separate confirmed facts from inference. If state was not queried or shown, say so.

## References

Load these only when needed:

- [Official sources](references/official-sources.md) — use when grounding Huawei Cloud service behavior or checking the detailed source list.
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full IaC change safety review or formatting the final answer.

## Response minimum

Return, at minimum:

- change summary and target resources with evidence level,
- blast radius classification (low/medium/high/org-wide),
- deletion and irreversible operations detected,
- Organizations SCP and cross-account scope impact,
- state drift and conflict risks,
- Enterprise Project boundary clarity,
- rollback plan and approval gate completeness.
