# IAM Permissions — Huawei Live OBS Bucket Policy Guard

## Minimum Read Roles (Discovery and Audit)

| Policy | Purpose |
|--------|---------|
| `OBS ReadOnlyAccess` | Read bucket ACL, bucket policy statements, cross-region replication configuration, and bucket metadata |

## Required Mutation Roles (Live Operations)

| Policy | Purpose | Scope Guidance |
|--------|---------|---------------|
| `OBS OperateAccess` | Modify bucket ACL and bucket policy statements | Bind scoped to the target bucket via enterprise project; never grant `OBS FullAccess` account-wide |

## Narrowing Guidance

- Bind mutation permissions to the IAM principal scoped to the **target enterprise project** containing the bucket, not account-wide.
- For read-only pre-flight audits, `OBS ReadOnlyAccess` is sufficient — do not use `OBS OperateAccess` for reads.
- Use IAM agency-based access for cross-account OBS operations rather than direct user credentials.
- Prefer IAM user groups bound to the target enterprise project for bucket operators over direct per-user policies.
- Never grant `OBS FullAccess` at account level — this grants full control over all OBS buckets in the account.

## Anti-Patterns — Never Grant

- `FullAccess` system policy at account level — grants complete control over all services in the account.
- `OBS FullAccess` without enterprise project scoping — blast radius covers every OBS bucket in the account.
- Storing access keys in plaintext — use temporary credentials via agency or IAM token exchange.
- Public-read/write ACL on buckets containing sensitive or regulated data — crawlers index within seconds.

## Audit Trail

All OBS mutations (ACL changes, policy updates, replication configuration changes) are logged in **Cloud Trace Service (CTS)**. Ensure CTS is enabled for the OBS service in the target region. Query: `CTS > Cloud Trace > OBS > setBucketAcl / setBucketPolicy / setBucketReplication`.
