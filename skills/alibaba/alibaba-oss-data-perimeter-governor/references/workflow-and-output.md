# Workflow and output contract

Use this reference only when performing a full OSS data perimeter audit, ACL conflict analysis, or compliance posture review.

## Audit domains

Check these areas before giving a recommendation:

- Bucket ACL: public-read, public-read-write, private — flag public access as CRITICAL
- Block Public Access (BPA): account-level and bucket-level BPA status
- Object ACL vs bucket ACL conflict: uniform access enforcement gaps
- Bucket policy: cross-account access, condition restrictions, IP-based controls
- VPC endpoint binding: private access enforcement, public internet exposure from ECS
- WORM (Object Lock): mode (Governance/Compliance), lock period, irreversibility risk
- Cross-Region Replication (CRR): destination regions, MLPS 2.0 and PIPL compliance
- Encryption: server-side encryption (SSE-OSS, SSE-KMS) configuration

## Safe workflow

1. **Frame the audit scope**
   - Buckets in scope and their account context (CN-* vs international):
   - Regulatory requirements (MLPS 2.0, PIPL, other):
   - Explicit out-of-scope items:
2. **Collect evidence**
   - Prefer live OSS console or API evidence if available.
   - Otherwise inspect IaC, sanitized user evidence, or official Alibaba Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the posture**
   - Are any buckets set to public-read or public-read-write?
   - Is BPA enabled at the account level?
   - Do any CRR rules replicate CN-* classified data to international regions?
   - Are there object ACLs that conflict with the bucket ACL?
   - What evidence is missing?
4. **Recommend the smallest safe remediation sequence**
   - Prioritize public ACL remediation above all other findings.
   - Require explicit review before enabling WORM (Object Lock).
   - If the safest action is to gather more evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# Alibaba Cloud OSS Data Perimeter Assessment: <scope>
## Executive summary
- Security verdict (pass / warn / fail):
- Evidence level:
- Critical findings:
## Public ACL exposure
- Buckets with public-read or public-read-write:
- CRITICAL flag:
- Recommended remediation:
## Block Public Access posture
- Account-level BPA enabled:
- Bucket-level BPA gaps:
## Object ACL vs bucket ACL conflicts
- Conflicts detected:
- Uniform access enforcement gaps:
## VPC endpoint binding
- Endpoint configured:
- Public internet exposure from ECS:
## WORM and data protection
- Object Lock enabled:
- Lock mode and period:
- Irreversibility risk:
## MLPS 2.0 data residency compliance
- CRR rules to international regions from CN-* classified buckets:
- PIPL transfer mechanism:
- Compliance verdict:
## Prioritized remediation actions
| Priority | Finding | Action | Evidence level |
|---|---|---|---|
## Open questions
1. <question> — owner: <owner>, impact: <impact if unresolved>
```
