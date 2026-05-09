# Workflow and output contract

Use this reference only when performing a full OBS data perimeter governance review or MLPS 2.0 compliance assessment for Huawei Cloud object storage.

## Governance domains

Check these areas before giving a verdict:

- ACL exposure: bucket ACL visibility (private/public-read/public-read-write) for all buckets in scope
- Block Public Access: account-level setting status and any bucket-level overrides
- Bucket policy: presence, principal scope, action scope, condition constraints
- VPCEP binding: private endpoint configuration for buckets accessed from ECS/CCE
- WORM (Object Lock): lock mode (COMPLIANCE/GOVERNANCE), lock period, and application scope
- Cross-region replication: destination regions, data classification, and MLPS 2.0 compliance
- Presigned URL: generation patterns, validity periods, and audit coverage

## Safe workflow

1. **Frame scope**
   - OBS bucket(s) and region(s) in scope:
   - Data classification (MLPS level, if applicable):
   - Current-state evidence:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live Huawei Cloud console evidence if available.
   - Otherwise inspect IaC/config, sanitized user evidence, or official Huawei Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test posture**
   - Which buckets have public ACLs that expose objects to the internet?
   - Is Block Public Access enabled at the account level to prevent bucket-level overrides?
   - Do bucket policies have overly broad principals (*)  or missing condition constraints?
   - Which OBS-accessing ECS/CCE workloads lack VPCEP binding and route traffic publicly?
   - Are any WORM lock periods misapplied and irreversible for too long?
   - Do cross-region replication targets include international regions for MLPS 2.0 classified data?
   - What evidence is missing to confirm perimeter integrity?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# Huawei Cloud OBS Data Perimeter: <scope>
## Public ACL and policy exposure assessment
## Block Public Access account-level posture
## VPC endpoint (VPCEP) binding and private access configuration
## WORM and data protection posture
## Cross-region replication MLPS 2.0 compliance
## Bucket policy least-privilege assessment
## Prioritized remediation actions
```

Each section must include an evidence level label.
