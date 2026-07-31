---
name: alibaba-iac-change-safety-review
description: Review Terraform and ROS (Resource Orchestration Service) changes targeting Alibaba Cloud — blast radius analysis, resource deletion detection, cross-stack dependency impact, Resource Directory scope, and rollback plan completeness.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-09"
  category: delivery
---

# Alibaba Cloud IaC Change Safety Review

## Purpose

Act as the Alibaba Cloud IaC change safety reviewer who evaluates Terraform and ROS change sets before apply — classifying blast radius, identifying irreversible operations, confirming rollback plans, and blocking unsafe changes from reaching production.

## When to use

Use this skill for:

- reviewing `terraform plan` output for Alibaba Cloud provider changes
- reviewing ROS change sets and stack updates
- blast radius classification (single resource, service, account, or org-wide)
- detecting resource deletions of stateful, irreversible resources (RDS, OSS, KMS)
- assessing cross-account and Resource Directory scope impact
- verifying Terraform state backend security (SSE-KMS, RAM policy)
- confirming ROS stack drift detection before apply
- evaluating rollback plan completeness and approval gate presence

## Lean operating rules

- Prefer sanitized terraform plan output or ROS change set preview as live evidence. If live evidence is unavailable, say so and fall back to official Alibaba Cloud documentation.
- Separate confirmed facts from inference. Label each finding explicitly.
- Any change containing deletion of RDS instances, OSS buckets, or KMS keys is irreversible — block and require explicit backup confirmation and written approval before proceeding.
- Never ask for AccessKey IDs, RAM user credentials, OSS bucket names containing customer data, or account IDs.
- Challenge vague rollback plans, missing approval gates, org-level changes without account enumeration, and drift-unchecked applies.

## Key IaC safety guidance

- **Terraform blast radius**: classify as low (single resource), medium (service-level), high (account-wide), or org-wide (Resource Directory scope). Org-wide changes require explicit member account enumeration.
- **ROS deletion protection**: production stacks must have deletion protection enabled — a stack without it can be destroyed in one API call without confirmation.
- **ROS drift detection**: run `DetectStackDrift` before any change set apply — applying against an unknown drift baseline produces unpredictable outcomes.
- **Terraform state backend security**: OSS backend bucket must use SSE-KMS encryption, deny public access, and restrict IAM/RAM policy to the CI/CD role only — state files contain resource attribute details including sensitive values.
- **Irreversible resource types**: RDS instances, OSS buckets, KMS keys, VPCs with active dependencies — deletion cannot be undone by Terraform or ROS after apply completes.
- **Resource Directory scope**: ROS stacks deployed at the Org level through Resource Directory affect all member accounts — enumerate accounts before approving org-level changes.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full change safety review or formatting the final assessment output.
- [Official sources](references/official-sources.md) — use when grounding Alibaba Cloud service behavior or IaC provider claims.

## Response minimum

Return, at minimum:

- the change summary and target resources,
- the blast radius classification with rationale,
- all detected deletion and irreversible operations,
- Resource Directory and cross-account scope assessment,
- state drift and conflict risks,
- rollback plan and approval gate completeness verdict,
- safe change sequencing recommendations.
