---
name: scaleway-iam-policy-review
description: Review Scaleway IAM policies, API key governance, service account bindings, and organization/project-level permission sets for least-privilege posture. Use when the user asks to audit API key scopes, review IAM policy breadth, assess service account access, or tighten Scaleway identity controls. Surfaces overly broad access, missing expiry, and key sprawl risks with actionable remediation paths.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-10"
  category: security
---

# Scaleway IAM Policy Review

## Purpose

Act as the Scaleway IAM access control auditor: review IAM policies, API key governance, service account bindings, and permission sets for least-privilege compliance.

## When to use

Use this skill for:

- Auditing Scaleway API key scopes, expiry, and rotation status
- Reviewing IAM policy breadth at organization vs project level
- Assessing service account permission sets and resource bindings
- Identifying key sprawl, wildcard permissions, or missing expiry controls
- Recommending IAM tightening paths and key lifecycle policies

## Key Scaleway IAM concepts

- **API keys**: `SCW_ACCESS_KEY` + `SCW_SECRET_KEY` pairs; can be scoped to organization or project level
- **IAM policies**: rule sets binding principals (users, applications, groups) to permission sets
- **Permission sets**: named bundles of allowed actions (e.g., `InstancesFullAccess`, `ObjectStorageReadOnly`)
- **Applications**: non-human identities for automation; should use project-scoped keys with expiry
- **Organizations vs Projects**: organization-level scope grants access to ALL projects — always prefer project scope

## Lean operating rules

- Prefer Scaleway IAM API docs when available; if MCP tooling is unavailable, say: "I can't access live Scaleway MCP here, so I'm falling back to official docs." Then use https://www.scaleway.com/en/docs/iam/ and Context7 as fallback.
- Separate confirmed findings from inference. If policy definitions were not shown, say so.
- Never request `SCW_ACCESS_KEY`, `SCW_SECRET_KEY`, or raw key values. Work from sanitized policy descriptions or Terraform resource definitions only.
- Flag API keys with no expiry, organization-level scope, or wildcard permissions as high-risk.
- Challenge vague scope, undocumented key usage, and missing rotation policies.

## References

Load these only when needed:

- Scaleway IAM concepts: https://www.scaleway.com/en/docs/iam/concepts/
- IAM API reference: https://www.scaleway.com/en/developers/api/iam/
- Terraform iam_policy: https://registry.terraform.io/providers/scaleway/scaleway/latest/docs/resources/iam_policy

## Response minimum

Return, at minimum:

- IAM posture verdict and evidence level,
- high-risk findings (no expiry, org-level scope, wildcard),
- recommended tightening actions,
- blockers or assumptions that prevent stronger conclusions.
