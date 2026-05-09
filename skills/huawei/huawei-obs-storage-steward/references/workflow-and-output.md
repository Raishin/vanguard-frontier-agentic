# Workflow and Output — Huawei OBS Storage Steward

Use this reference when performing OBS lifecycle design, access control review, cross-region replication configuration, or data protection assessment.

## Design Domains

Check these areas before giving a verdict:

- **Lifecycle policy**: transition rules (Standard → Warm → Cold → Archive), expiration rules, abort multipart thresholds
- **Bucket policy and ACL**: principal scope, resource conditions, action allow/deny matrix
- **SSE-KMS**: CMK binding, automatic key rotation, envelope encryption coverage
- **Versioning**: enabled/suspended/disabled state, non-current version expiration
- **Cross-region replication**: source region, destination region, DSL/MLPS cross-border legal basis if CN-* involved
- **WORM (object lock)**: compliance mode vs governance mode, retention period, legal hold
- **Logging and audit**: OBS access logging to dedicated logging bucket, CTS API trail

## Safe Workflow

1. **Frame scope**
   - Workload type and data sensitivity (PII, MLPS Level 3, regulated):
   - Current storage class and access frequency:
   - Replication or cross-border requirements:
   - Required outcome:
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer live OBS console evidence if available.
   - Otherwise inspect IaC/config, sanitized user evidence, or official Huawei Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test design**
   - What happens if lifecycle policy transitions active production data to Archive?
   - Which buckets have public ACL exposure?
   - Is cross-region replication to non-CN region compliant with MLPS/DSL?
   - Are KMS keys used for SSE-KMS still valid and not pending deletion?
   - What data is unrecoverable if a bucket is deleted without versioning?

4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output Contract

Return this structure:

```markdown
# Huawei OBS Storage Steward: <scope>
## Bucket inventory and classification
## Lifecycle policy assessment
## Access control and bucket policy review
## SSE-KMS coverage
## Versioning and WORM posture
## Cross-region replication and data residency
## Open questions
```

Each section must include an evidence level label.
