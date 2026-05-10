---
name: ovhcloud-iam-policy-review
description: Review OVHcloud IAM policies for overly permissive allow rules, missing deny blocks, unscoped URNs, absent condition blocks (IP CIDR, resource tag, expiration), and identity-group hygiene. Use when the user needs to audit access control, review `ovh_iam_policy` Terraform resources, assess OAuth2 service account scopes, or validate conditional access configuration against the principle of least privilege.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-10"
  category: security
---

# OVHcloud IAM Policy Review

## Purpose

Audit OVHcloud IAM policies for over-permissive access, missing conditional controls, and identity-group hygiene gaps. Produce an evidence-backed verdict with least-privilege recommendations.

## When to use

Use this skill for:

- Auditing `ovh_iam_policy` Terraform resources for scope and condition gaps
- Reviewing OAuth2 service account permissions against the principle of least privilege
- Assessing identity groups for membership sprawl or excessive aggregated permissions
- Evaluating conditional access blocks: IP CIDR restrictions, resource tag conditions, expiration dates
- Pre-deployment review of new IAM policies or policy changes

## Lean operating rules

- Prefer OVHcloud IAM docs and Terraform provider docs; if MCP tooling is unavailable, fall back to https://help.ovhcloud.com/ and Context7.
- Separate confirmed policy state from inference. If the policy was not shown, say so.
- Challenge policies with wildcarded URNs (`urn:v1:eu:resource:*`), missing condition blocks, or allow rules that supersede deny rules unexpectedly.
- Recommend least-privilege: scope to narrowest URN prefix, add IP condition, set expiration where supported.
- Keep recommendations reversible and explicit about blast radius.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full IAM audit or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use before privileged, access-granting, or production-impacting recommendations.
- [Official sources](references/official-sources.md) — use when grounding OVHcloud IAM service behavior or checking the source list.

## Response minimum

Return, at minimum:

- the policy verdict and evidence level,
- specific URN scope and condition gaps found,
- the blast radius of the current policy,
- safe remediation recommendations with rollback notes,
- blockers or unknowns that prevent stronger conclusions.
