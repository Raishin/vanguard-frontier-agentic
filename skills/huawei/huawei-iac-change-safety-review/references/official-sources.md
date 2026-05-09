# Official sources

Use this reference only when you need source grounding for Huawei Cloud IaC and change management service behavior or the detailed source list.

## Huawei Cloud documentation

Use these as starting points, not as proof of the user's live Huawei Cloud state:

- https://support.huaweicloud.com/intl/en-us/rfs/index.html
- https://registry.terraform.io/providers/huaweicloud/huaweicloud/latest/docs
- https://support.huaweicloud.com/intl/en-us/organizations/index.html
- https://support.huaweicloud.com/intl/en-us/obs/index.html
- https://support.huaweicloud.com/intl/en-us/dew/index.html

## IaC risk classification reference

| Change Type | Blast Radius | Notes |
|-------------|--------------|-------|
| Security group rule addition | Low | Additive; reversible |
| Security group rule deletion | Medium | May break existing traffic |
| OBS bucket deletion | High | Irreversible if versioning disabled |
| GaussDB instance deletion | High | Irreversible; requires backup confirmation |
| DEW/KMS key deletion | High | Irreversible after scheduled deletion window |
| Organizations SCP modification | Org-wide | Affects all member accounts in scope |
| RFS stack deletion without termination protection | High | Single API call removes entire stack |

## Grounding rule

Official documentation explains Huawei Cloud service behavior. It does not prove the user's current account, region, quota, resource configuration, Organizations SCP scope, state file backend encryption status, or change approval gate configuration. Prefer sanitized terraform plan output or RFS change set preview for current-state claims.
