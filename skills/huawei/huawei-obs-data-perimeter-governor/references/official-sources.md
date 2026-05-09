# Official sources

Use this reference only when you need source grounding for Huawei Cloud OBS service behavior or the detailed source list.

## Huawei Cloud documentation

Use these as starting points, not as proof of the user's live Huawei Cloud state:

- https://support.huaweicloud.com/intl/en-us/obs/index.html
- https://support.huaweicloud.com/intl/en-us/vpcep/index.html
- https://support.huaweicloud.com/intl/en-us/obs/obs_03_0086.html

## Key OBS security controls reference

| Control | Risk if Absent | Severity | Remediation |
|---------|---------------|---------|-------------|
| Bucket ACL: private | All objects internet-accessible | CRITICAL | Set ACL to private immediately |
| Block Public Access (account) | Bucket-level ACL can override to public | HIGH | Enable at account level |
| Bucket policy (authoritative) | ACL/policy conflicts | MEDIUM | Disable legacy ACL; use bucket policy |
| VPCEP binding | OBS traffic routes over public internet | HIGH | Configure VPCEP for private access |
| WORM (Object Lock) | Data tampered or deleted before retention | HIGH | Enable with reviewed lock period |
| Cross-region replication scope | MLPS 2.0 data residency violation | CRITICAL (regulated) | Restrict replication to CN regions |
| Presigned URL validity | Temporary public object exposure | MEDIUM | Set minimum validity window |

## MLPS 2.0 data residency reference

| MLPS Level | Data Residency Requirement | Cross-Region Replication |
|-----------|--------------------------|------------------------|
| Level 2 | CN regions preferred | International allowed with approval |
| Level 3 | Mainland China CN regions mandatory | International regions PROHIBITED |
| Level 4 | Classified government networks | OBS not applicable |

## Grounding rule

Official documentation explains Huawei Cloud service behavior. It does not prove the user's current account, region, quota, resource configuration, IAM boundary, pricing, or operational state. Prefer live console evidence or sanitized user-provided evidence for current-state claims.
