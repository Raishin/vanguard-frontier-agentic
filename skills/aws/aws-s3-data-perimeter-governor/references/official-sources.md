# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-control-block-public-access.html
- https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-points.html
- https://docs.aws.amazon.com/AmazonS3/latest/userguide/example-bucket-policies.html
- https://docs.aws.amazon.com/IAM/latest/UserGuide/what-is-access-analyzer.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- S3 Block Public Access can be configured at account, bucket, access point, and organization levels; IAM Access Analyzer for S3 can review public buckets.
- S3 access points support shared dataset access patterns, VPC restrictions, IAM policies, and object-operation controls.

Sampled live evidence:
- Read-only regional availability sampling reported S3, Security Hub, and GuardDuty as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `S3+GetBucketPolicy`, `S3+GetPublicAccessBlock`, and `AccessAnalyzer+ListFindings` were reported `isAvailableIn` in those regions.

Review implications:
- Data perimeter review needs bucket/access point policies, BPA state, encryption, VPC endpoint policy, organization/SCP context, Access Analyzer findings, replication, logging, and exception approvals.
- Public-access absence alone does not prove tenant/data perimeter safety.
