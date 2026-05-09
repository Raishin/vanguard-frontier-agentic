# Workflow and output contract

Use this reference only when performing a full OSS/NAS storage review, incident triage, or access control hardening assessment.

## Storage steward areas to check

- OSS bucket ACL: current ACL setting, sensitive data classification, justification for any non-private ACL
- Bucket policy: policy document review, IP restrictions, RAM user conditions, resource scoping
- Lifecycle rules: transition tier logic, expiration rules, production data protection (are any expiration rules targeting critical data?)
- Cross-region replication: enabled buckets, source/destination regions, DSL Article 31 assessment status for CN-* sources
- NAS/CPFS: protocol (SMB/NFS), mount target permissions, uid/gid mapping, VPC access control
- Signed URL governance: URL expiry settings, application-level signed URL generation patterns

## Safe workflow

1. **Frame scope** — confirm target buckets/NAS instances, data classification, evidence available, and explicit non-goals
2. **Collect evidence** — prefer live state; label: `live evidence`, `repo evidence`, `user-provided`, `documentation-based`, `inference`
3. **Stress-test** — what data is exposed? what is deleted by lifecycle? what is the CN-* replication status?
4. **Recommend safest action** — narrow scope, staged rollout, rollback path

## Output contract

Return this structure:

```markdown
# Alibaba Cloud Storage: <scope>
## Scope and evidence level
## Findings
## Risks
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
