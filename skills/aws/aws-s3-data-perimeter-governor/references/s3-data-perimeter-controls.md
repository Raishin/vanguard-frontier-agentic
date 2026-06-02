# S3 Data Perimeter Controls Guide

Use this reference for Amazon S3 data perimeter reviews covering Block Public Access, Object Ownership, ACL removal, bucket/access point policies, VPC endpoint conditions, TLS-only access, encryption, replication, logging, and cross-account access.

## What people get wrong

The lazy story is:

> Block Public Access is on, so the bucket is safe.

Wrong. S3 exposure is a policy system. Public blocking helps, but data can still leak through broad principals, cross-account trust, access points, replication, CloudFront origins, VPC endpoint policies, logging gaps, and weak object ownership assumptions.

Common bad assumptions:

- `Principal: *` is safe if conditions look specific.
- ACLs no longer matter everywhere because Object Ownership exists.
- VPC endpoint conditions prove private-only access.
- SSE-S3/SSE-KMS proves authorization and data minimization.
- Access points simplify policy without changing risk.
- Server access logging or CloudTrail data events are enabled unless proven.

## S3-specific failure modes

- Bucket policy allows public, cross-account, or organization-wide access without prefix/resource boundaries.
- Access point or Multi-Region Access Point policy bypasses intended bucket guardrails.
- Block Public Access differs at account, bucket, and access-point layers.
- Object Ownership/ACL state allows unexpected object-owner access or breaks writers.
- VPC endpoint policy, `aws:SourceVpce`, `aws:PrincipalOrgID`, and TLS conditions are missing or misapplied.
- Replication, lifecycle expiration, inventory, logging, or Macie coverage excludes sensitive prefixes.

## Minimum safe workflow

1. Identify bucket, account, Region, data classification, access patterns, writers/readers, and prefixes.
2. Review account-level and bucket-level Block Public Access plus Object Ownership and ACL posture.
3. Inspect bucket policy, access point policies, VPC endpoint policies, KMS key policy, and cross-account principals together.
4. Verify data perimeter conditions: organization, VPC endpoint, TLS, encryption, source account, source ARN, and prefix scope.
5. Check logging/detection: CloudTrail data events, S3 server access logs, Storage Lens, Macie, Access Analyzer, and Security Hub findings.
6. Recommend smallest reversible policy/control changes; destructive delete/lifecycle changes require separate approval.
7. State what live evidence was sampled and what remains unknown.

## Verification targets

- S3 Block Public Access at account, bucket, and access point level
- Object Ownership, ACL state, bucket policy, access point policy, and Multi-Region Access Point policy
- IAM/SCP/VPC endpoint/KMS policies affecting access
- policy conditions: `aws:PrincipalOrgID`, `aws:SourceVpce`, `aws:SecureTransport`, `s3:prefix`, `aws:SourceArn`, `aws:SourceAccount`
- CloudTrail data events, server access logs, S3 Inventory, Storage Lens, Macie classification, Access Analyzer, and Security Hub findings
- replication/lifecycle/object lock/retention settings for sensitive prefixes

## When to push back

Push back if the user asks to:

- allow broad cross-account or public access without data-owner approval
- disable Block Public Access to “fix” application access
- rely on encryption as a substitute for authorization
- remove logs, Object Lock, retention, or replication without compliance review
- trust a single bucket policy snippet without account/SCP/KMS/endpoint context
- patch production bucket policies without rollback and access test plan
