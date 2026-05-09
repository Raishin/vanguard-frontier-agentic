# IAM Permissions — Alibaba Cloud Live OSS Bucket Policy Guard

## Minimum Read Permissions (Discovery and Audit)

| Policy | Purpose |
|--------|---------|
| `AliyunOSSReadOnlyAccess` | Audit bucket policies, ACLs, replication rules, and versioning configuration without mutation rights |

## Required Mutation Permissions (Live Operations)

| Policy | Purpose | Scope Guidance |
|--------|---------|---------------|
| `AliyunOSSFullAccess` | Modify bucket ACL, bucket policy, CORS, or replication configuration | Must confirm exact bucket name and region before assuming this policy; never granted as standing access |

## Narrowing Guidance

- Confirm the exact bucket name and region before any mutation. Bucket names are globally unique within Alibaba Cloud OSS.
- `AliyunOSSFullAccess` should be assumed via STS for specific approved operations only — never a standing policy attachment.
- Use RAM conditions (`acs:RequestedRegion`, resource ARN prefix) to restrict OSS mutations to specific buckets or regions where the RAM policy language permits.
- Prefer `AliyunOSSReadOnlyAccess` for all pre-flight audits, ACL inspection, and policy review operations.

## Anti-Patterns — Never Grant

- `AdministratorAccess` — account-wide full control; never appropriate for an OSS guard agent.
- `AliyunOSSFullAccess` as a standing attachment to any RAM user or operational role.
- Long-lived RAM user access keys with `AliyunOSSFullAccess` — if compromised, the attacker can make all buckets public.
- Granting `AliyunOSSFullAccess` to CI/CD pipeline service accounts without bucket-scoped resource conditions.
- Setting bucket ACL to `public-read-write` without explicit data classification approval — web crawlers index public buckets within seconds.

## Audit Trail

All OSS bucket policy and ACL changes are logged in **ActionTrail** under the `oss` service. Ensure ActionTrail is enabled for the target region and delivering to OSS or SLS. Query events with `EventName` containing `PutBucketAcl`, `PutBucketPolicy`, `PutBucketReplication`, or `DeleteBucket`. Monitor for unexpected ACL changes via Cloud Monitor OSS access log alerts.
