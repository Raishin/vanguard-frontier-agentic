# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/audit-manager/latest/userguide/assessments.html
- https://docs.aws.amazon.com/audit-manager/latest/userguide/review-evidence.html
- https://docs.aws.amazon.com/config/latest/developerguide/conformance-packs.html
- https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards-fsbp-controls.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, or operational state. Prefer AWS managed MCP read-only evidence through the user's configured read-only AWS profile, read-only AWS CLI evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- AWS Audit Manager availability-change guidance says Audit Manager is moving into maintenance mode and points customers toward AWS Config Conformance Packs for many compliance use cases.
- Audit Manager evidence folders, Config conformance packs, Security Hub controls, and AWS Artifact reports are separate evidence sources with different scope and freshness limits.

Sampled live evidence:
- Read-only regional availability sampling reported `isAvailableIn` for AWS Audit Manager and AWS Config in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `Config Service+DescribeConformancePacks` and `Config Service+GetComplianceDetailsByConfigRule` were reported `isAvailableIn` in those regions.

Review implications:
- Audit Manager output, Config compliance, Security Hub findings, and AWS Artifact reports are evidence inputs; none alone proves compliance.
- Because Audit Manager availability and lifecycle changed, prefer Config conformance packs and exported evidence where AWS docs direct migration, and label legacy Audit Manager usage explicitly.
