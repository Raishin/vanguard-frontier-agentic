# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/kms/latest/developerguide/overview.html
- https://docs.aws.amazon.com/kms/latest/developerguide/rotating-keys.html
- https://docs.aws.amazon.com/secretsmanager/latest/userguide/intro.html
- https://docs.aws.amazon.com/secretsmanager/latest/userguide/mes-security.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- Secrets Manager supports managing secret lifecycle and automating credential rotation instead of hard-coded secrets.
- Secrets Manager security guidance says rotation requires appropriate IAM policies and KMS/key trust policies, with scope that can vary by region.

Sampled live evidence:
- Read-only regional availability sampling reported AWS KMS and AWS Secrets Manager as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `KMS+DescribeKey` and `Secrets Manager+DescribeSecret` were reported `isAvailableIn` in those regions.

Review implications:
- Do not claim secret safety without evidence for rotation status, KMS key policy/grants, resource policy, replica/region scope, access path, audit logs, recovery windows, and application cutover behavior.
- Key or secret existence does not prove least privilege, correct rotation, or safe deletion.
