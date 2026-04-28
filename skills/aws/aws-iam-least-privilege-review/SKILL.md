---
name: aws-iam-least-privilege-review
description: Use this skill whenever the user asks to review AWS IAM policies, trust policies, S3 bucket policies, role permissions, Access Analyzer findings, or least-privilege cloud access. Trigger even if the user says "just check this policy" or "is this role safe?".
metadata:
  author: github: Raishin
  version: 0.1.0
---

# AWS IAM Least Privilege Review

## Purpose

Review AWS identity, trust, and resource policies for unnecessary access, privilege escalation, wildcard risk, confused-deputy exposure, and missing validation evidence.

## Workflow

1. Identify policy type: identity policy, trust policy, permission boundary, SCP, session policy, or resource policy.
2. Separate facts from inference. Do not assume account IDs, regions, principals, or resource ARNs.
3. Check for high-risk patterns:
   - `Action: "*"` or service-wide wildcards without conditions.
   - `Resource: "*"` where resource scoping is supported.
   - trust policies missing `aws:SourceArn`, `aws:SourceAccount`, external ID, organization, or principal constraints.
   - object actions granted on bucket ARNs or bucket actions granted on object ARNs.
   - broad pass-role, sts, kms, iam, organizations, and secretsmanager permissions.
4. Prefer least-privilege remediation over preserving legacy breadth.
5. Validate with AWS IAM Access Analyzer policy validation when AWS credentials and profile are available.

## Output

Return:

- risk summary,
- exact risky statements,
- proposed minimum-permission replacement,
- validation command,
- assumptions and unknowns.

## Security notes

Never invent ARNs or account IDs. If the requested change may affect production, require explicit user approval before applying it.
